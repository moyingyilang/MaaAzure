//! `convert-assets`：把 `assets.py` 的 2,753 条声明变成 Pipeline 叶子节点。
//!
//! 输出（每个服务器一个资源包）：
//!
//! ```text
//! resource/<server>/pipeline/gen/<module>.json
//! ```
//!
//! 生成的只可能是「叶子」：`next` 一律为空。拓扑（谁接谁）是领域知识，必须人工写。
//!
//! 字段映射见 `docs/4-migration/4.2-assets.md` §2。

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Map, Value, json};

use crate::assets_py::{self, Decl};
use crate::naming;

/// 运行参数。
pub struct Options {
    /// 旧仓根目录
    pub old_root: PathBuf,
    /// 新仓 `resource/` 目录
    pub resource_dir: PathBuf,
    /// 只统计不落盘
    pub dry_run: bool,
}

/// 结果摘要。
#[derive(Debug, Default)]
pub struct Summary {
    /// 声明总数
    pub decls: usize,
    /// 生成的节点数（= 声明数，含跳过项时为 0）
    pub nodes: usize,
    /// 模板匹配节点
    pub template_nodes: usize,
    /// 纯色匹配节点
    pub color_nodes: usize,
    /// 动图节点（目录模板）
    pub animated_nodes: usize,
    /// 写了几个服务器的包
    pub servers: usize,
    /// 写了多少个文件
    pub files: usize,
    /// 缺 `area`（roi 未生成）的节点
    pub without_roi: usize,
}

/// 生成一个节点的 JSON。
///
/// `image` 是包内相对路径（`image/...` 去掉前缀后的部分），直接作为 `template`。
fn build_node(d: &Decl, server: &str, template: &str, animated: bool) -> Value {
    let mut node = Map::new();

    let area = d.areas.iter().find(|(s, _)| s == server).map(|(_, r)| *r);
    let color = d.colors.iter().find(|(s, _)| s == server).map(|(_, c)| *c);
    let button = d.buttons.iter().find(|(s, _)| s == server).map(|(_, r)| *r);

    if animated {
        // 目录模板：MaaFW 递归加载其中所有帧，任一帧命中即命中
        node.insert("template".into(), json!([format!("{template}/")]));
        node.insert("recognition".into(), json!("TemplateMatch"));
    } else {
        node.insert("template".into(), json!(template));
        node.insert("recognition".into(), json!("TemplateMatch"));
    }

    if let Some(a) = area
        && a.2 > a.0
        && a.3 > a.1
    {
        node.insert("roi".into(), json!([a.0, a.1, a.2, a.3]));
    }

    // 纯色按钮（无 file）才用 ColorMatch；实测旧仓 0 条走这里，但保留路径
    if d.files.is_empty() {
        node.insert("recognition".into(), json!("ColorMatch"));
        if let Some(c) = color {
            node.insert("expected".into(), json!([c.0, c.1, c.2]));
        }
    }

    let clickable = button.is_some_and(|b| b.2 > b.0 && b.3 > b.1);
    if clickable {
        let b = button.unwrap();
        node.insert("action".into(), json!("Click"));
        node.insert("target".into(), json!([b.0, b.1, b.2, b.3]));
    } else {
        node.insert("action".into(), json!("DoNothing"));
    }

    node.insert("next".into(), json!([]));
    Value::Object(node)
}

