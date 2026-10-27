//! `pipeline-check`：JSON 没有编译期检查，这是唯一防线。
//!
//! 检查项（`docs/3-authoring/3.1-pipeline-spec.md` §7）：
//!
//! | 检查 | 级别 |
//! |---|---|
//! 检查按**运行时语义**进行：每个服务器包先叠加 `resource/common`（手写拓扑），
//! 再叠加 `resource/<server>`（生成的叶子），后者覆盖前者。
//!
//! | `next` / `on_error` 引用的节点存在 | 错误 |
//! | `[Anchor]X` 有对应的 `anchor: "X"` 设置点 | 错误 |
//! | `template` 指向的图片/目录在包内存在 | 错误 |
//! | `roi` 四元组合法且不出 1280×720 | 错误 |
//! | 无入边且非 `*Start` 的孤儿节点 | 警告 |
//! | 节点 `next` 超过 12 项（帧预算） | 警告 |

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::naming;

/// 一个节点的来源。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// `pipeline/gen/`（xtask 生成，`next` 恒为空，不参与孤儿判定）
    Generated,
    /// 手写拓扑
    Handwritten,
}

/// 一次检查的统计。
#[derive(Debug, Default)]
pub struct Report {
    /// 检查的服务器包数
    pub bundles: usize,
    /// 节点总数
    pub nodes: usize,
    /// 生成叶子数（`gen/`，未接线属正常）
    pub generated: usize,
    /// 错误
    pub errors: Vec<String>,
    /// 警告
    pub warnings: Vec<String>,
}

