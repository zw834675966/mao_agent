//! Corpus domain classification — maps frontmatter `category` to a controlled-vocabulary
//! domain tag that flows through chunk metadata into retrieval filters.

use serde::{Deserialize, Serialize};

/// Domain of a corpus document, derived from the frontmatter `category` field.
///
/// The `Any` variant is the default (no domain filter) and the serde fallback for
/// chunks serialized before `domain` was added to `DocumentChunk`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Domain {
    /// Historical literature (Mao's works, scholarship, etc.)
    History,
    /// Engineering literature (scalability, hacker laws, algorithms, papers)
    Engineering,
    /// No domain constraint — used as default and for unfiltered retrieval.
    #[default]
    Any,
}

impl Domain {
    /// Map a frontmatter `category` string to a `Domain`.
    ///
    /// After the asset-phase scan (agy), corpus files carry `category: "history"` or
    /// `category: "engineering"`. Legacy semantic categories ("马克思主义哲学", etc.) are
    /// not recognised and fall through to `Any`.
    pub fn from_category(category: &str) -> Self {
        match category.trim().to_lowercase().as_str() {
            "history" => Domain::History,
            "engineering" => Domain::Engineering,
            _ => Domain::Any,
        }
    }

    /// Serde / index representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Domain::History => "history",
            Domain::Engineering => "engineering",
            Domain::Any => "any",
        }
    }

    /// Parse a CLI / MCP string into a `Domain`.
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "history" => Ok(Domain::History),
            "engineering" => Ok(Domain::Engineering),
            "any" => Ok(Domain::Any),
            other => Err(format!(
                "unknown domain: '{other}'. Valid values: history, engineering, any"
            )),
        }
    }
}
