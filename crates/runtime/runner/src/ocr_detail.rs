//! OCR 详情的解析工具（纯函数，不依赖 MaaFramework）。
//!
//! 单独成模块的原因：MaaFW 的 OCR `detail` JSON 形态在不同版本间变过
//! （`text` / `best.text` / `results[].text` / `all[].text`），
//! 这段「猜形状」的逻辑必须能被单测覆盖，而不是只能等真机跑通才发现挖不到文本。

/// 从 OCR 的 detail JSON 里挖出第一段非空文本。
///
/// 按可能性依次尝试；全都挖不到返回 `None`（调用方按「没读到」处理）。
pub fn extract_text(detail: &serde_json::Value) -> Option<String> {
    fn non_empty(v: Option<&serde_json::Value>) -> Option<String> {
        let s = v?.as_str()?;
        (!s.is_empty()).then(|| s.to_string())
    }

    non_empty(detail.get("text"))
        .or_else(|| non_empty(detail.get("best").and_then(|b| b.get("text"))))
        .or_else(|| {
            for key in ["results", "all"] {
                if let Some(arr) = detail.get(key).and_then(|v| v.as_array()) {
                    for item in arr {
                        if let Some(t) = non_empty(item.get("text")) {
                            return Some(t);
                        }
                    }
                }
            }
            None
        })
}

/// 解析 `{"slots": [[x1,y1,x2,y2], ...]}`；空参数返回 `None`（调用方用默认槽位）。
pub fn parse_slots(param: &str) -> Option<Vec<[i32; 4]>> {
    if param.trim().is_empty() {
        return None;
    }
    let v: serde_json::Value = serde_json::from_str(param).ok()?;
    let arr = v.get("slots")?.as_array()?;
    let mut out = Vec::new();
    for item in arr {
        let a = item.as_array()?;
        if a.len() != 4 {
            return None;
        }
        let mut r = [0i32; 4];
        for (i, x) in a.iter().enumerate() {
            r[i] = x.as_i64()? as i32;
        }
        out.push(r);
    }
    (!out.is_empty()).then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn text_at_top_level() {
        assert_eq!(
            extract_text(&json!({"text": "08:00:00"})).as_deref(),
            Some("08:00:00")
        );
    }

    #[test]
    fn text_in_best() {
        assert_eq!(
            extract_text(&json!({"best": {"text": "01:20:00"}})).as_deref(),
            Some("01:20:00")
        );
    }

    #[test]
    fn text_in_results_array() {
        let d = json!({"results": [{"text": ""}, {"text": "04:10:00"}]});
        assert_eq!(extract_text(&d).as_deref(), Some("04:10:00"));
    }

    #[test]
    fn text_in_all_array() {
        let d = json!({"all": [{"text": "02:30:00"}]});
        assert_eq!(extract_text(&d).as_deref(), Some("02:30:00"));
    }

    #[test]
    fn no_text_is_none() {
        assert_eq!(extract_text(&json!({})), None);
        assert_eq!(extract_text(&json!({"results": []})), None);
        assert_eq!(extract_text(&json!({"text": 123})), None);
    }

    #[test]
    fn slots_parsing() {
        let p = r#"{"slots": [[300,180,980,260],[300,270,980,350]]}"#;
        let s = parse_slots(p).unwrap();
        assert_eq!(s.len(), 2);
        assert_eq!(s[1], [300, 270, 980, 350]);

        assert_eq!(parse_slots(""), None);
        assert_eq!(parse_slots("{}"), None);
        assert_eq!(parse_slots(r#"{"slots": []}"#), None);
        assert_eq!(parse_slots(r#"{"slots": [[1,2,3]]}"#), None);
        assert_eq!(parse_slots(r#"{"slots": [["a",2,3,4]]}"#), None);
    }
}
