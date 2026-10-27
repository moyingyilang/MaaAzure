//! 领域 · 日常任务：跨帧比较类决策。
//!
//! **位置**：`2 领域` 下与 `battle` 平级的子树。
//! 日常任务 90% 是 Pipeline 就够的点击序列（`docs/3-authoring/3.1-pipeline-spec.md`），
//! 唯独「在若干候选里挑一个」必须写代码——本 crate 只装这类最小决策，不装流程。
//!
//! 约束：单个决策函数不超过 30 行、不持有状态、不做 IO。超过就说明该拆。

#![deny(missing_docs)]

/// 一个可领取的委托。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommissionSlot {
    /// 该卡片的点击区域中心（屏幕坐标）
    pub center: (i32, i32),
    /// 剩余秒数
    pub remaining_secs: u64,
    /// 是否紧急（紧急委托优先）
    pub urgent: bool,
}

/// 选出最该领的那一条：紧急优先，其次剩余时间最短。
///
/// 移植自旧版 `module/commission/commission.py` 的选择规则。
/// 空列表返回 `None`。
///
/// # Examples
///
/// ```
/// use maazure_daily::{CommissionSlot, pick_commission};
/// let slots = [
///     CommissionSlot { center: (100, 300), remaining_secs: 3600, urgent: false },
///     CommissionSlot { center: (100, 400), remaining_secs: 7200, urgent: true },
/// ];
/// assert_eq!(pick_commission(&slots).unwrap().center, (100, 400));
/// ```
pub fn pick_commission(slots: &[CommissionSlot]) -> Option<&CommissionSlot> {
    slots.iter().min_by_key(|s| {
        // 紧急的排前面（false < true，所以取反）
        (!s.urgent, s.remaining_secs)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(y: i32, secs: u64, urgent: bool) -> CommissionSlot {
        CommissionSlot {
            center: (100, y),
            remaining_secs: secs,
            urgent,
        }
    }

    #[test]
    fn urgent_wins_over_shorter() {
        let slots = [slot(300, 10, false), slot(400, 9999, true)];
        assert_eq!(pick_commission(&slots).unwrap().center, (100, 400));
    }

    #[test]
    fn shorter_wins_when_same_urgency() {
        let slots = [slot(300, 7200, false), slot(400, 3600, false)];
        assert_eq!(pick_commission(&slots).unwrap().center, (100, 400));
    }

    #[test]
    fn empty_is_none() {
        assert!(pick_commission(&[]).is_none());
    }
}
