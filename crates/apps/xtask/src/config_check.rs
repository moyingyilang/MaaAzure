//! `config-check`：`interface.d/*.json` 的结构与引用完整性。
//!
//! PI v2 是 JSON，同样没有编译期检查。这里查四类问题：
//!
//! | 检查 | 级别 |
//! |---|---|
//! | `task.entry` 在 Pipeline 里存在 | 错误 |
//! | `task.option` 引用的 option 已定义 | 错误 |
//! | option 的类型必填字段齐（select/switch 要 `cases`，input 要 `inputs`） | 错误 |
//! | `task.group` 引用的 group 已定义 | 错误 |
//! | 有 option 没有任何 task 引用（孤儿配置） | 警告 |

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::pipeline_check;

/// 检查结果。
#[derive(Debug, Default)]
pub struct Report {
    /// option 数
    pub options: usize,
    /// task 数
    pub tasks: usize,
    /// group 数
    pub groups: usize,
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
    let idir = root.join("interface.d");
    if !idir.is_dir() {
        return Err(format!("{} 不存在（先跑 convert-config）", idir.display()));
    }

    let options = load(&idir.join("options.json"))?;
    let tasks = load(&idir.join("tasks.json"))?;
    let groups = load(&idir.join("groups.json"))?;

    let mut r = Report::default();

    let opt_map = options.get("option").and_then(Value::as_object);
    let task_arr = tasks.get("task").and_then(Value::as_array);
    let group_arr = groups.get("group").and_then(Value::as_array);

    let Some(opt_map) = opt_map else {
        return Err("options.json 缺少 option 对象".to_string());
    };
    let Some(task_arr) = task_arr else {
        return Err("tasks.json 缺少 task 数组".to_string());
    };
    r.options = opt_map.len();
    r.tasks = task_arr.len();
    r.groups = group_arr.map(|a| a.len()).unwrap_or(0);

    // 1) option 结构
    for (name, opt) in opt_map {
        let ty = opt.get("type").and_then(Value::as_str).unwrap_or("");
        match ty {
            "select" | "switch" | "checkbox" => {
                if opt.get("cases").and_then(Value::as_array).is_none() {
                    r.errors.push(format!("option {name}: {ty} 缺 cases"));
                }
            }
            "input" => {
                if opt.get("inputs").and_then(Value::as_array).is_none() {
                    r.errors.push(format!("option {name}: input 缺 inputs"));
                }
            }
            "" => r.errors.push(format!("option {name}: 缺 type")),
            other => r.warnings.push(format!(
                "option {name}: type `{other}` 未在本工具的支持列表里"
            )),
        }
    }

    // 2) group 引用
    let group_names: BTreeSet<&str> = group_arr
        .into_iter()
        .flatten()
        .filter_map(|g| g.get("name").and_then(Value::as_str))
        .collect();

    // 计划中的任务（tasks.planned.json）不参与 entry 校验，但它们的 option 引用
    // 要算进「被引用」，否则会产生几百条孤儿噪声
    let planned = load(&idir.join("tasks.planned.json")).ok();
    let mut planned_used: BTreeSet<&str> = BTreeSet::new();
    if let Some(arr) = planned
        .as_ref()
        .and_then(|p| p.get("task"))
        .and_then(Value::as_array)
    {
        for t in arr {
            for o in t
                .get("option")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(os) = o.as_str() {
                    planned_used.insert(os);
                }
            }
        }
    }

    // 3) task 引用 + entry 存在性
    let nodes = pipeline_check::all_node_names(resource_dir)?;
    let mut used_options: BTreeSet<&str> = BTreeSet::new();

    for t in task_arr {
        let name = t.get("name").and_then(Value::as_str).unwrap_or("<unnamed>");
        let Some(entry) = t.get("entry").and_then(Value::as_str) else {
            r.errors.push(format!("task {name}: 缺 entry"));
            continue;
        };
        if !nodes.contains(entry) {
            r.errors
                .push(format!("task {name}: entry `{entry}` 在 Pipeline 里不存在"));
        }
        for g in t
            .get("group")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(gs) = g.as_str()
                && !group_names.contains(gs)
            {
                r.errors
                    .push(format!("task {name}: 引用了未定义的 group `{gs}`"));
            }
        }
        for o in t
            .get("option")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(os) = o.as_str() {
                if !opt_map.contains_key(os) {
                    r.errors
                        .push(format!("task {name}: 引用了未定义的 option `{os}`"));
                }
                used_options.insert(os);
            }
        }
    }

    let orphans: Vec<&str> = opt_map
        .keys()
        .map(String::as_str)
        .filter(|k| !used_options.contains(k) && !planned_used.contains(k))
        .collect();
    if !orphans.is_empty() {
        r.warnings.push(format!(
            "{} 个 option 未被任何 task 引用（示例：{:?}）",
            orphans.len(),
            &orphans[..orphans.len().min(3)]
        ));
    }

    Ok(r)
}

fn load(p: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(p).map_err(|e| format!("读 {} 失败: {e}", p.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("解析 {} 失败: {e}", p.display()))
}
