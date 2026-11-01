//! 委托列表的跨帧累积器。
//!
//! 上游 `_commission_scan_list` 的做法是「OCR 一屏 → 去重合并 → 滑动 → 重复」，
//! 本质是**跨帧累积一张列表**。MaaFW 的节点是无状态的，做不了这件事，
//! 所以累积逻辑放在 Rust 侧（见 docs/4-migration/4.6）。
//!
//! 这个模块只做纯数据：给一帧 OCR 出来的若干条文本，判断有没有新条目、
//! 累积总数是多少。不碰图像、不碰框架，因此可以单独单测。

use std::collections::HashMap;

/// 一张委托卡在列表里的位置与内容。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedCommission {
    /// 卡片在屏幕上的纵向中心（用于去重时容差比较）
    pub y: i32,
    /// OCR 出来的原始文本（名字/剩余时间的组合）
    pub text: String,
}

/// 跨帧累积器。
///
/// 去重规则：文本相同且纵向位置相差不超过 `tolerance`，视为同一张卡。
/// 只按文本去重会误合并「同名但不同位置」的卡；只按位置去重会误合并
/// 滑动后落到同一坐标的另一张卡。
#[derive(Debug)]
pub struct ScanAccumulator {
    entries: Vec<ScannedCommission>,
    tolerance: i32,
    /// 已经见过多少条（含去重掉的）
    seen_frames: usize,
}

impl Default for ScanAccumulator {
    fn default() -> Self {
        Self::new(12)
    }
}

impl ScanAccumulator {
    /// 用给定的纵坐标容差创建累积器。
    pub fn new(tolerance: i32) -> Self {
        Self {
            entries: Vec::new(),
            tolerance,
            seen_frames: 0,
        }
    }

    /// 喂一帧的识别结果，返回这一帧里**新出现**的条目数。
    ///
    /// 返回 0 表示这一屏没有新卡——对 Pipeline 来说就是「滑到底了」。
    pub fn observe(&mut self, frame: &[ScannedCommission]) -> usize {
        self.seen_frames += 1;
        let mut added = 0;
        for cand in frame {
            let dup = self.entries.iter().any(|e| {
                e.text == cand.text && (e.y - cand.y).abs() <= self.tolerance
            });
            if !dup {
                self.entries.push(cand.clone());
                added += 1;
            }
        }
        added
    }

    /// 已累积的条目数。
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 是否还没有任何条目。
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 已累积的全部条目。
    pub fn entries(&self) -> &[ScannedCommission] {
        &self.entries
    }

    /// 一共喂过多少帧。
    pub fn seen_frames(&self) -> usize {
        self.seen_frames
    }

    /// 按文本统计每个名字出现几次（同名不同位置的卡会累加）。
    pub fn name_counts(&self) -> HashMap<&str, usize> {
        let mut m = HashMap::new();
        for e in &self.entries {
            *m.entry(e.text.as_str()).or_insert(0) += 1;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(y: i32, text: &str) -> ScannedCommission {
        ScannedCommission {
            y,
            text: text.to_string(),
        }
    }

    #[test]
    fn 首帧全部是新的() {
        let mut acc = ScanAccumulator::default();
        assert_eq!(acc.observe(&[c(100, "A"), c(200, "B")]), 2);
        assert_eq!(acc.len(), 2);
    }

    #[test]
    fn 同一帧重复喂不会重复计数() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(100, "A")]);
        assert_eq!(acc.observe(&[c(100, "A")]), 0);
        assert_eq!(acc.len(), 1);
    }

    #[test]
    fn 滑动后位置变化视为新卡() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(300, "A")]);
        // 滑上去之后同一张卡跑到 y=100，位移超过容差 → 当作新条目
        assert_eq!(acc.observe(&[c(100, "A")]), 1);
        assert_eq!(acc.len(), 2);
    }

    #[test]
    fn 容差内的小位移仍然去重() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(300, "A")]);
        assert_eq!(acc.observe(&[c(305, "A")]), 0);
    }

    #[test]
    fn 滑到底时返回零() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(100, "A"), c(200, "B")]);
        // 第二屏只有已见过的卡
        assert_eq!(acc.observe(&[c(100, "A"), c(200, "B")]), 0);
        assert_eq!(acc.seen_frames(), 2);
    }

    #[test]
    fn 同名不同位置分别计数() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(100, "A"), c(400, "A")]);
        assert_eq!(acc.len(), 2);
        assert_eq!(acc.name_counts().get("A"), Some(&2));
    }
}
