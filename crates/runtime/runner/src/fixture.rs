//! 合成帧脚本：**不用录像的回放测试**。
//!
//! 真机语料要等设备，但「Pipeline 会不会按预期跳转」这件事可以先用合成帧断言。
//! 脚本描述每一帧显示哪些节点，跑完检查点击数、是否走过某帧、最终结果。
//!
//! ```jsonc
//! // data/fixtures/commission_happy_path.json
//! {
//!   "name": "commission_happy_path",
//!   "entry": "CommissionStart",
//!   "frames": [
//!     { "show": ["commission_goto_reward"] },
//!     { "show": ["commission_page"] }
//!   ],
//!   "expect": { "min_clicks": 2, "outcome": "ok" }
//! }
//! ```
//!
//! 与 golden corpus 的分工：
//!
//! | | 合成帧脚本 | golden corpus |
//! |---|---|---|
//! | 输入 | 模板贴图（我们造的） | 真实截图（录的） |
//! | 验证 | 流程跳转、坐标、Custom 是否被调到 | 真实画面下的识别与决策 |
//! | 需要设备 | 否 | 是（录制时） |

use serde::Deserialize;

/// 一帧的合成说明。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FrameSpec {
    /// 这一帧要贴哪些节点
    #[serde(default)]
    pub show: Vec<String>,
}

/// 期望结果。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Expect {
    /// 最少点击次数
    #[serde(default)]
    pub min_clicks: usize,
    /// 最多点击次数（0 表示不检查）
    #[serde(default)]
    pub max_clicks: usize,
    /// 期望的结束状态：`ok` / `failed` / `recoverable`
    #[serde(default)]
    pub outcome: Option<String>,
    /// 必须执行过的节点（顺序无关）
    #[serde(default)]
    pub visited: Vec<String>,
    /// 不允许执行到的节点
    #[serde(default)]
    pub not_visited: Vec<String>,
}

/// 一个合成帧用例。
#[derive(Debug, Clone, Deserialize)]
pub struct Script {
    /// 用例名
    pub name: String,
    /// Pipeline 入口
    pub entry: String,
    /// 帧序列
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
    /// 服务器（默认 cn）
    #[serde(default = "default_server")]
    pub server: String,
    /// 期望
    #[serde(default)]
    pub expect: Expect,
}

fn default_server() -> String {
    "cn".to_string()
}

impl Script {
    /// 从文件加载。
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        let p = path.as_ref();
        let text =
            std::fs::read_to_string(p).map_err(|e| format!("读 {} 失败: {e}", p.display()))?;
        let s: Self =
            serde_json::from_str(&text).map_err(|e| format!("解析 {} 失败: {e}", p.display()))?;
        if s.frames.is_empty() {
            return Err(format!("{}: frames 不能为空", p.display()));
        }
        Ok(s)
    }

    /// 结果是否符合期望；返回 `Err(原因)` 表示不符合。
    pub fn check(&self, outcome: &str, clicks: usize, visited: &[String]) -> Result<(), String> {
        if clicks < self.expect.min_clicks {
            return Err(format!(
                "{}: 点击 {} 次，少于期望的 {}",
                self.name, clicks, self.expect.min_clicks
            ));
        }
        if self.expect.max_clicks > 0 && clicks > self.expect.max_clicks {
            return Err(format!(
                "{}: 点击 {} 次，多于期望的 {}",
                self.name, clicks, self.expect.max_clicks
            ));
        }
        if let Some(want) = &self.expect.outcome
            && !want.eq_ignore_ascii_case(outcome)
        {
            return Err(format!("{}: 结束状态 {outcome}，期望 {want}", self.name));
        }
        for want in &self.expect.visited {
            if !visited.iter().any(|v| v == want) {
                return Err(format!(
                    "{}: 没执行到节点 `{want}`（实际执行：{visited:?}）",
                    self.name
                ));
            }
        }
        for deny in &self.expect.not_visited {
            if visited.iter().any(|v| v == deny) {
                return Err(format!(
                    "{}: 不该执行到节点 `{deny}`（实际执行：{visited:?}）",
                    self.name
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn script(json: &str) -> Script {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn minimal_script() {
        let s = script(r#"{"name":"a","entry":"CommissionStart","frames":[{"show":["x"]}]}"#);
        assert_eq!(s.server, "cn");
        assert_eq!(s.frames.len(), 1);
        assert_eq!(s.expect.min_clicks, 0);
        assert!(s.expect.outcome.is_none());
    }

    #[test]
    fn checks_clicks_and_outcome() {
        let s = script(
            r#"{"name":"a","entry":"E","frames":[{}],
                "expect":{"min_clicks":1,"max_clicks":3,"outcome":"ok"}}"#,
        );
        assert!(s.check("ok", 2, &[]).is_ok());
        assert!(s.check("ok", 0, &[]).unwrap_err().contains("少于"));
        assert!(s.check("ok", 5, &[]).unwrap_err().contains("多于"));
        assert!(s.check("failed", 2, &[]).unwrap_err().contains("结束状态"));
    }

    #[test]
    fn checks_visited_path() {
        let s = script(
            r#"{"name":"a","entry":"E","frames":[{}],
                "expect":{"visited":["B","C"],"not_visited":["Z"]}}"#,
        );
        let visited = vec!["A".to_string(), "B".to_string()];
        // 缺 C
        assert!(
            s.check("ok", 0, &visited)
                .unwrap_err()
                .contains("没执行到节点 `C`")
        );

        let ok_path = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        assert!(s.check("ok", 0, &ok_path).is_ok());

        let bad_path = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "Z".to_string(),
        ];
        assert!(
            s.check("ok", 0, &bad_path)
                .unwrap_err()
                .contains("不该执行到节点 `Z`")
        );
    }

    #[test]
    fn max_clicks_zero_means_no_limit() {
        let s = script(r#"{"name":"a","entry":"E","frames":[{}],"expect":{"max_clicks":0}}"#);
        assert!(s.check("ok", 999, &[]).is_ok());
    }

    #[test]
    fn empty_frames_is_rejected_on_load() {
        let dir = std::env::temp_dir().join("maazure_fixture_test");
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("bad.json");
        std::fs::write(&p, r#"{"name":"a","entry":"E","frames":[]}"#).unwrap();
        assert!(Script::load(&p).unwrap_err().contains("frames 不能为空"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
