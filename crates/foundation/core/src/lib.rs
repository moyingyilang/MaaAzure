//! 地基 · 核心类型：几何、任务结果、错误树。
//!
//! **位置**：`1 地基` 的根节点。兄弟节点：[`maazure_vision`](../vision)。
//! 本 crate 不依赖任何业务概念、不做 IO、不接触 MaaFramework——
//! 因此它能在没有 SDK、没有模拟器的机器上编译与测试。
//!
//! 设计依据：`docs/1-foundation/1.2-boundaries.md`。

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

    /// 宽高是否为正。
    pub const fn is_valid(&self) -> bool {
        self.x2 > self.x1 && self.y2 > self.y1
    }

    /// 点是否落在区域内（闭区间）。
    pub const fn contains(&self, p: Point) -> bool {
        p.x >= self.x1 && p.x <= self.x2 && p.y >= self.y1 && p.y <= self.y2
    }
}

/// 任务执行结果。对应旧版 `run()` 的 `True` / `False` / `'recoverable'` 三态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskOutcome {
    /// 成功
    Ok,
    /// 不可恢复失败
    Failed,
    /// 可自动恢复，不计入失败次数
    Recoverable,
}

/// 错误**树**。根是 [`AzureError`]，四个分支对应旧版异常层次。
///
/// ```text
/// AzureError
/// ├─ Ending     正常终止：CampaignEnd / OilExhausted / OilMaxed
/// ├─ Recoverable 可恢复：MapDetection / EnemyMoved
/// ├─ Restart     需重启游戏：GameStuck / GameTooManyClick
/// └─ Fatal       不可恢复：HumanTakeover / Controller
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureError {
    /// ─ 分支一：正常终止，不是故障
    CampaignEnd,
    /// 石油耗尽
    OilExhausted,
    /// 石油已满
    OilMaxed,

    /// ─ 分支二：可自动恢复
    MapDetection(String),
    /// 敌人移动导致路线失效
    EnemyMoved,

    /// ─ 分支三：需要重启游戏
    GameStuck {
        /// 无有效操作的持续时长（秒）
        idle_secs: u64,
    },
    /// 同一按钮点击过多
    GameTooManyClick {
        /// 按钮标识
        button: String,
    },

    /// ─ 分支四：不可恢复
    HumanTakeover(String),
    /// 控制器错误
    Controller(String),
}

/// 错误所属分支。调度器只需看分支，不需要匹配每个叶子。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorBranch {
    /// 正常终止
    Ending,
    /// 可恢复
    Recoverable,
    /// 重启游戏
    Restart,
    /// 不可恢复
    Fatal,
}

impl AzureError {
    /// 该错误属于哪条分支。
    pub const fn branch(&self) -> ErrorBranch {
        match self {
            Self::CampaignEnd | Self::OilExhausted | Self::OilMaxed => ErrorBranch::Ending,
            Self::MapDetection(_) | Self::EnemyMoved => ErrorBranch::Recoverable,
            Self::GameStuck { .. } | Self::GameTooManyClick { .. } => ErrorBranch::Restart,
            Self::HumanTakeover(_) | Self::Controller(_) => ErrorBranch::Fatal,
        }
    }

    /// 是否计入「连续失败次数」。
    ///
    /// 只有 `Restart` 与 `Fatal` 计入；正常终止与可恢复错误不计。
    pub const fn counts_as_failure(&self) -> bool {
        matches!(self.branch(), ErrorBranch::Restart | ErrorBranch::Fatal)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_geometry() {
        let r = Rect::new(100, 200, 300, 400);
        assert_eq!(r.center(), Point::new(200, 300));
        assert!(r.contains(Point::new(100, 200)));
        assert!(!r.contains(Point::new(99, 200)));
        assert!(!Rect::new(0, 0, 0, 10).is_valid());
    }

    #[test]
    fn error_tree_branches() {
        assert_eq!(AzureError::CampaignEnd.branch(), ErrorBranch::Ending);
        assert_eq!(
            AzureError::MapDetection("x".into()).branch(),
            ErrorBranch::Recoverable
        );
        assert_eq!(
            AzureError::GameStuck { idle_secs: 60 }.branch(),
            ErrorBranch::Restart
        );
        assert_eq!(
            AzureError::HumanTakeover("x".into()).branch(),
            ErrorBranch::Fatal
        );
    }

    #[test]
    fn only_restart_and_fatal_count_as_failure() {
        assert!(!AzureError::CampaignEnd.counts_as_failure());
        assert!(!AzureError::OilExhausted.counts_as_failure());
        assert!(!AzureError::EnemyMoved.counts_as_failure());
        assert!(AzureError::GameTooManyClick { button: "x".into() }.counts_as_failure());
        assert!(AzureError::Controller("x".into()).counts_as_failure());
    }
}
