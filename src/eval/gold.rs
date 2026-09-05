//! Gold retrieval query loading and lexical-hardness helpers (no network).

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Optional metadata filter attached to a gold query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoldQueryFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// One NDJSON gold query line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoldQuery {
    pub query: String,
    pub expected_chunk_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<GoldQueryFilter>,
    /// Optional hardness tag: paraphrase / hard_negative / cross_doc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl GoldQuery {
    /// Extract the first corner-quoted stem when present.
    pub fn quoted_stem(query: &str) -> Option<&str> {
        let start = query.find('\u{300c}')?;
        let after = start + '\u{300c}'.len_utf8();
        let end_rel = query[after..].find('\u{300d}')?;
        let stem = &query[after..after + end_rel];
        if stem.is_empty() { None } else { Some(stem) }
    }

    /// Template-style easy query: long quoted stem copied from chunk text.
    pub fn has_easy_quoted_stem(query: &str, min_stem_chars: usize) -> bool {
        Self::quoted_stem(query)
            .map(|s| s.chars().count() >= min_stem_chars)
            .unwrap_or(false)
    }

    /// Longest contiguous character substring shared by `a` and `b`.
    pub fn longest_char_substring(a: &str, b: &str) -> usize {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        if a.is_empty() || b.is_empty() {
            return 0;
        }
        let mut best = 0usize;
        let mut dp = vec![0usize; b.len() + 1];
        for &ca in &a {
            let mut prev = 0usize;
            for (j, &cb) in b.iter().enumerate() {
                let j1 = j + 1;
                let next_prev = dp[j1];
                if ca == cb {
                    dp[j1] = prev + 1;
                    best = best.max(dp[j1]);
                } else {
                    dp[j1] = 0;
                }
                prev = next_prev;
            }
        }
        best
    }

    /// Lexically easy if a long quoted stem exists, or body overlap is high.
    pub fn is_lexically_easy(query: &str, chunk_text: &str, overlap_chars: usize) -> bool {
        if Self::has_easy_quoted_stem(query, 12) {
            return true;
        }
        Self::longest_char_substring(query, chunk_text) >= overlap_chars
    }

    /// True if any contiguous n-char window of `query` appears in `haystack`.
    pub fn ngram_contains_any(query: &str, haystack: &str, n: usize) -> bool {
        if n == 0 {
            return false;
        }
        let q: Vec<char> = query.chars().collect();
        if q.len() < n {
            return haystack.contains(query);
        }
        for i in 0..=q.len() - n {
            let gram: String = q[i..i + n].iter().collect();
            if haystack.contains(&gram) {
                return true;
            }
        }
        false
    }

    /// Remove book-title spans so title leakage does not dominate lexical probes.
    pub fn strip_book_titles(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut rest = s;
        while let Some(start) = rest.find('\u{300a}') {
            out.push_str(&rest[..start]);
            let open_len = '\u{300a}'.len_utf8();
            if let Some(end_rel) = rest[start + open_len..].find('\u{300b}') {
                let after = start + open_len + end_rel + '\u{300b}'.len_utf8();
                rest = &rest[after..];
            } else {
                rest = &rest[start + open_len..];
            }
        }
        out.push_str(rest);
        out
    }
}

/// A loaded gold query set with static helpers for hardness audits.
#[derive(Debug, Clone, Default)]
pub struct GoldQuerySet {
    pub queries: Vec<GoldQuery>,
}

