//! 战役 / 战斗 / 大世界的状态机与地图模型。
//!
//! 本 crate 是**纯逻辑**：输入是「一帧的观测」，输出是「一个动作命令」。
//! 不依赖 MaaFramework、不做 IO，因此可以对着 golden corpus 录像做单元测试
//! （见 `docs/A6-testing.md`）。
//!
//! 设计依据：`docs/A4-rust-design.md` §4。

#![deny(missing_docs)]

use maazure_core::{Point, Rect};

/// 战役地图格子符号。
///
/// 与旧版 `campaign/*.py` 里 `MAP.map_data` 的文本符号一一对应，
/// 迁移时由 `xtask convert-campaign` 从 Python 字面量提取（`docs/A3-resource-migration.md` §3）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    /// `--` 海洋（可通行）
    Sea,
    /// `++` 陆地（不可通行）
    Land,
    /// `SP` 舰队出生点
    Spawn,
    /// `ME` 敌人可能出现
    EnemyMaybe,
    /// `MB` Boss 可能出现
    BossMaybe,
    /// `MM` 神秘敌人
    Mystery,
    /// `MA` 弹药拾取
    Ammo,
    /// `MS` 精英 / 塞壬
    Siren,
    /// `__` 空位（不可通行，无内容）
    Empty,
    /// `Me` 已确认的普通敌人
    Enemy,
}

impl Symbol {
    /// 解析单个符号；非法符号返回 `None`。
    pub fn parse(token: &str) -> Option<Self> {
        Some(match token {
            "--" => Self::Sea,
            "++" => Self::Land,
            "SP" => Self::Spawn,
            "ME" => Self::EnemyMaybe,
            "MB" => Self::BossMaybe,
            "MM" => Self::Mystery,
            "MA" => Self::Ammo,
            "MS" => Self::Siren,
            "__" => Self::Empty,
            "Me" => Self::Enemy,
            _ => return None,
        })
    }

    /// 舰队能否移动到该格。
    pub const fn walkable(&self) -> bool {
        !matches!(self, Self::Land | Self::Empty)
    }
}

/// 地图形状，如 `K8` = 11 列 × 8 行。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    /// 列数
    pub cols: usize,
    /// 行数
    pub rows: usize,
}

impl Shape {
    /// 解析旧版 `shape` 字段：字母 A–Z 表列数（A=1, K=11），数字表行数。
    pub fn parse(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let col = chars.next()?.to_ascii_uppercase();
        if !col.is_ascii_uppercase() {
            return None;
        }
        let cols = (col as u32 - 'A' as u32 + 1) as usize;
        let rows: usize = chars.as_str().parse().ok()?;
        Some(Self { cols, rows })
    }
}

/// 解析 `map_data` 的三引号文本为二维符号表。
///
/// 输入是旧版 `MAP.map_data` 的原文（含缩进与换行），行内以空格分隔。
/// 返回 `Err` 时携带具体原因，转换器据此生成待人工处理的清单。
pub fn parse_map_rows(raw: &str, shape: Shape) -> Result<Vec<Vec<Symbol>>, String> {
    let mut grid = Vec::with_capacity(shape.rows);

    for (i, line) in raw.lines().map(str::trim).filter(|l| !l.is_empty()).enumerate() {
        let row: Vec<Symbol> = line
            .split_whitespace()
            .map(|t| Symbol::parse(t).ok_or_else(|| format!("第 {i} 行有非法符号 `{t}`")))
            .collect::<Result<_, _>>()?;

        if row.len() != shape.cols {
            return Err(format!(
                "第 {i} 行有 {} 列，期望 {} 列（shape={}x{}）",
                row.len(),
                shape.cols,
                shape.cols,
                shape.rows
            ));
        }
        grid.push(row);
    }

    if grid.len() != shape.rows {
        return Err(format!(
            "实际 {} 行，期望 {} 行",
            grid.len(),
            shape.rows
        ));
    }
    Ok(grid)
}

/// 一个地图决策命令。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// 点击某点
    Click(Point),
    /// 滑动
    Swipe {
        /// 起点
        from: Point,
        /// 终点
        to: Point,
        /// 耗时（毫秒）
        ms: u64,
    },
    /// 等待若干帧
    WaitFrames(u8),
}

/// 屏幕上的一个可点击区域 + 其格子坐标。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridCell {
    /// 格子坐标 `(col, row)`
    pub location: (usize, usize),
    /// 该格在屏幕上的可点击区域
    pub screen: Rect,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_parsing() {
        assert_eq!(Shape::parse("K8"), Some(Shape { cols: 11, rows: 8 }));
        assert_eq!(Shape::parse("A1"), Some(Shape { cols: 1, rows: 1 }));
        assert_eq!(Shape::parse("88"), None);
        assert_eq!(Shape::parse(""), None);
    }

    #[test]
    fn symbol_walkable() {
        assert!(Symbol::parse("--").unwrap().walkable());
        assert!(Symbol::parse("ME").unwrap().walkable());
        assert!(!Symbol::parse("++").unwrap().walkable());
        assert!(!Symbol::parse("__").unwrap().walkable());
        assert_eq!(Symbol::parse("XX"), None);
    }

    #[test]
    fn map_rows_ok() {
        let raw = "\n    MB ME --\n    -- SP ME\n";
        let grid = parse_map_rows(raw, Shape { cols: 3, rows: 2 }).unwrap();
        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0][1], Symbol::EnemyMaybe);
        assert_eq!(grid[1][1], Symbol::Spawn);
    }

    #[test]
    fn map_rows_shape_mismatch() {
        let raw = "MB ME\n-- SP\n";
        let err = parse_map_rows(raw, Shape { cols: 3, rows: 2 }).unwrap_err();
        assert!(err.contains("期望 3 列"), "实际错误：{err}");
    }

    #[test]
    fn map_rows_bad_symbol() {
        let err = parse_map_rows("MB ZZ", Shape { cols: 2, rows: 1 }).unwrap_err();
        assert!(err.contains("非法符号"), "实际错误：{err}");
    }
}
