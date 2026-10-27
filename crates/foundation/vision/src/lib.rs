//! 地基 · 识别后处理：数字纠错、时长解析、几何变换。
//!
//! **位置**：`1 地基` 下与 `core` 平级的叶子。
//! 真正的 OCR / 模板匹配由 MaaFramework 内核完成；本 crate 只处理内核给不出的部分。
//! 上级：`docs/4-migration/4.2-assets.md`（哪些识别要保留后处理）。

#![deny(missing_docs)]

/// 把 OCR 原始文本纠正为纯数字。
///
/// 移植自旧版 `Digit` 后处理：`I/l/|`→`1`、`O/D/Q`→`0`、`S`→`5`、`B`→`8`。
///
/// **前置条件：至少有一个真 ASCII 数字**，否则返回 `None`。
/// 否则 `"Boss"` 会被映射成 `88`——旧版只在已知数字区域内调用，不需要这层保护，
/// 这里作为通用函数必须自己防。
///
/// # Examples
///
/// ```
/// use maazure_vision::normalize_digits;
/// assert_eq!(normalize_digits("O1S"), Some(15));
/// assert_eq!(normalize_digits("1,2O0"), Some(1200));
/// assert_eq!(normalize_digits("Boss"), None);
/// ```
pub fn normalize_digits(text: &str) -> Option<u64> {
    map_digits(text, true)
}

/// 把 `HH:MM:SS` 形式的时长解析为秒数。
///
/// 缺段时按「只有分钟」处理（游戏里 `05:30` 表示 5 分 30 秒）。
/// 每段用宽松映射，因此 `"O8:OO:OO"` 能解析成 8 小时。
pub fn parse_duration_secs(text: &str) -> Option<u64> {
    let nums: Vec<u64> = text
        .split(':')
        .map(|p| map_digits(p, false))
        .collect::<Option<Vec<_>>>()?;

    let (h, m, s) = match nums.as_slice() {
        [h, m, s] => (*h, *m, *s),
        [m, s] => (0, *m, *s),
        [s] => (0, 0, *s),
        _ => return None,
    };
    Some(h * 3600 + m * 60 + s)
}

/// 宽松版映射：`require_any_digit = false` 时允许整段都是字母误识。
fn map_digits(text: &str, require_any_digit: bool) -> Option<u64> {
    if require_any_digit && !text.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }

    let mut digits = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '0'..='9' => digits.push(ch),
            'O' | 'o' | 'D' | 'Q' => digits.push('0'),
            'I' | 'l' | '|' | 'i' => digits.push('1'),
            'Z' | 'z' => digits.push('2'),
            'S' | 's' => digits.push('5'),
            'B' | 'b' => digits.push('8'),
            'g' | 'q' => digits.push('9'),
            _ => continue,
        }
    }

    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_confusions() {
        assert_eq!(normalize_digits("O1S"), Some(15));
        assert_eq!(normalize_digits("B0"), Some(80));
        assert_eq!(normalize_digits("l2"), Some(12));
        assert_eq!(normalize_digits("1,2O0"), Some(1200));
    }

    #[test]
    fn words_are_not_digits() {
        assert_eq!(normalize_digits("Boss"), None);
        assert_eq!(normalize_digits("Oil"), None);
        assert_eq!(normalize_digits("--"), None);
    }

    #[test]
    fn durations() {
        assert_eq!(parse_duration_secs("08:00:00"), Some(28_800));
        assert_eq!(parse_duration_secs("01:20:00"), Some(4_800));
        assert_eq!(parse_duration_secs("05:30"), Some(330));
        assert_eq!(parse_duration_secs("O8:OO:OO"), Some(28_800));
        assert_eq!(parse_duration_secs("--:--"), None);
    }
}
