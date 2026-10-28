//! `convert-config`：旧仓 4 层 YAML 生成的 `args.json`（2,401 项）→ PI v2 配置 + 强类型。
//!
//! 产物：
//!
//! ```text
//! interface.d/options.json          精选 option（每任务 ≤ MAX_EXPOSED，被 interface.json import）
//! interface.d/options.all.json      全部 option（参考用，不被 import）
//! interface.d/groups.json           任务分组（来自 menu.json）
//! interface.d/tasks.json            任务（只收「入口节点已存在」的，被 import）
//! interface.d/tasks.planned.json    其余任务（P1 清单，不被 import）
//! crates/domain/config/src/generated.rs   强类型
//! ```
//!
//! 类型映射见 `docs/4-migration/4.4-config.md`。

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

/// 每个任务最多暴露多少配置项。
///
/// 旧版 2,401 项全塞进 GUI，其中大量是「调过一次再没动过」的。这里按
/// `looks_tunable` 排序后取前 N，剩下的仍在 `options.all.json` 里可查。
const MAX_EXPOSED: usize = 10;

/// 运行参数。
pub struct Options {
    /// 旧仓根目录
    pub old_root: PathBuf,
    /// 新仓根目录（写 interface.d 与 generated.rs）
    pub new_root: PathBuf,
    /// 可执行产物目录（用来判断入口节点是否已存在）
    pub resource_dir: PathBuf,
    /// 只统计不落盘
    pub dry_run: bool,
}

/// 结果摘要。
#[derive(Debug, Default)]
pub struct Summary {
    /// args.json 里的任务数
    pub tasks: usize,
    /// 参数总数
    pub params: usize,
    /// 生成 option 数（全部）
    pub options_all: usize,
    /// 生成 option 数（精选）
    pub options_exposed: usize,
    /// 内部项（storage/stored/state）
    pub internal: usize,
    /// `display: hide` 项
    pub hidden: usize,
    /// 有入口节点的任务
    pub runnable_tasks: usize,
    /// 待接线任务
    pub planned_tasks: usize,
    /// 生成的结构体数
    pub structs: usize,
}

/// 旧类型 → 我们的归类。
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Select,
    Switch,
    Input,
    Internal,
}

fn classify(old_type: &str) -> Kind {
    match old_type {
        "select" => Kind::Select,
        "checkbox" => Kind::Switch,
        "input" | "datetime" | "textarea" => Kind::Input,
        _ => Kind::Internal,
    }
}

/// 名字看起来像「用户会调」的项。
///
/// 纯启发式，写在这里而不是散落在代码里：优先级 = 非内部关键词 > 短名 > 字典序。
fn looks_tunable(group: &str, arg: &str) -> bool {
    const NOISY_GROUP: [&str; 3] = ["Scheduler", "Error", "Optimization"];
    const NOISY_ARG: [&str; 6] = [
        "OCR",
        "Timeout",
        "RateLimit",
        "Interval",
        "Threshold",
        "Retry",
    ];
    if NOISY_GROUP.contains(&group) {
        return false;
    }
    !NOISY_ARG.iter().any(|k| arg.contains(k))
}

/// `CamelCase` → `snake_case`。
fn snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    let mut prev_lower = false;
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_lower = false;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        } else {
            out.push('_');
            prev_lower = false;
        }
    }
    out
}

/// Rust 标识符（避开关键字）。
fn ident(s: &str) -> String {
    const KEYWORDS: [&str; 8] = [
        "type", "match", "move", "ref", "self", "super", "crate", "fn",
    ];
    if KEYWORDS.contains(&s) {
        format!("r#{s}")
    } else if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("t_{s}")
    } else {
        s.to_string()
    }
}

