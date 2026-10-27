//! 解析旧仓 `campaign/**/*.py` 的 `MAP.*` 字面量。
//!
//! 这些文件是「数据即代码」：`MAP.shape = 'K8'`、`MAP.map_data = """ ... """`。
//! 只有字面量，没有控制流，所以可以安全地用受限扫描器提取；遇到无法静态求值的
//! 表达式就报错并列清单，**不猜**（`docs/4-migration/4.3-campaign-data.md` §提取策略）。
//!
//! 支持：
//!
//! ```python
//! MAP = CampaignMap('12-4')          → chapter
//! MAP.shape = 'K8'                   → 字符串
//! MAP.map_data = """..."""           → 三引号文本
//! MAP.camera_data = ['D2', 'D6']     → 字符串列表
//! MAP.spawn_data = [{'battle': 0}]   → 字典列表
//! MAP.flatten()                      → flags
//! ```

use std::collections::BTreeMap;

use serde_json::{Map, Value};

/// 一个战役文件解析结果。
#[derive(Debug, Default)]
pub struct CampaignFile {
    /// `CampaignMap('12-4')` 里的章节名
    pub chapter: Option<String>,
    /// `from ... import Config` 的继承来源
    pub inherits: Option<String>,
    /// `MAP.<attr> = <literal>`
    pub fields: BTreeMap<String, Value>,
    /// `MAP.<attr>(...)` 调用
    pub flags: Vec<String>,
    /// 无法静态求值的赋值：`(属性, 原始片段)`
    pub unresolved: Vec<(String, String)>,
}

impl CampaignFile {
    /// 取文本字段（`map_data` / `weight_data` 等）。
    pub fn text(&self, key: &str) -> Option<&str> {
        self.fields.get(key).and_then(Value::as_str)
    }
}

/// 解析一个战役文件的源码。
pub fn parse_campaign_py(src: &str) -> Result<CampaignFile, String> {
    let mut out = CampaignFile::default();

    out.chapter = find_chapter(src);
    out.inherits = find_inherits(src);

    let bytes = src.as_bytes();
    let mut i = 0usize;

    while let Some(pos) = find_map_dot(bytes, i) {
        // pos 指向 "MAP." 之后
        i = pos;
        let Some((attr, after_attr)) = take_ident(bytes, pos) else {
            continue;
        };

        // 跳过空白（不跨行：`MAP.x` 与 `=` 一般同行）
        let mut k = after_attr;
        while k < bytes.len() && (bytes[k] == b' ' || bytes[k] == b'\t') {
            k += 1;
        }

        match bytes.get(k) {
            Some(b'(') => {
                // MAP.flatten() 之类的调用
                if !out.flags.contains(&attr) {
                    out.flags.push(attr);
                }
                i = k + 1;
            }
            Some(b'=') => {
                let vstart = k + 1;
                match parse_value(src, vstart) {
                    Some((value, vend)) => {
                        out.fields.insert(attr, value);
                        i = vend;
                    }
                    None => {
                        // 记录无法解析的片段（到行尾），列进待人工清单
                        let line_end = src[vstart..]
                            .find('\n')
                            .map(|o| vstart + o)
                            .unwrap_or(src.len());
                        out.unresolved
                            .push((attr, src[vstart..line_end].trim().to_string()));
                        i = line_end;
                    }
                }
            }
            _ => continue,
        }
    }

    Ok(out)
}

/// 找 `CampaignMap('12-4')`。
fn find_chapter(src: &str) -> Option<String> {
    let pos = src.find("CampaignMap(")?;
    let rest = &src[pos + "CampaignMap(".len()..];
    let q = rest.find(['\'', '"'])?;
    let quote = rest.as_bytes()[q] as char;
    let tail = &rest[q + 1..];
    let end = tail.find(quote)?;
    Some(tail[..end].to_string())
}

/// 找 `from campaign.campaign_main.campaign_12_1 import Config`。
fn find_inherits(src: &str) -> Option<String> {
    for line in src.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("from ") {
            if rest.ends_with("import Config") {
                return Some(rest.trim_end_matches(" import Config").to_string());
            }
        }
    }
    None
}

/// 找 `MAP.`，返回其后一位；跳过字符串与注释，避免误判。
fn find_map_dot(bytes: &[u8], from: usize) -> Option<usize> {
    let mut i = from;
    let mut quote: Option<u8> = None;
    while i < bytes.len() {
        let b = bytes[i];
        match quote {
            Some(q) => {
                if b == b'\\' {
                    i += 2;
                    continue;
                }
                if b == q {
                    quote = None;
                }
                i += 1;
            }
            None => {
                if b == b'#' {
                    // 跳到行尾
                    while i < bytes.len() && bytes[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                }
                if b == b'\'' || b == b'"' {
                    // 三引号也要吃掉
                    if i + 2 < bytes.len() && bytes[i + 1] == b && bytes[i + 2] == b {
                        i += 3;
                        while i + 2 < bytes.len()
                            && !(bytes[i] == b && bytes[i + 1] == b && bytes[i + 2] == b)
                        {
                            i += 1;
                        }
                        i += 3;
                        continue;
                    }
                    quote = Some(b);
                    i += 1;
                    continue;
                }
                if bytes[i..].starts_with(b"MAP.") {
                    return Some(i + 4);
                }
                i += 1;
            }
        }
    }
    None
}

/// 取标识符，返回 (名字, 结束位置)。
fn take_ident(bytes: &[u8], from: usize) -> Option<(String, usize)> {
    let mut i = from;
    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
        i += 1;
    }
    if i == from {
        return None;
    }
    Some((String::from_utf8_lossy(&bytes[from..i]).to_string(), i))
}

