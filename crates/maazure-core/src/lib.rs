//! MaaAzure 核心类型：错误、任务结果、几何、时间。
//!
//! 本 crate **不允许**依赖 `maa-framework`，也不允许做 IO。
//! 这样它能在任意机器（包括没有 MaaFramework SDK 的 Termux）上编译与测试。
//!
//! 设计依据：`docs/A4-rust-design.md`。

#![deny(missing_docs)]

use core::fmt;

/// 屏幕坐标点（1280×720 基准）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    /// 横坐标
    pub x: i32,
    /// 纵坐标
    pub y: i32,
}

impl Point {
    /// 新建一个点。
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// 矩形区域 `(x1, y1, x2, y2)`，与旧版 `area` / `button` 语义一致。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    /// 左上 x
    pub x1: i32,
    /// 左上 y
    pub y1: i32,
    /// 右下 x
    pub x2: i32,
    /// 右下 y
    pub y2: i32,
}

impl Rect {
    /// 新建一个区域。
    pub const fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// 中心点。
    pub const fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    /// 面积是否为正。
    pub const fn is_valid(&self) -> bool {
        self.x2 > self.x1 && self.y2 > self.y1
    }

    /// 点是否落在区域内（闭区间）。
    pub const fn contains(&self, p: Point) -> bool {
        p.x >= self.x1 && p.x <= self.x2 && p.y >= self.y1 && p.y <= self.y2
    }
}

/// 任务执行结果。
///
/// 对应旧版 `run()` 的三种返回值：`True` / `False` / `'recoverable'`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskOutcome {
    /// 成功
    Ok,
    /// 不可恢复失败
    Failed,
    /// 可自动恢复，不计入失败次数
    Recoverable,
}

/// 统一错误类型。
///
/// 对照旧版 `module/exception.py` 的异常层次，语义逐条保留，
/// 见 `docs/A4-rust-design.md` §5 的对照表。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureError {
    /// 战役正常结束（不是错误，只是终止信号）
    CampaignEnd,
    /// 石油耗尽
    OilExhausted,
    /// 石油已满
    OilMaxed,

    /// 地图检测失败
    MapDetection(String),
    /// 敌人移动导致路线失效
    EnemyMoved,

    /// 游戏卡死（超过阈值无有效操作）
    GameStuck {
        /// 无有效操作的持续时长（秒）
        idle_secs: u64,
    },
    /// 同一按钮点击过多
    GameTooManyClick {
        /// 按钮标识
        button: String,
    },

    /// 需要人工接管
    HumanTakeover(String),

    /// 控制器错误
    Controller(String),
}

impl fmt::Display for AzureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CampaignEnd => write!(f, "campaign finished"),
            Self::OilExhausted => write!(f, "oil exhausted"),
            Self::OilMaxed => write!(f, "oil maxed"),
            Self::MapDetection(m) => write!(f, "map detection failed: {m}"),
            Self::EnemyMoved => write!(f, "enemy moved"),
            Self::GameStuck { idle_secs } => write!(f, "game stuck for {idle_secs}s"),
            Self::GameTooManyClick { button } => write!(f, "too many clicks on {button}"),
            Self::HumanTakeover(m) => write!(f, "human takeover required: {m}"),
            Self::Controller(m) => write!(f, "controller error: {m}"),
        }
    }
}

impl core::error::Error for AzureError {}

/// 该错误是否应当计入「连续失败次数」。
///
/// 旧版只有显式的 `'recoverable'` 不计；正常结束（战役打完、油尽）也不算失败。
pub const fn counts_as_failure(e: &AzureError) -> bool {
    !matches!(
        e,
        AzureError::CampaignEnd
            | AzureError::OilExhausted
            | AzureError::OilMaxed
            | AzureError::EnemyMoved
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_center_and_contains() {
        let r = Rect::new(100, 200, 300, 400);
        assert_eq!(r.center(), Point::new(200, 300));
        assert!(r.contains(Point::new(100, 200)));
        assert!(!r.contains(Point::new(99, 200)));
        assert!(r.is_valid());
        assert!(!Rect::new(0, 0, 0, 10).is_valid());
    }

    #[test]
    fn normal_endings_do_not_count_as_failure() {
        assert!(!counts_as_failure(&AzureError::CampaignEnd));
        assert!(!counts_as_failure(&AzureError::OilExhausted));
        assert!(counts_as_failure(&AzureError::GameStuck { idle_secs: 60 }));
        assert!(counts_as_failure(&AzureError::HumanTakeover(
            "x".to_string()
        )));
    }
}