/// 执行转换。
pub fn run(opts: &Options) -> Result<Summary, String> {
    let mut sum = Summary::default();
    let args_path = opts.old_root.join("module/config/argument/args.json");
    let menu_path = opts.old_root.join("module/config/argument/menu.json");

    let args: Value = read_json(&args_path)?;
    let menu: Value = read_json(&menu_path)?;
    let Some(tasks_map) = args.as_object() else {
        return Err("args.json 顶层不是对象".to_string());
    };

    let entries = existing_entries(&opts.resource_dir)?;

    // group: menu.json 的键
    let mut groups: Vec<Value> = Vec::new();
    let mut task_group: BTreeMap<String, Vec<String>> = BTreeMap::new();
    if let Some(m) = menu.as_object() {
        for (gname, gval) in m {
            groups.push(json!({ "name": gname, "label": gname }));
            if let Some(list) = gval.get("tasks").and_then(Value::as_array) {
                for t in list.iter().filter_map(Value::as_str) {
                    task_group
                        .entry(t.to_string())
                        .or_default()
                        .push(gname.clone());
                }
            }
        }
    }

    let mut options_all: Map<String, Value> = Map::new();
    let mut options_exposed: Map<String, Value> = Map::new();
    let mut tasks_out: Vec<Value> = Vec::new();
    let mut planned_out: Vec<Value> = Vec::new();
    let mut gen_structs = String::new();
    let mut gen_tasks_fields = String::new();

    let mut sorted_tasks: Vec<(&String, &Value)> = tasks_map.iter().collect();
    sorted_tasks.sort_by(|a, b| a.0.cmp(b.0));

    for (task, tv) in sorted_tasks {
        sum.tasks += 1;
        let Some(groups_obj) = tv.as_object() else {
            continue;
        };

        // 一个任务的所有可暴露项，先收集再排序取前 N
        let mut candidates: Vec<(String, String, String, Kind, Value, bool)> = Vec::new();

        for (group, gv) in groups_obj {
            let Some(args_obj) = gv.as_object() else {
                continue;
            };
            for (arg, av) in args_obj {
                sum.params += 1;
                let old_type = av.get("type").and_then(Value::as_str).unwrap_or("input");
                let kind = classify(old_type);
                let hidden = av.get("display").and_then(Value::as_str) == Some("hide");
                if kind == Kind::Internal {
                    sum.internal += 1;
                    continue;
                }
                if hidden {
                    sum.hidden += 1;
                    continue;
                }

                let name = format!("{task}_{group}_{arg}");
                let opt = build_option(kind, av);
                options_all.insert(name.clone(), opt.clone());
                sum.options_all += 1;

                let tunable = looks_tunable(group, arg);
                candidates.push((group.clone(), arg.clone(), name, kind, opt, tunable));
            }
        }

        // 排序：可调项优先 → 名字短的优先（短名通常是主开关）→ 字典序
        candidates.sort_by(|a, b| {
            b.5.cmp(&a.5)
                .then(a.1.len().cmp(&b.1.len()))
                .then(a.1.cmp(&b.1))
        });

        let mut exposed_names: Vec<String> = Vec::new();
        for (.., name, _kind, opt, _) in candidates.iter().take(MAX_EXPOSED) {
            if options_exposed.insert(name.clone(), opt.clone()).is_none() {
                sum.options_exposed += 1;
            }
            exposed_names.push(name.clone());
        }

        // 强类型：一个任务一个结构体，字段名 = <group>_<arg>
        let struct_name = sanitize_struct(task);
        if !candidates.is_empty() {
            gen_structs.push_str(&gen_struct(&struct_name, task, &candidates)?);
            sum.structs += 1;
            gen_tasks_fields.push_str(&format!(
                "    /// `{task}` 任务\n    #[serde(rename = \"{task}\", default)]\n    pub {}: Option<{struct_name}>,\n",
                ident(&snake(task))
            ));
        }

        // 任务：入口节点存在才算「可跑」
        let entry = format!("{task}Start");
        let mut t = Map::new();
        t.insert("name".into(), json!(task));
        t.insert("entry".into(), json!(entry));
        t.insert("label".into(), json!(task));
        if let Some(gs) = task_group.get(task) {
            t.insert("group".into(), json!(gs));
        }
        t.insert("option".into(), json!(exposed_names));
        if entries.contains(&entry) {
            t.insert("default_check".into(), json!(true));
            tasks_out.push(Value::Object(t));
            sum.runnable_tasks += 1;
        } else {
            t.insert("default_check".into(), json!(false));
            t.insert(
                "description".into(),
                json!("入口节点尚未接线（P1），见 docs/6-plan/6.2-task-tree.md"),
            );
            planned_out.push(Value::Object(t));
            sum.planned_tasks += 1;
        }
    }

    if opts.dry_run {
        eprintln!("[dry-run] 未写入任何文件");
        return Ok(sum);
    }

    // interface.d
    let idir = opts.new_root.join("interface.d");
    fs::create_dir_all(&idir).map_err(|e| format!("建目录失败: {e}"))?;
    write_json(&idir.join("groups.json"), &json!({ "group": groups }))?;
    write_json(
        &idir.join("options.json"),
        &json!({ "option": Value::Object(options_exposed) }),
    )?;
    write_json(
        &idir.join("options.all.json"),
        &json!({ "option": Value::Object(options_all) }),
    )?;
    write_json(&idir.join("tasks.json"), &json!({ "task": tasks_out }))?;
    write_json(
        &idir.join("tasks.planned.json"),
        &json!({ "task": planned_out }),
    )?;

    // generated.rs
    let gen_path = opts.new_root.join("crates/domain/config/src/generated.rs");
    let mut out = String::new();
    out.push_str("//! 由 `xtask convert-config` 生成，**不要手改**。\n");
    out.push_str("//!\n");
    out.push_str("//! 字段名沿用旧仓的 CamelCase（`rename`），这样老配置文件可以直接读。\n");
    out.push_str("//! 重新生成：`cargo run -p xtask -- convert-config`\n\n");
    out.push_str("#![allow(missing_docs)]\n\n");
    out.push_str("use serde::Deserialize;\n\n");
    out.push_str(&gen_structs);
    out.push_str("/// 全部任务的配置根。\n");
    out.push_str("#[derive(Debug, Default, Clone, Deserialize)]\n");
    out.push_str("pub struct Tasks {\n");
    out.push_str(&gen_tasks_fields);
    out.push_str("}\n");
    fs::write(&gen_path, out).map_err(|e| format!("写 {} 失败: {e}", gen_path.display()))?;

    Ok(sum)
}

