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
