//! 注册给 MaaFramework 的业务组件：把领域函数接到 Pipeline 的 `Custom` 节点上。
//!
//! 这里的代码**只做桥接**：解析参数、调 `maazure-daily` / `maazure-vision` 的纯函数、
//! 把结果翻译成 MaaFW 的返回类型。业务判断不在这一层写（`docs/1-foundation/1.2-boundaries.md`）。

use maa_framework::buffer::MaaImageBuffer;
use maa_framework::common::Rect;
use maa_framework::context::Context;
use maa_framework::custom::CustomRecognition;
use maa_framework::sys;

use maazure_daily::{CommissionSlot, pick_commission};
use maazure_vision::parse_duration_secs;

use crate::commission_scan::{ScanAccumulator, ScannedCommission};
use crate::ocr_detail::{extract_text, parse_slots};

/// 委托卡片默认的四个槽位（1280×720 下的列表区）。
///
/// 真实坐标由 Pipeline 节点的 `custom_recognition_param` 覆盖；
/// 这里的默认值只是让「忘了传参」时跑得起来、日志里能看出问题。
const DEFAULT_SLOTS: [[i32; 4]; 4] = [
    [300, 180, 980, 260],
    [300, 270, 980, 350],
    [300, 360, 980, 440],
    [300, 450, 980, 530],
];

/// `CommissionPick`：读四条委托的剩余时间，选最该领的那条，返回它的点击框。
///
/// Pipeline 侧：
///
/// ```jsonc
/// "commission_pick": {
///   "recognition": "Custom",
///   "custom_recognition": "CommissionPick",
///   "custom_recognition_param": "{\"slots\": [[300,180,980,260], ...]}",
///   "action": "DoNothing"
/// }
/// ```
///
/// 返回值：命中槽位的矩形 + 决策详情 JSON（写进 MaaFW 日志，便于复盘）。
pub struct CommissionPick;

impl CustomRecognition for CommissionPick {
    fn analyze(
        &self,
        context: &Context,
        _task_id: sys::MaaTaskId,
        _node_name: &str,
        _custom_recognition_name: &str,
        custom_recognition_param: &str,
        image: &MaaImageBuffer,
        _roi: &Rect,
    ) -> Option<(Rect, String)> {
        let slots_cfg =
            parse_slots(custom_recognition_param).unwrap_or_else(|| DEFAULT_SLOTS.to_vec());

        let mut slots: Vec<CommissionSlot> = Vec::with_capacity(slots_cfg.len());
        let mut raw_texts: Vec<Option<String>> = Vec::with_capacity(slots_cfg.len());

        for roi in &slots_cfg {
            let text = ocr_text(context, image, roi);
            let remaining = text
                .as_deref()
                .and_then(parse_duration_secs)
                .unwrap_or(u64::MAX);
            slots.push(CommissionSlot {
                center: ((roi[0] + roi[2]) / 2, (roi[1] + roi[3]) / 2),
                remaining_secs: remaining,
                urgent: false,
            });
            raw_texts.push(text);
        }

        // 全是 u64::MAX 说明一条都没读出来：不算命中，让 Pipeline 走别的分支
        if slots.iter().all(|s| s.remaining_secs == u64::MAX) {
            return None;
        }

        let picked = pick_commission(&slots)?;
        let idx = slots.iter().position(|s| s == picked)?;
        let roi = slots_cfg[idx];

        let detail = serde_json::json!({
            "picked_index": idx,
            "remaining_secs": picked.remaining_secs,
            "raw_texts": raw_texts,
            "candidates": slots.iter().map(|s| s.remaining_secs).collect::<Vec<_>>(),
        });

        Some((
            Rect {
                x: roi[0],
                y: roi[1],
                width: roi[2] - roi[0],
                height: roi[3] - roi[1],
            },
            detail.to_string(),
        ))
    }
}

/// 在一个 ROI 上跑 OCR，取第一段文本。
///
/// 用 `run_recognition_direct` 而不是自己截图：省一次截图（~350ms 的大头），
/// 也避免在回调里阻塞。
fn ocr_text(context: &Context, image: &MaaImageBuffer, roi: &[i32; 4]) -> Option<String> {
    let param = format!(r#"{{"roi":[{},{},{},{}]}}"#, roi[0], roi[1], roi[2], roi[3]);
    let detail = context
        .run_recognition_direct("OCR", &param, image)
        .ok()
        .flatten()?;
    if !detail.hit {
        return None;
    }
    extract_text(&detail.detail)
}


/// `CommissionScan`：委托列表的**跨帧累积**识别。
///
/// MaaFW 节点无状态，做不了「扫 15 屏再合并」，所以累积放在这里，
/// 用全局状态跨帧保存（同一个 agent 进程内有效）。
///
/// Pipeline 侧：
///
/// ```jsonc
/// "commission_scan_swipe": {
///   "recognition": "Custom",
///   "custom_recognition": "CommissionScan",
///   "custom_recognition_param": "{"slots": [[300,180,980,260], ...]}",
///   "action": { "type": "Swipe", ... },
///   "next": ["commission_scan_swipe", "commission_pick"]
/// }
/// ```
///
/// 语义：**命中 = 这一帧出现了还没见过的委托**。
/// 滑到底时不再命中，`next` 自然落到 `commission_pick`。
pub struct CommissionScan;

/// 全局累积器。切任务时通过 `reset` 清空（见 `commission_scan_reset`）。
static SCAN: std::sync::Mutex<Option<ScanAccumulator>> = std::sync::Mutex::new(None);

/// 清空累积状态。任务开始时调用一次。
pub fn commission_scan_reset() {
    if let Ok(mut g) = SCAN.lock() {
        *g = Some(ScanAccumulator::default());
    }
}

/// 读出当前累积结果（给 CommissionPick 用）。
pub fn commission_scan_entries() -> Vec<ScannedCommission> {
    SCAN.lock()
        .ok()
        .and_then(|g| g.as_ref().map(|a| a.entries().to_vec()))
        .unwrap_or_default()
}

impl CustomRecognition for CommissionScan {
    fn analyze(
        &self,
        context: &Context,
        _task_id: sys::MaaTaskId,
        _node_name: &str,
        _custom_recognition_name: &str,
        custom_recognition_param: &str,
        image: &MaaImageBuffer,
        _roi: &Rect,
    ) -> Option<(Rect, String)> {
        let slots_cfg =
            parse_slots(custom_recognition_param).unwrap_or_else(|| DEFAULT_SLOTS.to_vec());

        let mut frame = Vec::with_capacity(slots_cfg.len());
        for roi in &slots_cfg {
            let text = match ocr_text(context, image, roi) {
                Some(t) if !t.trim().is_empty() => t,
                _ => continue,
            };
            frame.push(ScannedCommission {
                y: roi[1] + roi[3] / 2,
                text: t_trim(&text),
            });
        }

        let mut guard = SCAN.lock().ok()?;
        let acc = guard.get_or_insert_with(ScanAccumulator::default);
        let added = acc.observe(&frame);
        let total = acc.len();
        drop(guard);

        // 有新增才算命中：这样节点会自动滑到「没有新卡」为止。
        if added == 0 {
            return None;
        }
        let first = slots_cfg.first().copied().unwrap_or([0, 0, 1, 1]);
        // 注意：这里是 [x, y, w, h]（MaaFramework 的 roi 语义）
        let hit = Rect {
            x: first[0],
            y: first[1],
            width: first[2],
            height: first[3],
        };
        Some((hit, format!(r#"{{"new":{added},"total":{total}}}"#)))
    }
}

fn t_trim(s: &str) -> String {
    s.trim().to_string()
}
