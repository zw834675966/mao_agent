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

    /// Check if a chunk satisfies this filter.
    pub fn matches(&self, chunk: &DocumentChunk) -> bool {
        if let Some(p) = self.period
            && chunk.period != p
        {
            return false;
        }
        if let Some(ref periods) = self.periods
            && !periods.contains(&chunk.period)
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
                if !chunk.tags.iter().any(|t| t.contains(tag)) {
                    return false;
                }
            }
        }
        if let Some(ref start) = self.start_date
            && chunk.date != "未知"
            && chunk.date.as_str() < start.as_str()
        {
            return false;
        }
        if let Some(ref end) = self.end_date
            && chunk.date != "未知"
            && chunk.date.as_str() > end.as_str()
        {
            return false;
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
