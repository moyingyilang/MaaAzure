//! 运行时 · 执行引擎边界。
//!
//! **位置**：`3 运行时`。上层是 `4 应用`（daemon），下层是 `2 领域`。
//! 这是**唯一允许依赖 `maa-framework` 的 crate**（`docs/2-architecture/2.4-ffi.md`）。
//! 骨架阶段尚未引入该依赖：没有 SDK 的机器也要能 `cargo check` 整个 workspace。
//!
//! 下一步（P0）：
//! 1. 加 `maa-framework = { version = "1", features = ["toolkit","adb","custom"] }`
//! 2. `src/ffi.rs` 封装 `Engine`，MaaFW 类型不出该文件
//! 3. 实现 `ReplayEngine`（读 golden corpus）

#![deny(missing_docs)]

use maazure_core::{Point, TaskOutcome};

pub mod replay;

/// 一帧图像 + 序号。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    /// 帧序号，用于状态机去重
    pub seq: u64,
    /// 宽度（1280 基准）
    pub width: u32,
    /// 高度（720 基准）
    pub height: u32,
}

/// 引擎层错误。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    /// 设备未连接
    NotConnected,
    /// 平台或能力未实现
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

/// 执行引擎抽象：生产实现是 MaaFramework，测试实现是录像回放。
///
/// 抽成 trait 的目的：让 `2 领域` 的决策能在**没有模拟器、没有 MaaFramework** 的环境里被验证。
pub trait Engine {
    /// 取当前帧。
    fn screenshot(&self) -> Result<Frame, EngineError>;
    /// 投递点击（异步，不等待生效）。
    fn click(&mut self, p: Point) -> Result<(), EngineError>;
    /// 投递滑动。
    fn swipe(&mut self, from: Point, to: Point, ms: u64) -> Result<(), EngineError>;
}

/// 任务规格：调度器的最小单位。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskSpec {
    /// 任务名，与 Pipeline 入口节点同名（如 `Commission`）
    pub name: &'static str,
    /// 优先级，数字越小越优先
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
    fn error_display() {
        assert_eq!(EngineError::NotConnected.to_string(), "device not connected");
    }

    #[test]
    fn baseline_frame_is_16_9() {
        let f = Frame { seq: 0, width: 1280, height: 720 };
        assert_eq!(f.width * 9, f.height * 16);
    }
}
