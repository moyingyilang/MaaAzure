//! `build-pipeline`：**接线声明 → 完整 Pipeline JSON**。
//!
//! 手写拓扑时最烦的不是「谁接谁」，而是给每个节点抄 `template` / `roi` / `target`。
//! 抄错就静默偏移，而且和生成的叶子节点会漂移。
//!
//! 所以把这一层拆成两件事：
//!
//! | 文件 | 谁维护 | 内容 |
//! |---|---|---|
//! | `resource/wiring/**/*.json` | **人** | 只有「谁接谁」和少量覆盖 |
//! | `resource/common/pipeline/**/*.json` | `xtask build-pipeline` | 完整节点（含从 gen 抄来的字段） |
//!
//! 接线里用 `"use": "<生成节点名>"` 引用生成器已算好的模板与坐标。
//!
//! ```jsonc
//! // resource/wiring/1-daily/commission.json
//! {
//!   "layer": "1-daily",
//!   "file": "commission/entry.json",
//!   "nodes": {
//!     "CommissionStart": { "next": ["[Anchor]home", "commission_goto_reward"] },
//!     "commission_goto_reward": {
//!       "use": "ui_main_goto_reward",
//!       "next": ["commission_goto_commission", "commission_goto_reward", "global_recover"]
//!     }
//!   }
//! }
//! ```
//!
//! 生成结果里**不带 `_meta`**：MaaFW 会把 pipeline JSON 的每个顶层键当节点，
//! 元信息键会和别的文件撞车（实测踩过，见 docs/5-engineering/5.2-testing.md）。

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

/// 运行参数。
pub struct Options {
    /// 新仓根目录
    pub root: PathBuf,
    /// 资源目录
    pub resource_dir: PathBuf,
    /// 只统计不落盘
    pub dry_run: bool,
}

/// 结果摘要。
#[derive(Debug, Default)]
pub struct Summary {
    /// 接线文件数
    pub wiring_files: usize,
    /// 生成节点数
    pub nodes: usize,
    /// 其中引用了 gen 节点的（`use`）
    pub reused: usize,
    /// 写出的 JSON 文件
    pub written: usize,
    /// 缺少 `use` 目标
    pub missing: Vec<String>,
}

/// 执行。
pub fn run(opts: &Options) -> Result<Summary, String> {
    let mut sum = Summary::default();
    let wiring_dir = opts.root.join("resource/wiring");
    if !wiring_dir.is_dir() {
        return Err(format!("{} 不存在", wiring_dir.display()));
    }

    // 生成节点索引：node 名 → (来源包, 节点 JSON)
    let generated = load_gen_nodes(&opts.resource_dir)?;

    let mut files: Vec<PathBuf> = Vec::new();
    collect_json(&wiring_dir, &mut files);
    files.sort();

    for path in files {
        sum.wiring_files += 1;
        let text =
            fs::read_to_string(&path).map_err(|e| format!("读 {} 失败: {e}", path.display()))?;
        let doc: Value = serde_json::from_str(&text)
            .map_err(|e| format!("解析 {} 失败: {e}", path.display()))?;

        let layer = doc
            .get("layer")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{}: 缺 layer", path.display()))?;
        let file = doc
            .get("file")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{}: 缺 file", path.display()))?;
        let nodes = doc
            .get("nodes")
            .and_then(Value::as_object)
            .ok_or_else(|| format!("{}: 缺 nodes", path.display()))?;

        let mut out = Map::new();
        for (name, spec) in nodes {
            let spec = spec
                .as_object()
                .ok_or_else(|| format!("{}: 节点 {name} 不是对象", path.display()))?;

            // 1) 若声明了 use，先抄 gen 节点的机械字段
            let mut node = Map::new();
            if let Some(use_name) = spec.get("use").and_then(Value::as_str) {
                match generated.get(use_name) {
                    Some(base) => {
                        for key in ["recognition", "template", "roi", "target", "action"] {
                            if let Some(v) = base.get(key) {
                                node.insert(key.to_string(), v.clone());
                            }
                        }
                        sum.reused += 1;
                    }
                    None => sum.missing.push(format!(
                        "{}: 节点 {name} 的 use `{use_name}` 找不到",
                        path.display()
                    )),
                }
            }

            // 2) 接线里的其它字段覆盖/补充（next / anchor / action / recognition / custom_*）
            for (k, v) in spec {
                if k == "use" {
                    continue;
                }
                node.insert(k.clone(), v.clone());
            }

            // 3) 默认补齐
            node.entry("next".to_string()).or_insert(json!([]));
            node.entry("recognition".to_string())
                .or_insert(json!("DirectHit"));
            node.entry("action".to_string())
                .or_insert(json!("DoNothing"));

            out.insert(name.clone(), Value::Object(node));
            sum.nodes += 1;
        }

        if opts.dry_run {
            continue;
        }
        let target = opts
            .resource_dir
            .join("common/pipeline")
            .join(layer)
            .join(file);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("建目录失败: {e}"))?;
        }
        let text = serde_json::to_string_pretty(&Value::Object(out))
            .map_err(|e| format!("序列化失败: {e}"))?;
        fs::write(&target, format!("{text}\n"))
            .map_err(|e| format!("写 {} 失败: {e}", target.display()))?;
        sum.written += 1;
    }

    Ok(sum)
}

/// 收集 `<server>/pipeline/gen/**/*.json` 里的节点，按名字索引（先到先得，通常是 cn）。
fn load_gen_nodes(resource_dir: &Path) -> Result<BTreeMap<String, Value>, String> {
    let mut out: BTreeMap<String, Value> = BTreeMap::new();
    let rd = fs::read_dir(resource_dir)
        .map_err(|e| format!("读 {} 失败: {e}", resource_dir.display()))?;

    let mut servers: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("pipeline/gen").is_dir())
        .collect();
    // 固定顺序：cn 优先，其余按名字
    servers.sort_by_key(|p| {
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        (name != "cn", name)
    });

    for srv in servers {
        let mut files = Vec::new();
        collect_json(&srv.join("pipeline/gen"), &mut files);
        files.sort();
        for f in files {
            let Ok(text) = fs::read_to_string(&f) else {
                continue;
            };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else {
                continue;
            };
            if let Some(obj) = doc.as_object() {
                for (k, v) in obj {
                    if v.is_object() {
                        out.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
            }
        }
    }
    Ok(out)
}

fn collect_json(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().and_then(|x| x.to_str()) == Some("json") {
                out.push(p);
            }
        }
    }
}
