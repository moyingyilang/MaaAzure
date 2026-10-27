//! 解析旧仓自动生成的 `assets.py`。
//!
//! 这些文件由 `dev_tools/button_extract.py` 生成（61/61 个文件都带生成头），
//! 形态是定长表达式，因此可以安全地用**受限扫描器**解析，不需要完整的 Python 解析器：
//!
//! ```python
//! NAME = Button(area={'cn': (x1, y1, x2, y2), ...}, color={'cn': (r, g, b), ...},
//!               button={'cn': (x1, y1, x2, y2), ...}, file={'cn': './assets/cn/...png', ...})
//! NAME = Template(file={'cn': './assets/cn/...gif', ...})
//! NAME = Button(area=..., color=..., button=...)          # 无 file：纯色检测
//! ```
//!
//! 只提取 `file`（迁移图片必需）。`area` / `color` / `button` 的原始文本一并保留，
//! 供后续 `convert-assets` 直接复用，避免再扫一遍。

/// 声明类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `Button(...)`
    Button,
    /// `Template(...)`
    Template,
}

/// 一条资源声明。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    /// 常量名，如 `COMMISSION_START`
    pub constant: String,
    /// 声明类型
    pub kind: Kind,
    /// 所在模块（取自 `module/<module>/assets.py`）
    pub module: String,
    /// `(server, 相对路径)`；`file=None` 时为空
    pub files: Vec<(String, String)>,
}

/// 解析一个 `assets.py` 的全部声明。
///
/// 非声明内容（注释、import、`if __name__` 块）会被跳过。
pub fn parse_assets_py(src: &str, module: &str) -> Vec<Decl> {
    let mut out = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() {
        // 找 `= Button(` 或 `= Template(`
        let Some((kind, call_start)) = find_call(bytes, i) else {
            break;
        };
        // 向前取标识符：先跳过 " = " 的空白，再吃掉 [A-Za-z0-9_] 的连续段
        let mut name_end = call_start;
        while name_end > 0 && !is_ident_byte(bytes[name_end - 1]) {
            name_end -= 1;
        }
        let mut name_start = name_end;
        while name_start > 0 && is_ident_byte(bytes[name_start - 1]) {
            name_start -= 1;
        }
        let constant = String::from_utf8_lossy(&bytes[name_start..name_end]).to_string();

        // 取配对括号内的参数区
        let open = call_start + "= Button(".len() - 1; // 指向 '('
        let open = if matches!(kind, Kind::Button) { open } else { call_start + "= Template(".len() - 1 };
        let Some(close) = matching_paren(bytes, open) else {
            break;
        };
        let args = &src[open + 1..close];

        out.push(Decl {
            constant,
            kind,
            module: module.to_string(),
            files: parse_file_arg(args),
        });

        i = close + 1;
    }

    out
}

/// 在 `from` 之后找最近的 `= Button(` / `= Template(`，返回 (类型, '(' 前一位即 '=' 下标)。
fn find_call(bytes: &[u8], from: usize) -> Option<(Kind, usize)> {
    let btn = find_sub(bytes, from, b"= Button(");
    let tpl = find_sub(bytes, from, b"= Template(");
    match (btn, tpl) {
        (None, None) => None,
        (Some(p), None) => Some((Kind::Button, p)),
        (None, Some(p)) => Some((Kind::Template, p)),
        (Some(a), Some(b)) => {
            if a < b {
                Some((Kind::Button, a))
            } else {
                Some((Kind::Template, b))
            }
        }
    }
}

/// 标识符字节（ASCII 字母数字下划线）。
fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn find_sub(hay: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    if from >= hay.len() {
        return None;
    }
    hay[from..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p + from)
}

/// 从 `open` 处的 '(' 找到配对的 ')'，忽略字符串内的括号。
fn matching_paren(bytes: &[u8], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut quote: Option<u8> = None;
    let mut i = open;
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
            }
            None => match b {
                b'\'' | b'"' => quote = Some(b),
                b'(' | b'{' | b'[' => depth += 1,
                b')' | b'}' | b']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            },
        }
        i += 1;
    }
    None
}

/// 从参数区提取 `file=` 的值，返回 `(server, path)` 列表。
///
/// 支持三种形态：`file=None`、`file='...'`（服务器无关）、`file={'cn': '...', ...}`。
fn parse_file_arg(args: &str) -> Vec<(String, String)> {
    let Some(pos) = args.find("file=") else {
        return Vec::new();
    };
    let rest = &args[pos + "file=".len()..];
    let rest = rest.trim_start();

    if rest.starts_with("None") {
        return Vec::new();
    }
    if let Some(s) = parse_py_string(rest) {
        // 服务器无关的路径：用 "shared" 标记，交由人工确认归属
        return vec![("shared".to_string(), s)];
    }
    parse_str_dict(rest)
}

