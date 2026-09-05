use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

static YEAR_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\b(18\d{2}|19\d{2}|20\d{2})\b").unwrap());

/// Historical periods representing different eras of modern Chinese history and Mao's writings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HistoricalPeriod {
    /// 早期文稿与建党前 (1893 - 1923)
    EarlyWritings,
    /// 第一次国内革命战争时期 / 大革命时期 (1924 - 1927)
    FirstRevolutionaryWar,
    /// 土地革命战争时期 / 苏区时期 (1927 - 1937)
    AgrarianRevolutionaryWar,
    /// 全民族抗日战争时期 (1937 - 1945)
    WarOfResistance,
    /// 全国解放战争时期 (1945 - 1949)
    WarOfLiberation,
    /// 社会主义革命与建设时期 (1949 - 1976)
    SocialistConstruction,
    /// 未知或未分类时期
    #[default]
    Unknown,
}

impl HistoricalPeriod {
    /// Parse Chinese period name or date into HistoricalPeriod.
    pub fn from_str_or_date(s: &str) -> Self {
        let trimmed = s.trim();
        if trimmed.contains("早期") || trimmed.contains("建党") || trimmed.contains("新民学会")
        {
            return HistoricalPeriod::EarlyWritings;
        }
        if trimmed.contains("第一次国内") || trimmed.contains("大革命") {
            return HistoricalPeriod::FirstRevolutionaryWar;
        }
        if trimmed.contains("土地革命")
            || trimmed.contains("井冈山")
            || trimmed.contains("中央苏区")
            || trimmed.contains("长征")
        {
            return HistoricalPeriod::AgrarianRevolutionaryWar;
        }
        if trimmed.contains("抗日") || trimmed.contains("抗战") || trimmed.contains("延安") {
            return HistoricalPeriod::WarOfResistance;
        }
        if trimmed.contains("解放战争")
            || trimmed.contains("西柏坡")
            || trimmed.contains("第三次国内")
        {
            return HistoricalPeriod::WarOfLiberation;
        }
        if trimmed.contains("社会主义") || trimmed.contains("建国") || trimmed.contains("新中国")
        {
            return HistoricalPeriod::SocialistConstruction;
        }

        // Try extracting year (e.g. 1938 -> WarOfResistance)
        if let Some(year) = extract_year(trimmed) {
            return match year {
                y if y < 1924 => HistoricalPeriod::EarlyWritings,
                1924..=1927 => HistoricalPeriod::FirstRevolutionaryWar,
                1928..=1936 => HistoricalPeriod::AgrarianRevolutionaryWar,
                1937..=1944 => HistoricalPeriod::WarOfResistance,
                1945..=1948 => HistoricalPeriod::WarOfLiberation,
                1949..=1976 => HistoricalPeriod::SocialistConstruction,
                _ => HistoricalPeriod::Unknown,
            };
        }

        HistoricalPeriod::Unknown
    }

    /// Display period in standard Chinese historical convention.
    pub fn as_str(&self) -> &'static str {
        match self {
            HistoricalPeriod::EarlyWritings => "早期文稿时期 (1893-1923)",
            HistoricalPeriod::FirstRevolutionaryWar => "第一次国内革命战争时期 (1924-1927)",
            HistoricalPeriod::AgrarianRevolutionaryWar => "土地革命战争时期 (1927-1937)",
            HistoricalPeriod::WarOfResistance => "抗日战争时期 (1937-1945)",
            HistoricalPeriod::WarOfLiberation => "全国解放战争时期 (1945-1949)",
            HistoricalPeriod::SocialistConstruction => "社会主义革命和建设时期 (1949-1976)",
            HistoricalPeriod::Unknown => "未分类时期",
        }
    }
}