/// 解析一个字面量，返回 (JSON 值, 结束位置)。
fn parse_value(src: &str, start: usize) -> Option<(Value, usize)> {
    let bytes = src.as_bytes();
    let mut i = start;
    while i < bytes.len() && (bytes[i] as char).is_whitespace() {
        i += 1;
    }
    let b = *bytes.get(i)?;

    match b {
        b'\'' | b'"' => parse_string(src, i),
        b'[' => parse_list(src, i),
        b'(' => parse_list(src, i),
        b'-' | b'0'..=b'9' => parse_number(src, i),
        b'T' | b'F' => parse_bool(src, i),
        _ => None,
    }
}

fn parse_string(src: &str, start: usize) -> Option<(Value, usize)> {
    let bytes = src.as_bytes();
    let q = bytes[start];
    let triple = start + 2 < bytes.len() && bytes[start + 1] == q && bytes[start + 2] == q;
    let (body_start, close) = if triple {
        (start + 3, [q, q, q])
    } else {
        (start + 1, [q, 0, 0])
    };

    let mut i = body_start;
    while i < bytes.len() {
        if triple {
            if i + 2 < bytes.len() && bytes[i] == close[0] && bytes[i + 1] == close[0] && bytes[i + 2] == close[0] {
                return Some((Value::String(src[body_start..i].to_string()), i + 3));
            }
        } else if bytes[i] == q {
            return Some((Value::String(src[body_start..i].to_string()), i + 1));
        } else if bytes[i] == b'\\' {
            i += 1;
        }
        i += 1;
    }
    None
}

fn parse_list(src: &str, start: usize) -> Option<(Value, usize)> {
    let bytes = src.as_bytes();
    let close = if bytes[start] == b'(' { b')' } else { b']' };
    let mut i = start + 1;
    let mut items: Vec<Value> = Vec::new();
    loop {
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        match bytes.get(i)? {
            b if *b == close => return Some((Value::Array(items), i + 1)),
            b',' => {
                i += 1;
            }
            b'{' => {
                let (v, next) = parse_dict(src, i)?;
                items.push(v);
                i = next;
            }
            _ => {
                let (v, next) = parse_value(src, i)?;
                items.push(v);
                i = next;
            }
        }
    }
}

fn parse_dict(src: &str, start: usize) -> Option<(Value, usize)> {
    let bytes = src.as_bytes();
    let mut i = start + 1;
    let mut obj = Map::new();
    loop {
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        match bytes.get(i)? {
            b'}' => return Some((Value::Object(obj), i + 1)),
            b',' => {
                i += 1;
                continue;
            }
            _ => {
                let (k, next) = parse_string(src, i)?;
                let key = k.as_str()?.to_string();
                i = next;
                while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                    i += 1;
                }
                if bytes.get(i)? != &b':' {
                    return None;
                }
                let (v, next) = parse_value(src, i + 1)?;
                obj.insert(key, v);
                i = next;
            }
        }
    }
}

fn parse_number(src: &str, start: usize) -> Option<(Value, usize)> {
    let bytes = src.as_bytes();
    let mut i = start;
    if bytes[i] == b'-' {
        i += 1;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    let text = &src[start..i];
    let n: i64 = text.parse().ok()?;
    Some((Value::from(n), i))
}

fn parse_bool(src: &str, start: usize) -> Option<(Value, usize)> {
    let rest = &src[start..];
    if rest.starts_with("True") {
        Some((Value::Bool(true), start + 4))
    } else if rest.starts_with("False") {
        Some((Value::Bool(false), start + 5))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_typical_map() {
        let src = r#"
from campaign.campaign_main.campaign_12_1 import Config
from module.campaign.campaign_base import CampaignBase
from module.map.map_base import CampaignMap

MAP = CampaignMap('12-4')
MAP.shape = 'K8'
MAP.camera_data = ['D2', 'D6', 'H2', 'H6']
MAP.map_data = """
    MB ME -- ++
    -- SP ME ++
"""
MAP.spawn_data = [
    {'battle': 0, 'enemy': 2},
    {'battle': 1, 'enemy': 1, 'boss': 1},
]
MAP.flatten()
"#;
        let f = parse_campaign_py(src).unwrap();
        assert_eq!(f.chapter.as_deref(), Some("12-4"));
        assert_eq!(
            f.inherits.as_deref(),
            Some("campaign.campaign_main.campaign_12_1")
        );
        assert_eq!(f.text("shape"), Some("K8"));
        assert!(f.text("map_data").unwrap().contains("MB ME -- ++"));
        assert_eq!(
            f.fields["camera_data"],
            serde_json::json!(["D2", "D6", "H2", "H6"])
        );
        let spawn = f.fields["spawn_data"].as_array().unwrap();
        assert_eq!(spawn.len(), 2);
        assert_eq!(spawn[1]["boss"], 1);
        assert_eq!(f.flags, vec!["flatten".to_string()]);
        assert!(f.unresolved.is_empty());
    }

    #[test]
    fn unresolvable_expression_is_reported_not_guessed() {
        let src = "MAP.shape = get_shape()\nMAP.map_data = \"\"\"- -\"\"\"\n";
        let f = parse_campaign_py(src).unwrap();
        assert!(f.fields.contains_key("map_data"));
        assert_eq!(f.unresolved.len(), 1);
        assert_eq!(f.unresolved[0].0, "shape");
    }

    #[test]
    fn ignores_map_dot_inside_strings_and_comments() {
        let src = r#"
# MAP.fake = 'x'
MAP.map_data = """
    note: MAP.y is not code
"""
MAP.shape = 'A1'
"#;
        let f = parse_campaign_py(src).unwrap();
        assert_eq!(f.text("shape"), Some("A1"));
        assert!(!f.fields.contains_key("fake"));
        assert!(!f.fields.contains_key("y"));
    }
}
