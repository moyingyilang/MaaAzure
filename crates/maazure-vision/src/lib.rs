//! 自定义识别用的图像后处理工具。
//!
//! 本 crate 负责「识别结果的清洗与几何计算」，**不负责截图、不负责点击、不依赖 MaaFramework**。
//! 真正的 OCR / 模板匹配由 MaaFramework 内核完成，这里只处理它给不出的东西：
//! 数字纠错、网格建模、单应性变换。
//!
//! 设计依据：`docs/A4-rust-design.md`、`docs/A3-resource-migration.md`。

#![deny(missing_docs)]

/// 把 OCR 原始文本纠正为纯数字。
///
/// 移植自旧版 `module/ocr/ocr.py` 的 `Digit` 后处理规则：
/// 游戏字体下 `I`/`l`/`|` 常被识别成 `1`，`D`/`O`/`o` 成 `0`，`S` 成 `5`，`B` 成 `8`。
///
/// **前置条件：文本里必须至少有一个真正的 ASCII 数字**，否则返回 `None`。
/// 这条约束是为了避免把普通单词误当成数字（例如 `"Boss"` 里的 `B` 会变成 `8`，
/// 若不设前置条件就会得到 `88`）。旧版只在明确的数字区域内调用，因此不需要这层保护；
/// 新版是通用函数，必须自己防。
///
/// # Examples
///
/// ```
/// use maazure_vision::normalize_digits;
/// assert_eq!(normalize_digits("O1S"), Some(15));
/// assert_eq!(normalize_digits("1,2O0"), Some(1200));
/// assert_eq!(normalize_digits("--"), None);
/// // 无真数字的单词不会被误判：Boss 里的 B 不会变成 8
/// assert_eq!(normalize_digits("Boss"), None);
/// ```
pub fn normalize_digits(text: &str) -> Option<u64> {
    // 前置条件：至少有一个真数字，否则整段文本不当作数字处理
    map_digits(text, true)
}

/// 宽松版：允许整段都是「字母误识」。
///
/// 用于 `Duration` 这类**已知位置必为数字**的场景：`"OO"` 应被解析成 `0`，
/// 若用 [`normalize_digits`] 的严格规则会因缺少真数字而被拒绝。
fn map_digits(text: &str, require_any_digit: bool) -> Option<u64> {
    if require_any_digit && !text.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }

    let mut digits = String::with_capacity(text.len());

    for ch in text.chars() {
        let mapped = match ch {
            '0'..='9' => ch,
            // 常见误识
            'O' | 'o' | 'D' | 'Q' => '0',
            'I' | 'l' | '|' | 'i' => '1',
            'Z' | 'z' => '2',
            'S' | 's' => '5',
            'B' | 'b' => '8',
            'g' | 'q' => '9',
            // 千分位与空白直接丢弃
            ',' | ' ' | '\u{00a0}' => continue,
            _ => continue,
        };
        digits.push(mapped);
    }

    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

/// 把 `HH:MM:SS` 形式的时长解析为秒数。
///
/// 移植自旧版 `module/ocr/ocr.py` 的 `Duration`：
/// OCR 结果先过 [`normalize_digits`]，再按冒号分三段。
/// 缺段时按「只有分钟」处理（游戏里 `05:00` 表示 5 分钟）。
pub fn parse_duration_secs(text: &str) -> Option<u64> {
    let parts: Vec<&str> = text.split(':').collect();
    let nums: Vec<u64> = parts
        .iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_common_confusions() {
        assert_eq!(normalize_digits("O1S"), Some(15));
        assert_eq!(normalize_digits("B0"), Some(80));
        assert_eq!(normalize_digits("l2"), Some(12));
        assert_eq!(normalize_digits("1,2O0"), Some(1200));
    }

    #[test]
    fn digits_empty() {
        assert_eq!(normalize_digits(""), None);
        assert_eq!(normalize_digits("--"), None);
    }

    #[test]
    fn words_are_not_digits() {
        // 无真数字时整段不做映射，避免 "Boss" → 88 这类误判
        assert_eq!(normalize_digits("abc"), None);
        assert_eq!(normalize_digits("Boss"), None);
        assert_eq!(normalize_digits("Oil"), None);
    }

    #[test]
    fn duration_forms() {
        assert_eq!(parse_duration_secs("08:00:00"), Some(8 * 3600));
        assert_eq!(parse_duration_secs("01:20:00"), Some(4800));
        assert_eq!(parse_duration_secs("05:30"), Some(330));
        assert_eq!(parse_duration_secs("O8:O0:OO"), Some(8 * 3600));
        assert_eq!(parse_duration_secs("--:--"), None);
    }
}