impl Report {
    /// 是否通过（无错误）。
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// 执行检查。
pub fn run(resource_dir: &Path) -> Result<Report, String> {
    let mut report = Report::default();
    let bundles = list_bundles(resource_dir)?;
    if bundles.is_empty() {
        return Err(format!("{} 下没有资源包（目录名应为服务器名）", resource_dir.display()));
    }

    for (server, bundle) in bundles {
        report.bundles += 1;
        // 运行时是先加载 common 再加载 server 包，后者覆盖前者
        let mut nodes: BTreeMap<String, (Value, Source)> = BTreeMap::new();
        if let Some(common) = common_bundle(resource_dir) {
            nodes.extend(load_nodes(&common.join("pipeline"))?);
        }
        nodes.extend(load_nodes(&bundle.join("pipeline"))?);
        report.nodes += nodes.len();
        report.generated += nodes.values().filter(|(_, s)| *s == Source::Generated).count();

        // 1) 引用完整性 + 锚点
        let anchors: BTreeSet<String> = nodes
            .iter()
            .filter_map(|(_, (n, _))| n.get("anchor"))
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect();

        for (name, (node, _src)) in &nodes {
            for field in ["next", "on_error"] {
                let Some(list) = node.get(field) else { continue };
                let Some(items) = list.as_array() else {
                    report.errors.push(format!("[{server}] {name}: {field} 不是数组"));
                    continue;
                };
                if field == "next" && items.len() > 12 {
                    report
                        .warnings
                        .push(format!("[{server}] {name}: next 有 {} 项，帧预算风险", items.len()));
                }
                for it in items {
                    let Some(s) = it.as_str() else { continue };
                    if let Some(anchor) = s.strip_prefix("[Anchor]") {
                        if !anchors.contains(anchor) {
                            report
                                .errors
                                .push(format!("[{server}] {name}: 引用了未设置的锚点 `{anchor}`"));
                        }
                        continue;
                    }
                    if let Some(back) = s.strip_prefix("[JumpBack]") {
                        if !nodes.contains_key(back) {
                            report.errors.push(format!(
                                "[{server}] {name}: [JumpBack] 指向不存在的节点 `{back}`"
                            ));
                        }
                        continue;
                    }
                    if !nodes.contains_key(s) {
                        report
                            .errors
                            .push(format!("[{server}] {name}: {field} 指向不存在的节点 `{s}`"));
                    }
                }
            }
        }

        // 2) 图片存在性 + roi 合法性
        for (name, (node, _src)) in &nodes {
            if let Some(tpl) = node.get("template") {
                let paths: Vec<String> = match tpl {
                    Value::String(s) => vec![s.clone()],
                    Value::Array(a) => a.iter().filter_map(Value::as_str).map(str::to_string).collect(),
                    _ => {
                        report.errors.push(format!("[{server}] {name}: template 类型错误"));
                        vec![]
                    }
                };
                for p in paths {
                    let full = bundle.join("image").join(p.trim_end_matches('/'));
                    let ok = if p.ends_with('/') { full.is_dir() } else { full.is_file() };
                    if !ok {
                        report
                            .errors
                            .push(format!("[{server}] {name}: 图片不存在 `{p}`"));
                    }
                }
            }

            if let Some(roi) = node.get("roi").and_then(Value::as_array) {
                let v: Vec<i64> = roi.iter().filter_map(Value::as_i64).collect();
                if v.len() != 4 {
                    report.errors.push(format!("[{server}] {name}: roi 必须是 4 个数字"));
                } else if !(v[0] >= 0 && v[1] >= 0 && v[2] > v[0] && v[3] > v[1] && v[2] <= 1280 && v[3] <= 720) {
                    report
                        .errors
                        .push(format!("[{server}] {name}: roi 越界或非法 {:?}", v));
                }
            }
        }

        // 3) 孤儿节点（警告）：无入边且不是 *Start
        let mut has_incoming: BTreeSet<&str> = BTreeSet::new();
        for (node, _) in nodes.values() {
            for field in ["next", "on_error"] {
                if let Some(items) = node.get(field).and_then(Value::as_array) {
                    for it in items {
                        if let Some(s) = it.as_str() {
                            let s = s.strip_prefix("[JumpBack]").unwrap_or(s);
                            if !s.starts_with("[Anchor]") {
                                has_incoming.insert(s);
                            }
                        }
                    }
                }
            }
        }
        for (name, (node, src)) in &nodes {
            if *src == Source::Generated {
                // gen/ 的叶子 next 恒为空，未接线是设计态而非缺陷
                continue;
            }
            // 设置锚点的节点是入口（被 [Anchor] 引用而非节点名），不算孤儿
            if node.get("anchor").is_some() {
                continue;
            }
            if !has_incoming.contains(name.as_str()) && !name.ends_with("Start") {
                report
                    .warnings
                    .push(format!("[{server}] {name}: 孤儿节点（无入边且非 *Start）"));
            }
        }
    }

    Ok(report)
}

/// `resource/common` 包（可选）：手写拓扑与 `default_pipeline.json` 的宿主。
fn common_bundle(resource_dir: &Path) -> Option<PathBuf> {
    let p = resource_dir.join("common");
    p.join("pipeline").is_dir().then_some(p)
}

/// 列出服务器资源包：`resource/<server>/` 且存在 `pipeline/`。
fn list_bundles(resource_dir: &Path) -> Result<Vec<(String, PathBuf)>, String> {
    let rd = fs::read_dir(resource_dir)
        .map_err(|e| format!("读 {} 失败: {e}", resource_dir.display()))?;
    let mut out = Vec::new();
    for entry in rd.flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if naming::is_server(&name) && p.join("pipeline").is_dir() {
            out.push((name, p));
        }
    }
    out.sort();
    Ok(out)
}

/// 加载一个包里 `pipeline/**/*.json` 的全部节点（含 gen/）。
fn load_nodes(pipeline_dir: &Path) -> Result<BTreeMap<String, (Value, Source)>, String> {
    let mut nodes: BTreeMap<String, (Value, Source)> = BTreeMap::new();
    let mut stack = vec![pipeline_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else { continue };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            if p.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = fs::read_to_string(&p).map_err(|e| format!("读 {} 失败: {e}", p.display()))?;
            let doc: Value = serde_json::from_str(&text)
                .map_err(|e| format!("解析 {} 失败: {e}", p.display()))?;
            let Some(obj) = doc.as_object() else {
                return Err(format!("{} 顶层不是对象", p.display()));
            };
            let src = if p.components().any(|c| c.as_os_str() == "gen") {
                Source::Generated
            } else {
                Source::Handwritten
            };
            for (k, v) in obj {
                if k.starts_with("_meta") {
                    continue;
                }
                if v.as_object().is_none() {
                    return Err(format!("{} 的节点 `{k}` 不是对象", p.display()));
                }
                if let Some((_, prev_src)) = nodes.insert(k.clone(), (v.clone(), src)) {
                    // 同名覆盖是 MaaFW 的 bundle 语义（后加载覆盖先加载），
                    // 同一个包内出现同名才是错误
                    if prev_src == src {
                        return Err(format!("节点 `{k}` 在同一包内重复定义（{}）", p.display()));
                    }
                }
            }
        }
    }
    Ok(nodes)
}
