//! `convert-campaign`：`campaign/**/*.py` → `data/campaign/**/*.json`。
//!
//! 数据不再编译进代码：新增活动 = 加目录 + 重跑本工具，不改 Rust
//! （`docs/4-migration/4.3-campaign-data.md`）。
//!
//! 每个文件都会做**结构性校验**：
//!
//! | 检查 | 依据 |
//! |---|---|
//! | 行列数与 `shape` 一致 | `K`=11 列，数字=行数（`maazure_battle::Shape`） |
//! | 每行 token 合法 | `++ -- SP ME MB MM MA MS __ Me` |
//! | `spawn_data` 的 `battle` 从 0 连续递增 | 旧版约定 |
//!
//! 校验复用 `maazure-battle` 的解析器——这是**领域 crate 被工具链真实使用**的证据，
//! 而不是两份实现互相漂移。

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use maazure_battle::{parse_map_rows, Shape};
use serde_json::{Value, json};

use crate::campaign_py;

/// 运行参数。
pub struct Options {
    /// 旧仓根目录
    pub old_root: PathBuf,
    /// 新仓 `data/` 目录
    pub data_dir: PathBuf,
    /// 只统计不落盘
    pub dry_run: bool,
}

/// 结果摘要。
#[derive(Debug, Default)]
pub struct Summary {
    /// 扫描到的 .py 文件数
    pub scanned: usize,
    /// 含 `MAP.map_data` 的地图文件数
    pub maps: usize,
    /// 写出的 JSON 数
    pub written: usize,
    /// 无 `map_data` 的辅助文件（基类、`__init__` 等）
    pub skipped: usize,
    /// 校验失败
    pub errors: Vec<String>,
    /// 有无法静态求值字段的文件
    pub unresolved: Vec<String>,
    /// 地图里的未知符号（保留数据、显式登记）
    pub unknown_tokens: Vec<String>,
}

/// 执行转换。
pub fn run(opts: &Options) -> Result<Summary, String> {
    let mut sum = Summary::default();
    let campaign_root = opts.old_root.join("campaign");
    if !campaign_root.is_dir() {
        return Err(format!("找不到 {}", campaign_root.display()));
    }

    let files = collect_py(&campaign_root);
    let rev = git_rev(&opts.old_root);

    for path in files {
        sum.scanned += 1;
        let rel = path
            .strip_prefix(&opts.old_root)
            .map_err(|e| format!("相对路径失败: {e}"))?
            .to_path_buf();

        let src = fs::read_to_string(&path)
            .map_err(|e| format!("读 {} 失败: {e}", path.display()))?;
        let parsed = campaign_py::parse_campaign_py(&src)
            .map_err(|e| format!("解析 {} 失败: {e}", path.display()))?;

        if !parsed.unresolved.is_empty() {
            sum.unresolved.push(format!(
                "{}: {:?}",
                rel.display(),
                parsed.unresolved.iter().map(|(k, _)| k).collect::<Vec<_>>()
            ));
        }

        let Some(map_data) = parsed.text("map_data") else {
            sum.skipped += 1;
            continue;
        };
        sum.maps += 1;

        // ── 校验 ──
        // 旧仓有一批文件不写 shape（从 Config 继承），这时按 map_data 的实际行列推断，
        // 并在产物里标记 shape_inferred，避免被误当成"已声明"。
        let (shape, shape_inferred) = match parsed.text("shape") {
            Some(t) => match Shape::parse(t) {
                Some(s) => (s, false),
                None => {
                    sum.errors
                        .push(format!("{}: shape `{t}` 无法解析", rel.display()));
                    continue;
                }
            },
            None => match infer_shape(map_data) {
                Some(s) => (s, true),
                None => {
                    sum.errors
                        .push(format!("{}: 既无 shape 也无法从 map_data 推断", rel.display()));
                    continue;
                }
            },
        };
        // 未知 token 不当成致命错误：旧仓有 4 处历史遗留（`SI`、单 `-`），
        // 数据要原样保留并显式登记，运行时按不可通行处理。
        let mut unknown_tokens: Vec<String> = Vec::new();
        if let Err(e) = parse_map_rows(map_data, shape) {
            if e.contains("非法符号") {
                unknown_tokens.push(e.clone());
                sum.unknown_tokens.push(format!("{}: {e}", rel.display()));
            } else {
                sum.errors.push(format!("{}: {e}", rel.display()));
                continue;
            }
        }
        let shape_text = format!("{}{}", (b'A' + shape.cols as u8 - 1) as char, shape.rows);
        if let Some(spawn) = parsed.fields.get("spawn_data").and_then(Value::as_array) {
            for (i, item) in spawn.iter().enumerate() {
                let b = item.get("battle").and_then(Value::as_i64).unwrap_or(-1);
                if b != i as i64 {
                    sum.errors.push(format!(
                        "{}: spawn_data[{i}].battle = {b}，应从 0 连续递增",
                        rel.display()
                    ));
                    break;
                }
            }
        }

        // ── 落盘 ──
        let out_path = opts
            .data_dir
            .join("campaign")
            .join(rel.strip_prefix("campaign").unwrap_or(&rel))
            .with_extension("json");

        let doc = json!({
            "_meta": {
                "source": rel.display().to_string(),
                "source_rev": rev,
                "generated_by": "xtask convert-campaign",
                "do_not_edit": true,
                "schema_doc": "docs/4-migration/4.3-campaign-data.md",
            },
            "chapter": parsed.chapter,
            "inherits": parsed.inherits,
            "shape": shape_text,
            "shape_inferred": shape_inferred,
            "grid": { "cols": shape.cols, "rows": shape.rows },
            "fields": parsed.fields,
            "flags": parsed.flags,
            "validation": { "unknown_tokens": unknown_tokens },
        });

        if !opts.dry_run {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| format!("建目录失败: {e}"))?;
            }
            let text = serde_json::to_string_pretty(&doc)
                .map_err(|e| format!("序列化失败: {e}"))?;
            fs::write(&out_path, format!("{text}\n"))
                .map_err(|e| format!("写 {} 失败: {e}", out_path.display()))?;
            sum.written += 1;
        }
    }

    Ok(sum)
}

/// 按 `map_data` 的实际内容推断形状：行数 = 非空行，列数 = 首行 token 数。
fn infer_shape(map_data: &str) -> Option<Shape> {
    let mut rows = 0usize;
    let mut cols = 0usize;
    for line in map_data.lines().map(str::trim).filter(|l| !l.is_empty()) {
        rows += 1;
        if rows == 1 {
            cols = line.split_whitespace().count();
        }
    }
    if rows == 0 || cols == 0 {
        None
    } else {
        Some(Shape { cols, rows })
    }
}

/// 递归收集 `campaign/**/*.py`。
fn collect_py(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else { continue };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().and_then(|e| e.to_str()) == Some("py") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn git_rev(root: &Path) -> String {
    Command::new("git")
        .args(["-C", &root.display().to_string(), "rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

/// 供上层打印用：把错误按文件去重。
pub fn summarize_errors(errors: &[String]) -> BTreeMap<String, usize> {
    let mut m = BTreeMap::new();
    for e in errors {
        let key = e.split(':').next().unwrap_or("?").to_string();
        *m.entry(key).or_insert(0) += 1;
    }
    m
}