impl GoldQuerySet {
    pub fn parse_jsonl(text: &str) -> Result<Self, String> {
        let mut queries = Vec::new();
        for (line_no, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let q: GoldQuery = serde_json::from_str(trimmed)
                .map_err(|e| format!("invalid JSON on line {}: {e}", line_no + 1))?;
            if q.expected_chunk_ids.is_empty() {
                return Err(format!(
                    "line {}: expected_chunk_ids must be non-empty",
                    line_no + 1
                ));
            }
            queries.push(q);
        }
        if queries.is_empty() {
            return Err("queries file contained no records".into());
        }
        Ok(Self { queries })
    }

    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text =
            std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        Self::parse_jsonl(&text)
    }

    pub fn len(&self) -> usize {
        self.queries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queries.is_empty()
    }

    /// Fraction of queries with an easy corner-quoted stem.
    pub fn easy_quoted_stem_rate(&self, min_stem_chars: usize) -> f32 {
        if self.queries.is_empty() {
            return 0.0;
        }
        let n = self
            .queries
            .iter()
            .filter(|q| GoldQuery::has_easy_quoted_stem(&q.query, min_stem_chars))
            .count();
        n as f32 / self.queries.len() as f32
    }

    /// Rank corpus pairs by longest shared substring with `query`.
    pub fn substring_rank(query: &str, corpus: &[(String, String)], k: usize) -> Vec<String> {
        let mut scored: Vec<(usize, &str)> = corpus
            .iter()
            .map(|(id, text)| (GoldQuery::longest_char_substring(query, text), id.as_str()))
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        scored
            .into_iter()
            .take(k)
            .map(|(_, id)| id.to_string())
            .collect()
    }

    /// Mean Recall@k under the naive substring ranker.
    pub fn mean_substring_recall_at_k(&self, corpus: &[(String, String)], k: usize) -> f32 {
        self.mean_substring_recall_at_k_ex(corpus, k, false)
    }

    /// Mean Recall@k under substring ranking, optionally stripping book titles.
    pub fn mean_substring_recall_at_k_ex(
        &self,
        corpus: &[(String, String)],
        k: usize,
        strip_titles: bool,
    ) -> f32 {
        if self.queries.is_empty() {
            return 0.0;
        }
        let prepared: Vec<(String, String)> = if strip_titles {
            corpus
                .iter()
                .map(|(id, text)| (id.clone(), GoldQuery::strip_book_titles(text)))
                .collect()
        } else {
            corpus.to_vec()
        };
        let mut sum = 0.0_f32;
        for q in &self.queries {
            let probe = if strip_titles {
                GoldQuery::strip_book_titles(&q.query)
            } else {
                q.query.clone()
            };
            let retrieved = Self::substring_rank(&probe, &prepared, k);
            sum += crate::eval::recall_at_k(&retrieved, &q.expected_chunk_ids, k);
        }
        sum / self.queries.len() as f32
    }

    /// Mean Recall@k where a chunk is kept iff any n-char gram of the query appears in it.
    pub fn mean_ngram_containment_recall_at_k(
        &self,
        corpus: &[(String, String)],
        k: usize,
        n: usize,
        strip_titles: bool,
    ) -> f32 {
        if self.queries.is_empty() {
            return 0.0;
        }
        let prepared: Vec<(String, String)> = if strip_titles {
            corpus
                .iter()
                .map(|(id, text)| (id.clone(), GoldQuery::strip_book_titles(text)))
                .collect()
        } else {
            corpus.to_vec()
        };
        let mut sum = 0.0_f32;
        for q in &self.queries {
            let probe = if strip_titles {
                GoldQuery::strip_book_titles(&q.query)
            } else {
                q.query.clone()
            };
            let mut hits: Vec<(usize, &str)> = prepared
                .iter()
                .filter(|(_, text)| GoldQuery::ngram_contains_any(&probe, text, n))
                .map(|(id, text)| (GoldQuery::longest_char_substring(&probe, text), id.as_str()))
                .collect();
            hits.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
            let retrieved: Vec<String> = hits
                .into_iter()
                .take(k)
                .map(|(_, id)| id.to_string())
                .collect();
            sum += crate::eval::recall_at_k(&retrieved, &q.expected_chunk_ids, k);
        }
        sum / self.queries.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_stem_and_easy_detection() {
        let q = "根据《实践论》，如何理解「马克思主义者认为人类的生产活动是最基本的实践活动」？";
        assert_eq!(
            GoldQuery::quoted_stem(q),
            Some("马克思主义者认为人类的生产活动是最基本的实践活动")
        );
        assert!(GoldQuery::has_easy_quoted_stem(q, 12));
        assert!(!GoldQuery::has_easy_quoted_stem(
            "《实践论》认为检验认识真理性的标准是什么？",
            12
        ));
    }

    #[test]
    fn longest_substring_and_rank() {
        assert_eq!(
            GoldQuery::longest_char_substring("没有调查没有发言权", "没有调查，没有发言权"),
            5
        );
        let corpus = vec![
            ("a".into(), "xxxx没有调查没有发言权yyyy".into()),
            ("b".into(), "完全不相关的段落".into()),
        ];
        let ranked = GoldQuerySet::substring_rank("没有调查没有发言权", &corpus, 2);
        assert_eq!(ranked[0], "a");
    }

    #[test]
    fn strip_titles_and_ngram() {
        let s = GoldQuery::strip_book_titles("见《实践论》与《矛盾论》两篇");
        assert_eq!(s, "见与两篇");
        assert!(GoldQuery::ngram_contains_any(
            "没有调查没有发言权",
            "没有调查，没有发言权",
            4
        ));
        assert!(!GoldQuery::ngram_contains_any("abcdef", "zzzzzz", 3));
    }
}