impl fmt::Display for HistoricalPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn extract_year(s: &str) -> Option<i32> {
    if let Some(cap) = YEAR_REGEX.captures(s)
        && let Ok(y) = cap[1].parse::<i32>()
    {
        return Some(y);
    }
    None
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Parse a historical date into an inclusive closed interval `[start_date, end_date]` in ISO `YYYY-MM-DD`.
/// Handles "YYYY", "YYYY-MM", and "YYYY-MM-DD".
fn date_to_interval(date_str: &str) -> Option<(String, String)> {
    let trimmed = date_str.trim();
    if trimmed.is_empty() || trimmed == "未知" {
        return None;
    }

    let parts: Vec<&str> = trimmed.split('-').collect();
    let year: i32 = parts.first()?.parse().ok()?;

    match parts.len() {
        1 => {
            // "YYYY" -> [YYYY-01-01, YYYY-12-31]
            Some((format!("{year:04}-01-01"), format!("{year:04}-12-31")))
        }
        2 => {
            // "YYYY-MM" -> [YYYY-MM-01, YYYY-MM-last_day]
            let month: u32 = parts[1].parse().ok()?;
            if !(1..=12).contains(&month) {
                return None;
            }
            let last_day = days_in_month(year, month);
            Some((
                format!("{year:04}-{month:02}-01"),
                format!("{year:04}-{month:02}-{last_day:02}"),
            ))
        }
        3 => {
            // "YYYY-MM-DD" -> [YYYY-MM-DD, YYYY-MM-DD]
            let month: u32 = parts[1].parse().ok()?;
            let day: u32 = parts[2].parse().ok()?;
            if !(1..=12).contains(&month) {
                return None;
            }
            let max_day = days_in_month(year, month);
            let clamped_day = day.clamp(1, max_day);
            Some((
                format!("{year:04}-{month:02}-{clamped_day:02}"),
                format!("{year:04}-{month:02}-{clamped_day:02}"),
            ))
        }
        _ => None,
    }
}

/// Metadata extracted from document YAML frontmatter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub title: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default = "default_unknown")]
    pub date: String,
    #[serde(default)]
    pub period: String,
    #[serde(default)]
    pub volume: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub place: String,
    #[serde(default)]
    pub document_type: String,
    #[serde(default)]
    pub recipient: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

fn default_author() -> String {
    "毛泽东".to_string()
}

fn default_unknown() -> String {
    "未知".to_string()
}

/// A complete historical document with metadata and content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub metadata: DocumentMetadata,
    pub period_enum: HistoricalPeriod,
    pub headnote: Option<String>,
    pub content: String,
    pub footnotes: Vec<String>,
    pub file_path: Option<String>,
}

/// A semantic chunk ready for embedding, indexing, and citation grounding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub chunk_id: String,
    pub doc_id: String,
    pub doc_title: String,
    pub author: String,
    pub period: HistoricalPeriod,
    pub date: String,
    pub volume: String,
    pub category: String,
    pub tags: Vec<String>,
    pub chunk_index: usize,
    pub total_chunks: usize,
    pub char_count: usize,
    pub raw_text: String,
    pub contextualized_text: String,
    pub section_path: Vec<String>,
}

/// Stored entry in the vector database with embedding and metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorEntry {
    pub id: String,
    pub vector: Vec<f32>,
    pub chunk: DocumentChunk,
}

/// Multi-dimensional filter criteria for vector search.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VectorFilter {
    /// Specific historical period
    pub period: Option<HistoricalPeriod>,
    /// Multiple allowed periods
    pub periods: Option<Vec<HistoricalPeriod>>,
    /// Specific volume (e.g. "毛泽东选集第二卷", "文集第一卷")
    pub volume: Option<String>,
    /// Specific category (e.g. "军事", "哲学", "党建", "书信")
    pub category: Option<String>,
    /// Tags that must be present
    pub tags: Option<Vec<String>>,
    /// Minimum date (ISO format YYYY-MM-DD or YYYY)
    pub start_date: Option<String>,
    /// Maximum date (ISO format YYYY-MM-DD or YYYY)
    pub end_date: Option<String>,
    /// Specific document ID
    pub doc_id: Option<String>,
    /// Plaintext keyword filter that must appear in text
    pub keyword: Option<String>,
}

