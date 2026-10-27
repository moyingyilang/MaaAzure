//! Tasker 驱动、Pipeline 入口、事件桥、调度器。
//!
//! **这是唯一允许依赖 `maa-framework` 的 crate**（见 `docs/A4-rust-design.md` §2）。
//! 骨架阶段刻意还没有引入该依赖：没有 MaaFramework SDK 的机器（例如 Termux）
//! 仍应能 `cargo check` 整个 workspace。
//!
//! 下一步（P0）：
//! 1. 引入 `maa-framework = { version = "1", features = ["toolkit", "adb", "custom"] }`
//! 2. 在 `src/ffi.rs` 里封装 `Engine`，MaaFW 类型不出该文件
//! 3. 实现 `ReplayEngine`（读 golden corpus，见 `docs/A6-testing.md` §4）

#![deny(missing_docs)]

use maazure_core::{Point, TaskOutcome};

/// 执行引擎抽象：生产实现是 MaaFramework，测试实现是 golden corpus 回放。
///
/// 把引擎抽象成 trait，是为了让 `maazure-battle` 的决策逻辑可以在**没有模拟器、
/// 没有 MaaFramework** 的环境里被验证。
pub trait Engine {
    /// 取当前帧（回放引擎返回下一张录像帧）。
    fn screenshot(&self) -> Result<Frame, EngineError>;

    /// 投递一次点击（异步，不等待生效）。
    fn click(&mut self, p: Point) -> Result<(), EngineError>;

    /// 投递一次滑动。
    fn swipe(&mut self, from: Point, to: Point, ms: u64) -> Result<(), EngineError>;
}

/// 一帧图像 + 序号。图像格式在引入 `image` crate 后具体化。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    /// 帧序号，用于状态机去重
    pub seq: u64,
    /// 图像宽度（1280 基准）
    pub width: u32,
    /// 图像高度（720 基准）
    pub height: u32,
}

/// 引擎层错误。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    /// 设备未连接
    NotConnected,
    /// 目标平台未实现
    Unsupported(String),
}

impl core::fmt::Display for EngineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotConnected => write!(f, "device not connected"),
            Self::Unsupported(m) => write!(f, "unsupported: {m}"),
        }
    }
}

impl core::error::Error for EngineError {}

/// 任务规格：调度器的最小单位。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskSpec {
    /// 任务名（与 Pipeline 入口节点同名，如 `Commission`）
    pub name: &'static str,
    /// 调度优先级，数字越小越优先（沿用旧版顺序）
    pub priority: u8,
}

/// 一次任务执行的结果汇总。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskReport {
    /// 任务名
    pub name: String,
    /// 结果
    pub outcome: TaskOutcome,
    /// 消耗帧数
    pub frames: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_error_display() {
        assert_eq!(EngineError::NotConnected.to_string(), "device not connected");
    }

    #[test]
    fn frame_baseline_is_720p() {
        let f = Frame { seq: 0, width: 1280, height: 720 };
        assert_eq!(f.width * 9, f.height * 16);
    }
}
