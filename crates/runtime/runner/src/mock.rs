//! 内存引擎：不碰设备，用来测「引擎契约」与上层逻辑。
//!
//! 与 [`ReplayEngine`](crate::replay::ReplayEngine) 的分工：
//!
//! | 引擎 | 帧从哪来 | 用途 |
//! |---|---|---|
//! | `MockEngine` | 调用方推入（编程控制） | 单测领域状态机的分支覆盖 |
//! | `ReplayEngine` | golden corpus 录像 | 回归：与旧版决策对齐 |
//!
//! 两者都实现 [`Engine`](crate::Engine)，所以领域逻辑不需要知道自己跑在哪。

use maazure_core::Point;

use crate::{Engine, EngineError, Frame};

/// 可编程的假引擎。
#[derive(Debug, Default)]
pub struct MockEngine {
    frames: Vec<Frame>,
    cursor: usize,
    /// 被投递的点击
    pub clicks: Vec<Point>,
    /// 被投递的滑动
    pub swipes: Vec<(Point, Point, u64)>,
    /// 设为 true 后所有操作返回 [`EngineError::NotConnected`]
    pub disconnected: bool,
}

impl MockEngine {
    /// 用给定的帧序列新建。
    pub fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            ..Default::default()
        }
    }

    /// 推进一帧（到头停在末帧）。
    pub fn advance(&mut self) -> bool {
        if self.cursor + 1 >= self.frames.len() {
            return false;
        }
        self.cursor += 1;
        true
    }

    /// 重置到第一帧并清空操作记录。
    pub fn reset(&mut self) {
        self.cursor = 0;
        self.clicks.clear();
        self.swipes.clear();
    }

    /// 当前帧序号。
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

impl Engine for MockEngine {
    fn screenshot(&self) -> Result<Frame, EngineError> {
        if self.disconnected {
            return Err(EngineError::NotConnected);
        }
        self.frames
            .get(self.cursor)
            .copied()
            .ok_or(EngineError::NotConnected)
    }

    fn click(&mut self, p: Point) -> Result<(), EngineError> {
        if self.disconnected {
            return Err(EngineError::NotConnected);
        }
        self.clicks.push(p);
        Ok(())
    }

    fn swipe(&mut self, from: Point, to: Point, ms: u64) -> Result<(), EngineError> {
        if self.disconnected {
            return Err(EngineError::NotConnected);
        }
        self.swipes.push((from, to, ms));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(seq: u64) -> Frame {
        Frame {
            seq,
            width: 1280,
            height: 720,
        }
    }

    #[test]
    fn advances_and_stops_at_last_frame() {
        let mut e = MockEngine::new(vec![frame(0), frame(1)]);
        assert_eq!(e.screenshot().unwrap().seq, 0);
        assert!(e.advance());
        assert_eq!(e.screenshot().unwrap().seq, 1);
        assert!(!e.advance());
        assert_eq!(e.screenshot().unwrap().seq, 1);
    }

    #[test]
    fn records_operations() {
        let mut e = MockEngine::new(vec![frame(0)]);
        e.click(Point::new(3, 4)).unwrap();
        e.swipe(Point::new(0, 0), Point::new(9, 9), 300).unwrap();
        assert_eq!(e.clicks, vec![Point::new(3, 4)]);
        assert_eq!(e.swipes.len(), 1);

        e.reset();
        assert!(e.clicks.is_empty());
        assert_eq!(e.cursor(), 0);
    }

    #[test]
    fn disconnected_engine_errors() {
        let mut e = MockEngine::new(vec![frame(0)]);
        e.disconnected = true;
        assert_eq!(e.screenshot(), Err(EngineError::NotConnected));
        assert_eq!(e.click(Point::new(1, 1)), Err(EngineError::NotConnected));
    }

    #[test]
    fn empty_engine_is_an_error() {
        let e = MockEngine::default();
        assert_eq!(e.screenshot(), Err(EngineError::NotConnected));
    }
}