impl VectorFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_period(mut self, period: HistoricalPeriod) -> Self {
        self.period = Some(period);
        self
    }

    pub fn with_volume<S: Into<String>>(mut self, volume: S) -> Self {
        self.volume = Some(volume.into());
        self
    }

    pub fn with_category<S: Into<String>>(mut self, category: S) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn with_date_range<S: Into<String>>(mut self, start: S, end: S) -> Self {
        self.start_date = Some(start.into());
        self.end_date = Some(end.into());
        self
    }

    pub fn with_keyword<S: Into<String>>(mut self, keyword: S) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// Periods allowed by this filter.
    ///
    /// If both `period` and `periods` are set, **`periods` wins** (Scout Rule: one source of truth).
    pub fn effective_periods(&self) -> Option<Vec<HistoricalPeriod>> {
        if let Some(ref periods) = self.periods {
            if periods.is_empty() {
                return None;
            }
            return Some(periods.clone());
        }
        self.period.map(|p| vec![p])
    }

    pub fn with_periods(mut self, periods: Vec<HistoricalPeriod>) -> Self {
        self.periods = Some(periods);
        // Clear singular to avoid dual-field contradiction at construction.
        self.period = None;
        self
    }

    /// Normalize dual `period`/`periods` so only `periods` remains when both were set.
    pub fn normalize_periods(&mut self) {
        if let Some(ref periods) = self.periods
            && !periods.is_empty()
        {
            self.period = None;
        }
    }

    /// Check if a chunk satisfies this filter.
    pub fn matches(&self, chunk: &DocumentChunk) -> bool {
        if let Some(allowed) = self.effective_periods()
            && !allowed.contains(&chunk.period)
        {
            return false;
        }
        if let Some(ref vol) = self.volume
            && !chunk.volume.contains(vol)
            && !vol.contains(&chunk.volume)
        {
            return false;
        }
        if let Some(ref cat) = self.category
            && !chunk.category.contains(cat)
            && !cat.contains(&chunk.category)
        {
            return false;
        }
        if let Some(ref tags) = self.tags {
            for tag in tags {
                if !chunk
                    .tags
                    .iter()
                    .any(|t| t.contains(tag) || tag.contains(t))
                {
                    return false;
                }
            }
        }
        if self.start_date.is_some() || self.end_date.is_some() {
            // Unknown / empty dates must not silently pass interval filters.
            let date_trim = chunk.date.trim();
            if date_trim.is_empty() || date_trim == "未知" {
                return false;
            }
            if let Some((chunk_start, chunk_end)) = date_to_interval(&chunk.date) {
                if let Some(ref start) = self.start_date {
                    let filter_start = date_to_interval(start)
                        .map(|(s, _)| s)
                        .unwrap_or_else(|| start.clone());
                    if chunk_end < filter_start {
                        return false;
                    }
                }
                if let Some(ref end) = self.end_date {
                    let filter_end = date_to_interval(end)
                        .map(|(_, e)| e)
                        .unwrap_or_else(|| end.clone());
                    if chunk_start > filter_end {
                        return false;
                    }
                }
            } else {
                // Unparseable non-unknown dates: fail closed under interval filters.
                return false;
            }
        }
        if let Some(ref doc_id) = self.doc_id
            && chunk.doc_id != *doc_id
        {
            return false;
        }
        if let Some(ref kw) = self.keyword
            && !chunk.raw_text.contains(kw)
            && !chunk.contextualized_text.contains(kw)
            && !chunk.doc_title.contains(kw)
        {
            return false;
        }
        true
    }
}

/// Search result from vector similarity retrieval.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub chunk_id: String,
    pub score: f32,
    pub rank: usize,
    pub chunk: DocumentChunk,
}