/// 生成一个任务的嵌套结构体：`<Task>` → `<Task><Group>` → 字段。
///
/// 旧配置是三层嵌套 `{"Task": {"Group": {"Arg": value}}}`，
/// 所以类型也必须是三层——拍平成一层会让 loader 永远读到 `None`。
fn gen_struct(
    struct_name: &str,
    task: &str,
    candidates: &[(String, String, String, Kind, Value, bool)],
) -> Result<String, String> {
    // group → [(arg, kind)]
    let mut groups: BTreeMap<String, Vec<(&str, Kind)>> = BTreeMap::new();
    for (group, arg, ..) in candidates {
        groups
            .entry(group.clone())
            .or_default()
            .push((arg.as_str(), kind_of(candidates, group, arg)));
    }

    let mut s = String::new();
    let mut task_fields = String::new();

    for (group, args) in &groups {
        let gname = format!("{struct_name}{}", sanitize_struct(group));
        s.push_str(&format!(
            "/// `{task}.{group}` 组的配置。\n#[derive(Debug, Default, Clone, Deserialize)]\npub struct {gname} {{\n"
        ));
        let mut used: BTreeSet<String> = BTreeSet::new();
        for (arg, kind) in args {
            let mut field = ident(&snake(arg));
            while used.contains(&field) {
                field.push('_');
            }
            used.insert(field.clone());
            let ty = match kind {
                Kind::Switch => "Option<bool>",
                _ => "Option<String>",
            };
            s.push_str(&format!(
                "    /// 旧键 `{task}.{group}.{arg}`\n    #[serde(rename = \"{arg}\", default)]\n    pub {field}: {ty},\n"
            ));
        }
        s.push_str("}\n\n");

        task_fields.push_str(&format!(
            "    /// 旧键 `{task}.{group}.*`\n    #[serde(rename = \"{group}\", default)]\n    pub {}: Option<{gname}>,\n",
            ident(&snake(group))
        ));
    }

    s.push_str(&format!(
        "/// `{task}` 任务的配置（三层嵌套，与旧配置同形）。\n#[derive(Debug, Default, Clone, Deserialize)]\npub struct {struct_name} {{\n"
    ));
    s.push_str(&task_fields);
    s.push_str("}\n\n");
    Ok(s)
}

/// 取某 (group, arg) 的 Kind。
fn kind_of(
    candidates: &[(String, String, String, Kind, Value, bool)],
    group: &str,
    arg: &str,
) -> Kind {
    candidates
        .iter()
        .find(|(g, a, ..)| g == group && a == arg)
        .map(|(_, _, _, k, _, _)| *k)
        .unwrap_or(Kind::Input)
}

/// 生成一个 PI v2 option 定义。
fn build_option(kind: Kind, av: &Value) -> Value {
    let default = av.get("value").cloned().unwrap_or(Value::Null);
    match kind {
        Kind::Select => {
            let cases: Vec<Value> = av
                .get("option")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|v| json!({ "name": value_to_case(v) }))
                        .collect()
                })
                .unwrap_or_default();
            json!({
                "type": "select",
                "cases": cases,
                "default_case": value_to_case(&default),
            })
        }
        Kind::Switch => json!({
            "type": "switch",
            "cases": [{ "name": "true" }, { "name": "false" }],
            "default_case": if default.as_bool().unwrap_or(false) { "true" } else { "false" },
        }),
        _ => json!({
            "type": "input",
            "inputs": [{
                "name": "value",
                "default": value_to_string(&default),
            }],
        }),
    }
}

fn value_to_case(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// 结构体名：非字母数字压成 `_`，数字开头补 `T`。
fn sanitize_struct(task: &str) -> String {
    let mut out = String::new();
    for ch in task.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out = format!("T{out}");
    }
    out
}

/// 收集 `resource/<server>/pipeline/**` 里已存在的节点名。
fn existing_entries(resource_dir: &Path) -> Result<BTreeSet<String>, String> {
    let mut set = BTreeSet::new();
    let mut stack = vec![resource_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            if p.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = fs::read_to_string(&p) else {
                continue;
            };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else {
                continue;
            };
            if let Some(obj) = doc.as_object() {
                for k in obj.keys() {
                    if !k.starts_with("_meta") {
                        set.insert(k.clone());
                    }
                }
            }
        }
    }
    Ok(set)
}

fn read_json(p: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(p).map_err(|e| format!("读 {} 失败: {e}", p.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("解析 {} 失败: {e}", p.display()))
}

fn write_json(p: &Path, v: &Value) -> Result<(), String> {
    let text = serde_json::to_string_pretty(v).map_err(|e| format!("序列化失败: {e}"))?;
    fs::write(p, format!("{text}\n")).map_err(|e| format!("写 {} 失败: {e}", p.display()))
}
