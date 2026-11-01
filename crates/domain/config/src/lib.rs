//! 任务配置：由 `interface.json` 的 option 定义生成的强类型视图。
//!
//! **位置**：`2 领域` 下的 config 子树。只依赖 `1 地基` 与 serde。
//!
//! 生成物在 [`generated`]，**不要手改**：
//!
//! ```bash
//! cargo run -p xtask -- convert-config
//! ```
//!
//! 旧版靠 `self.config.Commission_Receive` 的字符串拼接访问配置，拼错只在运行时
//! KeyError；这里换成结构体字段，拼错是编译错误。

#![deny(missing_docs)]

pub mod generated;

use std::path::Path;

use serde::de::DeserializeOwned;

/// 一份配置文件的加载结果。
pub struct Config {
    raw: serde_json::Value,
}

impl Config {
    /// 从 JSON 文件加载（旧版的 `config/alas.json` 也能直接读，字段名沿用）。
    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let p = path.as_ref();
        let text =
            std::fs::read_to_string(p).map_err(|e| format!("读 {} 失败: {e}", p.display()))?;
        Self::from_json(&text)
    }

    /// 从 JSON 文本加载。
    pub fn from_json(text: &str) -> Result<Self, String> {
        let raw: serde_json::Value =
            serde_json::from_str(text).map_err(|e| format!("解析配置失败: {e}"))?;
        Ok(Self { raw })
    }

    /// 取一个任务的配置，转成强类型。
    pub fn task<T: DeserializeOwned>(&self, name: &str) -> Result<Option<T>, String> {
        let Some(v) = self.raw.get(name) else {
            return Ok(None);
        };
        serde_json::from_value(v.clone())
            .map(Some)
            .map_err(|e| format!("任务 {name} 的配置类型不匹配: {e}"))
    }

    /// 取全部任务的配置。
    pub fn tasks(&self) -> Result<generated::Tasks, String> {
        serde_json::from_value(self.raw.clone()).map_err(|e| format!("配置类型不匹配: {e}"))
    }

    /// 设置某个任务的下次运行时间（`next_run`）。
    ///
    /// 对应上游 `config.task_delay(target=...)` / `task_delay(success=False)`：
    /// 传 `None` 表示「没有已知完成时刻」，由调度器按失败重试处理。
    pub fn set_next_run(&mut self, task: &str, value: Option<String>) {
        let Some(obj) = self.raw.as_object_mut() else {
            return;
        };
        let entry = obj
            .entry(task.to_string())
            .or_insert_with(|| serde_json::json!({}));
        if let Some(t) = entry.as_object_mut() {
            match value {
                Some(v) => {
                    t.insert("next_run".to_string(), serde_json::Value::String(v));
                }
                None => {
                    t.remove("next_run");
                }
            }
        }
    }

    /// 读某个任务的 `next_run`。
    pub fn next_run(&self, task: &str) -> Option<&str> {
        self.raw.get(task)?.get("next_run")?.as_str()
    }

    /// 写回磁盘（保持两空格缩进，便于人工检查）。
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let p = path.as_ref();
        let text = serde_json::to_string_pretty(&self.raw)
            .map_err(|e| format!("序列化配置失败: {e}"))?;
        std::fs::write(p, format!("{text}\n"))
            .map_err(|e| format!("写 {} 失败: {e}", p.display()))
    }

    /// 原始 JSON（排查用）。
    pub fn raw(&self) -> &serde_json::Value {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_typed_task() {
        // 字段名沿用旧仓的 CamelCase，便于直接读老配置
        // 与旧配置同形：Task → Group → Arg，三层嵌套
        let text = r#"{
            "Commission": {
                "Commission": { "PresetFilter": "custom" },
                "Scheduler": { "Enable": true }
            },
            "Alas": { "Error": {} }
        }"#;
        let cfg = Config::from_json(text).unwrap();
        let c: generated::Commission = cfg.task("Commission").unwrap().unwrap();
        assert_eq!(
            c.commission.as_ref().unwrap().preset_filter.as_deref(),
            Some("custom")
        );
        assert_eq!(c.scheduler.as_ref().unwrap().enable, Some(true));

        // 缺组/缺字段不报错（旧配置往往不全）
        let partial = Config::from_json(r#"{"Commission": {"Commission": {}}}"#).unwrap();
        let c2: generated::Commission = partial.task("Commission").unwrap().unwrap();
        assert!(c2.commission.unwrap().preset_filter.is_none());
        assert!(c2.scheduler.is_none());

        // 不存在的任务返回 None，而不是错误
        assert!(cfg.task::<generated::Commission>("Nope").unwrap().is_none());
    }
}

#[cfg(test)]
mod next_run_tests {
    use super::*;

    #[test]
    fn 设置与读取() {
        let mut c = Config::from_json("{}").unwrap();
        assert_eq!(c.next_run("Commission"), None);
        c.set_next_run("Commission", Some("2026-11-01 12:00:00".into()));
        assert_eq!(c.next_run("Commission"), Some("2026-11-01 12:00:00"));
    }

    #[test]
    fn 传none表示清除() {
        let mut c = Config::from_json(r#"{"Commission":{"next_run":"x"}}"#).unwrap();
        c.set_next_run("Commission", None);
        assert_eq!(c.next_run("Commission"), None);
    }

    #[test]
    fn 保存后能重新读回() {
        let dir = std::env::temp_dir().join("maazure_cfg_test");
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("alas.json");
        let mut c = Config::from_json(r#"{"Commission":{"next_run":"2026-11-01 12:00:00"}}"#).unwrap();
        c.save(&p).unwrap();
        let again = Config::load(&p).unwrap();
        assert_eq!(again.next_run("Commission"), Some("2026-11-01 12:00:00"));
        let _ = std::fs::remove_file(&p);
        let mut c2 = Config::from_json("{}").unwrap();
        c2.set_next_run("Daily", Some("2026-11-01 13:00:00".into()));
        assert_eq!(c2.next_run("Daily"), Some("2026-11-01 13:00:00"));
        assert_eq!(c2.next_run("Commission"), None);
    }
}
