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
