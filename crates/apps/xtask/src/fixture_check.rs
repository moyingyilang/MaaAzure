//! `fixture-check`：合成帧用例的引用完整性。
//!
//! 用例是 JSON，同样没有编译期检查。这里查：
//!
//! | 检查 | 级别 |
//! |---|---|
//! | `entry` 在 Pipeline 里存在 | 错误 |
//! | `frames[].show` / `expect.visited` / `expect.not_visited` 的节点存在 | 错误 |
//! | `frames` 为空 | 错误 |
//! | 可跑任务（`interface.d/tasks.json`）没有对应用例 | 警告 |

use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::pipeline_check;

/// 检查结果。
#[derive(Debug, Default)]
pub struct Report {
    /// 用例数
    pub fixtures: usize,
    /// 错误
    pub errors: Vec<String>,
    /// 警告
    pub warnings: Vec<String>,
}

impl Report {
    /// 是否通过。
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// 执行检查。
pub fn run(root: &Path, resource_dir: &Path) -> Result<Report, String> {
    let mut r = Report::default();
    let nodes = pipeline_check::all_node_names(resource_dir)?;

    let dir = root.join("data/fixtures");
    let mut entries: Vec<String> = Vec::new();

    if dir.is_dir() {
        let rd = fs::read_dir(&dir).map_err(|e| format!("读 {} 失败: {e}", dir.display()))?;
        let mut files: Vec<_> = rd.flatten().map(|e| e.path()).collect();
        files.sort();
        for p in files {
            if p.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            r.fixtures += 1;
            let text =
                fs::read_to_string(&p).map_err(|e| format!("读 {} 失败: {e}", p.display()))?;
            let doc: Value = serde_json::from_str(&text)
                .map_err(|e| format!("解析 {} 失败: {e}", p.display()))?;
            let name = doc
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("<unnamed>")
                .to_string();

            let Some(entry) = doc.get("entry").and_then(Value::as_str) else {
                r.errors.push(format!("{name}: 缺 entry"));
                continue;
            };
            entries.push(entry.to_string());
            if !nodes.contains(entry) {
                r.errors
                    .push(format!("{name}: entry `{entry}` 在 Pipeline 里不存在"));
            }

            let frames = doc.get("frames").and_then(Value::as_array);
            match frames {
                Some(f) if !f.is_empty() => {
                    for (i, frame) in f.iter().enumerate() {
                        for node in frame
                            .get("show")
                            .and_then(Value::as_array)
                            .into_iter()
                            .flatten()
                            .filter_map(Value::as_str)
                        {
                            if !nodes.contains(node) {
                                r.errors
                                    .push(format!("{name}: 第 {i} 帧引用了不存在的节点 `{node}`"));
                            }
                        }
                    }
                }
                _ => r.errors.push(format!("{name}: frames 为空")),
            }

            for key in ["visited", "not_visited"] {
                for node in doc
                    .get("expect")
                    .and_then(|e| e.get(key))
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                {
                    if !nodes.contains(node) {
                        r.errors
                            .push(format!("{name}: expect.{key} 引用了不存在的节点 `{node}`"));
                    }
                }
            }
        }
    } else {
        r.warnings
            .push("没有 data/fixtures（合成帧用例为空）".to_string());
    }

    // 可跑任务是否都有用例
    let tasks_path = root.join("interface.d/tasks.json");
    if let Ok(text) = fs::read_to_string(&tasks_path)
        && let Ok(doc) = serde_json::from_str::<Value>(&text)
        && let Some(arr) = doc.get("task").and_then(Value::as_array)
    {
        for t in arr {
            let Some(name) = t.get("name").and_then(Value::as_str) else {
                continue;
            };
            let want = format!("{name}Start");
            if !entries.iter().any(|e| e == &want) {
                r.warnings
                    .push(format!("任务 {name} 没有以 `{want}` 为入口的合成帧用例"));
            }
        }
    }

    Ok(r)
}
