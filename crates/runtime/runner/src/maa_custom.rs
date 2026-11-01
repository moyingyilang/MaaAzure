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

use crate::commission_scan::{
    card_area, choose, detect_lines, row_means, PixelOrder, ScanAccumulator, ScannedCommission,
    CARD_PARSE_SPAN, CARD_X0, CARD_X1, LINE_X0, LINE_X1,
};
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
        // 优先用 CommissionScan 累积出来的结果（这是 B3 的接线点）：
        // 有数据就按上游规则选一张卡，直接把它的区域返回给 Click 动作。
        let scanned = commission_scan_entries();
        if !scanned.is_empty() {
            let picked = choose(&scanned);
            if picked.is_empty() {
                return None; // 槽位已满 / 没有可派的，这一轮不点
            }
            let target = &scanned[picked[0]];
            let detail = format!(
                r#"{{"picked":{},"y":{},"text":{},"candidates":{}}}"#,
                picked[0],
                target.y,
                serde_json::to_string(&target.text).unwrap_or_else(|_| "\"\"".to_string()),
                scanned.len()
            );
            return Some((
                Rect {
                    x: CARD_X0,
                    y: target.y - CARD_PARSE_SPAN,
                    width: CARD_X1 - CARD_X0,
                    height: CARD_PARSE_SPAN,
                },
                detail,
            ));
        }

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
        // 槽位来源：参数里显式给了 slots 就用（调试/合成帧），否则**按上游行检测算**。
        let slots_cfg: Vec<[i32; 4]> = if custom_recognition_param.contains("slots") {
            parse_slots(custom_recognition_param).unwrap_or_default()
        } else {
            detect_slots(image).unwrap_or_default()
        };
        if slots_cfg.is_empty() {
            return None;
        }

        let mut frame = Vec::with_capacity(slots_cfg.len());
        for slot in &slots_cfg {
            let text = match ocr_text(context, image, slot) {
                Some(t) if !t.trim().is_empty() => t,
                _ => continue,
            };
            let t = text.trim().to_string();
            frame.push(ScannedCommission {
                y: slot[1] + slot[3] / 2,
                remaining_secs: parse_duration_secs(&t).unwrap_or(u64::MAX),
                running: t.contains("进行"),
                is_daily_event: false,
                text: t,
            });
        }

        let mut guard = SCAN.lock().ok()?;
        let acc = guard.get_or_insert_with(ScanAccumulator::default);
        let added = acc.observe(&frame);
        let total = acc.len();
        drop(guard);

        if added == 0 {
            return None;
        }
        let first = slots_cfg[0];
        let hit = Rect {
            x: first[0],
            y: first[1],
            width: first[2],
            height: first[3],
        };
        Some((hit, format!(r#"{{"new":{added},"total":{total}}}"#)))
    }
}

/// 按上游 `lines_detect` 从截图里算出每张委托卡的解析区域。
///
/// 上游看的是 x∈[597,619) 这条白色分割线竖带，逐行灰度均值做峰值检测，
/// 再用 `card_area`（`[188, y-119, 1011, 119]`）得到每张卡的 OCR 区域。
fn detect_slots(image: &MaaImageBuffer) -> Option<Vec<[i32; 4]>> {
    let data = image.raw_data()?;
    let w = image.width().max(0) as usize;
    let h = image.height().max(0) as usize;
    let bpp = image.channels().max(0) as usize;
    if w == 0 || h == 0 || bpp < 3 {
        return None;
    }
    let order = match std::env::var("MAAGUI_PIXEL_ORDER").as_deref() {
        Ok("bgr") => PixelOrder::Bgr,
        Ok("rgba") => PixelOrder::Rgba,
        _ => PixelOrder::Rgb,
    };
    let stride = w * bpp;
    let x1 = LINE_X1.min(w);
    let means = row_means(data, h, stride, bpp, order, LINE_X0.min(w), x1);
    let lines = detect_lines(&means);
    Some(lines.into_iter().map(card_area).collect())
}

fn t_trim(s: &str) -> String {
    s.trim().to_string()
}