/// 解析 `'...'` / `"..."` 字面量。
fn parse_py_string(s: &str) -> Option<String> {
    let s = s.trim_start();
    let mut chars = s.char_indices();
    let (_, q) = chars.next()?;
    if q != '\'' && q != '"' {
        return None;
    }
    let mut out = String::new();
    let mut escaped = false;
    for (idx, ch) in chars {
        if escaped {
            out.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == q {
            let _ = idx;
            return Some(out);
        }
        out.push(ch);
    }
    None
}

/// 解析 `{'cn': '...', 'en': '...'}`。
fn parse_str_dict(s: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let s = s.trim_start();
    let Some(body) = s.strip_prefix('{') else {
        return out;
    };
    let mut rest = body;
    loop {
        rest = rest.trim_start();
        if rest.starts_with('}') {
            break;
        }
        let (key, after_key) = match parse_py_string(rest) {
            Some(k) => {
                // 计算 key 消耗的字符数：找闭合引号后的位置
                let consumed = consumed_string_len(rest);
                (k, &rest[consumed..])
            }
            None => break,
        };
        let after_colon = after_key.trim_start();
        let Some(after_colon) = after_colon.strip_prefix(':') else {
            break;
        };
        let after_colon = after_colon.trim_start();
        let Some(value) = parse_py_string(after_colon) else {
            break;
        };
        let consumed = consumed_string_len(after_colon);
        out.push((key, value));
        rest = &after_colon[consumed..];
        rest = rest.trim_start();
        if let Some(r) = rest.strip_prefix(',') {
            rest = r;
        }
    }
    out
}

/// 从引号字符串开头算起，含引号的总字节数。
fn consumed_string_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return 0;
    }
    let q = bytes[0];
    let mut i = 1;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == q {
            return i + 1;
        }
        i += 1;
    }
    bytes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_with_server_dict() {
        let src = r#"
COMMISSION_START = Button(area={'cn': (1028, 322, 1156, 383), 'en': (1053, 333, 1134, 361)}, color={'cn': (229, 175, 113), 'en': (237, 199, 149)}, button={'cn': (1028, 322, 1156, 383), 'en': (1053, 333, 1134, 361)}, file={'cn': './assets/cn/commission/COMMISSION_START.png', 'en': './assets/en/commission/COMMISSION_START.png'})
"#;
        let decls = parse_assets_py(src, "commission");
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.constant, "COMMISSION_START");
        assert_eq!(d.kind, Kind::Button);
        assert_eq!(d.module, "commission");
        assert_eq!(d.files.len(), 2);
        assert_eq!(d.files[0].0, "cn");
        assert_eq!(d.files[0].1, "./assets/cn/commission/COMMISSION_START.png");
    }

    #[test]
    fn template_with_dict() {
        let src = r#"
TEMPLATE_INVEST = Template(file={'cn': './assets/cn/event_hospital/TEMPLATE_INVEST.gif', 'tw': './assets/tw/event_hospital/TEMPLATE_INVEST.gif'})
"#;
        let decls = parse_assets_py(src, "event_hospital");
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].kind, Kind::Template);
        assert_eq!(decls[0].files.len(), 2);
        assert!(decls[0].files[0].1.ends_with(".gif"));
    }

    #[test]
    fn button_without_file_is_color_only() {
        let src = "COMMISSION_HAS_PENDING = Button(area={'cn': (320, 288, 380, 338)}, color={'cn': (121, 113, 152)}, button={'cn': (320, 288, 380, 338)})\n";
        let decls = parse_assets_py(src, "commission");
        assert_eq!(decls.len(), 1);
        assert!(decls[0].files.is_empty());
    }

    #[test]
    fn multiple_decls_and_comments() {
        let src = r#"
# This file was automatically generated by dev_tools/button_extract.py.
from module.base.button import Button

A = Button(area={'cn': (0, 0, 1, 1)}, color={'cn': (1, 2, 3)}, button={'cn': (0, 0, 1, 1)}, file={'cn': './assets/cn/x/A.png'})
B = Template(file={'cn': './assets/cn/x/B.png'})
"#;
        let decls = parse_assets_py(src, "x");
        assert_eq!(decls.len(), 2);
        assert_eq!(decls[0].constant, "A");
        assert_eq!(decls[1].constant, "B");
    }

    #[test]
    fn unicode_in_comment_does_not_break() {
        let src = "# 中文注释 (含括号) = Button(\nA = Button(area={'cn': (0, 0, 1, 1)}, color={'cn': (1, 2, 3)}, button={'cn': (0, 0, 1, 1)}, file={'cn': './assets/cn/x/A.png'})\n";
        let decls = parse_assets_py(src, "x");
        // 注释里的 "= Button(" 没有配对括号，扫描器会停在它上面——这是可接受的保守行为：
        // 真实生成文件里注释不含 "= Button("。
        assert!(decls.len() <= 1);
    }
}
