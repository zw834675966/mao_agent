use regex::Regex;
use std::sync::LazyLock;

static CJK_SPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Matches horizontal whitespace between CJK characters, excluding \r and \n
    // CJK range: \u4e00-\u9fff (Basic), \u3400-\u4dbf (Ext A), \uf900-\ufaff (Compatibility), fullwidth punctuation
    Regex::new(r"([\u4e00-\u9fa5\u3400-\u4dbf\uf900-\ufaff\u3000-\u303f\uff01-\uff5e])[ \t\u3000\u00a0]+([\u4e00-\u9fa5\u3400-\u4dbf\uf900-\ufaff\u3000-\u303f\uff01-\uff5e])").unwrap()
});

static DIGIT_CJK_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([0-9])[ \t]+([\u4e00-\u9fa5])").unwrap());

static CJK_DIGIT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([\u4e00-\u9fa5])[ \t]+([0-9])").unwrap());

static DIGIT_DIGIT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([0-9])[ \t]+([0-9])").unwrap());

static MULTI_NEWLINE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n{3,}").unwrap());

/// Cleans OCR artifact spaces in Chinese text while preserving newlines and markdown layout.
pub fn clean_cjk_spaces(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut result = text.replace(['\u{3000}', '\u{00a0}'], " ");

    // Clean spaced numbers (e.g. "1 9 4 9" -> "1949")
    for _ in 0..3 {
        let replaced = DIGIT_DIGIT_REGEX.replace_all(&result, "$1$2");
        if replaced == result {
            break;
        }
        result = replaced.to_string();
    }

    // Iterative replacement for overlapping CJK spaces (e.g. "毛 泽 东" -> "毛泽东")
    for _ in 0..3 {
        let replaced = CJK_SPACE_REGEX.replace_all(&result, "$1$2");
        if replaced == result {
            break;
        }
        result = replaced.to_string();
    }

    // Clean space between digits and Chinese characters in dates like "1949 年"
    result = DIGIT_CJK_REGEX.replace_all(&result, "$1$2").to_string();
    result = CJK_DIGIT_REGEX.replace_all(&result, "$1$2").to_string();

    // Normalize multiple excessive newlines
    result = MULTI_NEWLINE_REGEX.replace_all(&result, "\n\n").to_string();

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_cjk_spaces() {
        assert_eq!(clean_cjk_spaces("毛 泽 东 选 集"), "毛泽东选集");
        assert_eq!(clean_cjk_spaces("1 9 4 9 年 1 0 月"), "1949年10月");
    }

    #[test]
    fn test_preserve_newlines() {
        let input = "# 第一章 矛盾论\n\n事物的矛盾法则，即对立统一的法则。";
        let expected = "# 第一章矛盾论\n\n事物的矛盾法则，即对立统一的法则。";
        assert_eq!(clean_cjk_spaces(input), expected);
    }
}