/// 执行转换。
pub fn run(opts: &Options) -> Result<Summary, String> {
    let mut sum = Summary::default();

    let asset_files = find_assets_py(&opts.old_root.join("module"));
    if asset_files.is_empty() {
        return Err(format!(
            "在 {} 下没找到 assets.py，检查 --old-root",
            opts.old_root.display()
        ));
    }

    // server → module → { node: value }
    let mut per_server: BTreeMap<String, BTreeMap<String, Map<String, Value>>> = BTreeMap::new();
    for s in naming::SERVERS {
        per_server.insert(s.to_string(), BTreeMap::new());
    }

    for path in asset_files {
        let module = module_of(&path);
        let src =
            fs::read_to_string(&path).map_err(|e| format!("读 {} 失败: {e}", path.display()))?;

        for d in assets_py::parse_assets_py(&src, &module) {
            sum.decls += 1;
            let node_name = naming::node_name(&module, &d.constant);

            // 每个服务器写一份自己的 gen，用自己包的模板路径
            for (server, rel) in &d.files {
                if !naming::is_server(server) {
                    continue;
                }
                let Some((_, _)) = naming::split_server_path(rel) else {
                    continue;
                };
                let animated = rel.to_ascii_lowercase().ends_with(".gif");
                let template = if animated {
                    naming::gif_dir(&module, &node_name)
                } else {
                    naming::image_path(&module, &node_name)
                };
                // template 相对包的 image/：去掉 "image/" 前缀
                let template = template.trim_start_matches("image/").to_string();

                let node = build_node(&d, server, &template, animated);
                per_server
                    .get_mut(server)
                    .expect("server 表已预置")
                    .entry(module.clone())
                    .or_default()
                    .insert(node_name.clone(), node);

                sum.nodes += 1;
                if animated {
                    sum.animated_nodes += 1;
                } else {
                    sum.template_nodes += 1;
                }
            }
        }
    }

    // 统计 roi 缺失（每次运行都重算，避免累加）
    sum.without_roi = 0;
    for modules in per_server.values() {
        for nodes in modules.values() {
            for n in nodes.values() {
                if n.get("recognition").and_then(Value::as_str) == Some("TemplateMatch")
                    && n.get("roi").is_none()
                {
                    sum.without_roi += 1;
                }
            }
        }
    }

    if opts.dry_run {
        eprintln!("[dry-run] 未写入任何文件");
        sum.servers = per_server.len();
        return Ok(sum);
    }

    let rev = git_rev(&opts.old_root);
    let manifest_dir = opts.resource_dir.join("manifest");
    fs::create_dir_all(&manifest_dir).map_err(|e| format!("建目录失败: {e}"))?;
    for (server, modules) in &per_server {
        let gen_dir = opts.resource_dir.join(server).join("pipeline").join("gen");
        fs::create_dir_all(&gen_dir).map_err(|e| format!("建目录失败: {e}"))?;

        for (module, nodes) in modules {
            // 注意：**不能**在 pipeline JSON 里放 `_meta`。
            // MaaFramework 会把每个顶层键当节点解析，`_meta` 会和别的文件里的 `_meta`
            // 撞成 "key already exists"，整个资源包加载失败（实测踩到）。
            // 元信息统一写到 resource/manifest/<server>.json。
            let mut doc = Map::new();
            for (name, node) in nodes {
                doc.insert(name.clone(), node.clone());
            }
            let text = serde_json::to_string_pretty(&Value::Object(doc))
                .map_err(|e| format!("序列化失败: {e}"))?;
            fs::write(gen_dir.join(format!("{module}.json")), format!("{text}\n"))
                .map_err(|e| format!("写 {} 失败: {e}", module))?;
            sum.files += 1;
        }
        // 元信息落在这里（不在 pipeline/ 下，MaaFW 不会把它当节点）
        let manifest = json!({
            "generated_by": "xtask convert-assets",
            "source_rev": rev,
            "server": server,
            "do_not_edit": true,
            "schema_doc": "docs/4-migration/4.2-assets.md",
            "modules": modules.keys().collect::<Vec<_>>(),
        });
        write_json(&manifest_dir.join(format!("{server}.json")), &manifest)?;
        sum.servers += 1;
    }

    Ok(sum)
}

/// 递归找 `module/**/assets.py`。
fn find_assets_py(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else { continue };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n == "assets.py") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn module_of(path: &Path) -> String {
    path.parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("misc")
        .to_string()
}

fn write_json(p: &Path, v: &Value) -> Result<(), String> {
    let text = serde_json::to_string_pretty(v).map_err(|e| format!("序列化失败: {e}"))?;
    fs::write(p, format!("{text}\n")).map_err(|e| format!("写 {} 失败: {e}", p.display()))
}

fn git_rev(root: &Path) -> String {
    Command::new("git")
        .args([
            "-C",
            &root.display().to_string(),
            "rev-parse",
            "--short",
            "HEAD",
        ])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}