/// Statistics and health metrics of the vector store.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VectorStoreStats {
    pub total_vectors: usize,
    pub total_documents: usize,
    pub vector_dimension: usize,
    pub period_distribution: HashMap<String, usize>,
    pub volume_distribution: HashMap<String, usize>,
    pub total_characters_indexed: usize,
    pub estimated_memory_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year_and_days_in_month() {
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1936));
        assert!(!is_leap_year(1937));
        assert!(!is_leap_year(1938));

        assert_eq!(days_in_month(1936, 2), 29);
        assert_eq!(days_in_month(1937, 2), 28);
        assert_eq!(days_in_month(1938, 5), 31);
        assert_eq!(days_in_month(1938, 4), 30);
    }

    #[test]
    fn test_date_to_interval_formats() {
        assert_eq!(
            date_to_interval("1938"),
            Some(("1938-01-01".to_string(), "1938-12-31".to_string()))
        );
        assert_eq!(
            date_to_interval("1936-02"),
            Some(("1936-02-01".to_string(), "1936-02-29".to_string()))
        );
        assert_eq!(
            date_to_interval("1937-02"),
            Some(("1937-02-01".to_string(), "1937-02-28".to_string()))
        );
        assert_eq!(
            date_to_interval("1938-05-15"),
            Some(("1938-05-15".to_string(), "1938-05-15".to_string()))
        );
        assert_eq!(date_to_interval("未知"), None);
        assert_eq!(date_to_interval(""), None);
    }

    #[test]
    fn test_matches_interval_overlap_month_precision() {
        let chunk = DocumentChunk {
            chunk_id: "c_overlap".to_string(),
            doc_id: "doc_test".to_string(),
            doc_title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::WarOfResistance,
            date: "1938-05".to_string(),
            volume: "选集第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec!["持久战".to_string()],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 30,
            raw_text: "持久战".to_string(),
            contextualized_text: "持久战".to_string(),
            section_path: vec![],
        };

        // Filter covering part of month: 1938-05-10 to 1938-05-15 (should overlap [1938-05-01, 1938-05-31])
        let filter_mid = VectorFilter::new().with_date_range("1938-05-10", "1938-05-15");
        assert!(chunk.date == "1938-05");
        assert!(filter_mid.matches(&chunk));

        // Filter starting after month: 1938-06-01 (should NOT overlap)
        let filter_after = VectorFilter::new().with_date_range("1938-06-01", "1938-06-30");
        assert!(!filter_after.matches(&chunk));

        // Filter ending before month: 1938-04-30 (should NOT overlap)
        let filter_before = VectorFilter::new().with_date_range("1938-04-01", "1938-04-30");
        assert!(!filter_before.matches(&chunk));
    }

    #[test]
    fn test_unknown_date_fails_interval_filter() {
        let chunk_unknown = DocumentChunk {
            chunk_id: "c_unknown".to_string(),
            doc_id: "doc_unknown".to_string(),
            doc_title: "未注明日期文稿".to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::Unknown,
            date: "未知".to_string(),
            volume: "选集".to_string(),
            category: "其他".to_string(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 10,
            raw_text: "内容".to_string(),
            contextualized_text: "内容".to_string(),
            section_path: vec![],
        };
        let filter = VectorFilter::new().with_date_range("1937-01-01", "1945-12-31");
        assert!(
            !filter.matches(&chunk_unknown),
            "chunk.date == \"未知\" must not silently pass date interval filters"
        );

        let mut chunk_empty = chunk_unknown.clone();
        chunk_empty.date = String::new();
        assert!(!filter.matches(&chunk_empty));
    }

    #[test]
    fn test_matches_leap_year_feb_29() {
        let chunk_leap = DocumentChunk {
            chunk_id: "c_leap".to_string(),
            doc_id: "doc_leap".to_string(),
            doc_title: "红军东征".to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::AgrarianRevolutionaryWar,
            date: "1936-02-29".to_string(),
            volume: "选集第一卷".to_string(),
            category: "军事".to_string(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 20,
            raw_text: "红军东征".to_string(),
            contextualized_text: "红军东征".to_string(),
            section_path: vec![],
        };

        let filter_leap = VectorFilter::new().with_date_range("1936-02-01", "1936-02-29");
        assert!(filter_leap.matches(&chunk_leap));

        let filter_non_leap = VectorFilter::new().with_date_range("1936-03-01", "1936-03-31");
        assert!(!filter_non_leap.matches(&chunk_leap));
    }

    #[test]
    fn test_periods_prefer_over_singular_period() {
        let chunk = DocumentChunk {
            chunk_id: "c1".to_string(),
            doc_id: "d1".to_string(),
            doc_title: "t".to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::WarOfResistance,
            date: "1938".to_string(),
            volume: "二卷".to_string(),
            category: "军事".to_string(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 1,
            raw_text: "x".to_string(),
            contextualized_text: "x".to_string(),
            section_path: vec![],
        };
        // Singular says Agrarian, periods says WarOfResistance — periods wins.
        let mut f = VectorFilter::new().with_period(HistoricalPeriod::AgrarianRevolutionaryWar);
        f.periods = Some(vec![HistoricalPeriod::WarOfResistance]);
        assert!(
            f.matches(&chunk),
            "when both period and periods set, periods must win"
        );
        assert_eq!(
            f.effective_periods(),
            Some(vec![HistoricalPeriod::WarOfResistance])
        );

        f.normalize_periods();
        assert!(f.period.is_none());
        assert!(f.matches(&chunk));
    }

    #[test]
    fn test_with_periods_clears_singular() {
        let f = VectorFilter::new()
            .with_period(HistoricalPeriod::Unknown)
            .with_periods(vec![HistoricalPeriod::WarOfLiberation]);
        assert!(f.period.is_none());
        assert_eq!(
            f.effective_periods(),
            Some(vec![HistoricalPeriod::WarOfLiberation])
        );
    }
}
