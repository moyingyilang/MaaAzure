//! 领域 · 战役与地图：符号、形状、网格解析。
//!
//! **位置**：`2 领域` 下 `battle` 子树的根。
//! 纯逻辑：输入「一帧观测」，输出「一个命令」，不依赖 MaaFramework、不做 IO，
//! 因此可以对着 golden corpus 录像做单元测试（`docs/5-engineering/5.2-testing.md`）。
//!
//! 数据来源：旧版 `campaign/*.py` 的 `MAP.map_data` 文本
//! （`docs/4-migration/4.3-campaign-data.md`）。

#![deny(missing_docs)]

/// 地图格子符号，与旧版 `map_data` 的 token 一一对应。
///
/// ```text
/// Symbol
/// ├─ 可通行  Sea(--)  Spawn(SP)  EnemyMaybe(ME)  BossMaybe(MB)
/// │          Mystery(MM)  Ammo(MA)  Siren(MS)  Enemy(Me)
/// └─ 不可通行 Land(++)  Empty(__)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    /// `--` 海洋
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
    /// `__` 空位（不可通行）
    Empty,
    /// `Me` 已确认的普通敌人
    Enemy,
}

impl Symbol {
    /// 解析单个 token；非法 token 返回 `None`。
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
    /// 列数（字母 A=1 … K=11）
    pub cols: usize,
    /// 行数
    pub rows: usize,
}

impl Shape {
    /// 解析旧版 `shape` 字段。
    pub fn parse(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let col = chars.next()?.to_ascii_uppercase();
        if !col.is_ascii_uppercase() {
            return None;
        }
        Some(Self {
            cols: (col as u32 - 'A' as u32 + 1) as usize,
            rows: chars.as_str().parse().ok()?,
        })
    }
}

/// 解析 `MAP.map_data` 原文为二维符号表。
///
/// 转换器据此生成待人工处理清单：任何非法 token 或行列不符都会带位置报错。
pub fn parse_map_rows(raw: &str, shape: Shape) -> Result<Vec<Vec<Symbol>>, String> {
    let mut grid = Vec::with_capacity(shape.rows);

    for (i, line) in raw
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        let row = line
            .split_whitespace()
            .map(|t| Symbol::parse(t).ok_or_else(|| format!("第 {i} 行有非法符号 `{t}`")))
            .collect::<Result<Vec<_>, _>>()?;

        if row.len() != shape.cols {
            return Err(format!(
                "第 {i} 行 {} 列，期望 {} 列（shape={}x{}）",
                row.len(),
                shape.cols,
                shape.cols,
                shape.rows
            ));
        }
        grid.push(row);
    }

    if grid.len() != shape.rows {
        return Err(format!("实际 {} 行，期望 {} 行", grid.len(), shape.rows));
    }
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes() {
        assert_eq!(Shape::parse("K8"), Some(Shape { cols: 11, rows: 8 }));
        assert_eq!(Shape::parse("A1"), Some(Shape { cols: 1, rows: 1 }));
        assert_eq!(Shape::parse("88"), None);
    }

    #[test]
    fn walkability() {
        assert!(Symbol::parse("--").unwrap().walkable());
        assert!(Symbol::parse("ME").unwrap().walkable());
        assert!(!Symbol::parse("++").unwrap().walkable());
        assert!(!Symbol::parse("__").unwrap().walkable());
    }

    #[test]
    fn rows_ok_and_errors() {
        let grid = parse_map_rows("\n MB ME --\n -- SP ME\n", Shape { cols: 3, rows: 2 }).unwrap();
        assert_eq!(grid[0][1], Symbol::EnemyMaybe);
        assert_eq!(grid[1][1], Symbol::Spawn);

        assert!(
            parse_map_rows("MB ME", Shape { cols: 3, rows: 1 })
                .unwrap_err()
                .contains("期望 3 列")
        );
        assert!(
            parse_map_rows("MB ZZ", Shape { cols: 2, rows: 1 })
                .unwrap_err()
                .contains("非法符号")
        );
    }
}
