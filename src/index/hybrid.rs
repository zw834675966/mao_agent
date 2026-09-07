use crate::index::fulltext::FullTextSearchResult;
use crate::model::{DocumentChunk, VectorSearchResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;
use std::sync::LazyLock;

/// Unified result from dual-stream Hybrid Retrieval (BM25 + Dense Vector RRF Fusion).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridSearchResult {
    pub chunk_id: String,
    pub rrf_score: f32,
    pub bm25_score: Option<f32>,
    pub vector_score: Option<f32>,
    /// Cohere (or other) cross-encoder relevance score when rerank was applied.
    #[serde(default)]
    pub rerank_score: Option<f32>,
    /// Graph expander paths when this chunk was annotated or injected. Absent on dual-only hits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_paths: Option<Vec<String>>,
    pub rank: usize,
    pub chunk: DocumentChunk,
}

/// Coordinator for fusing BM25 full-text search and Vector dense search using Reciprocal Rank Fusion (RRF).
pub struct HybridSearchCoordinator {
    pub k_constant: f32,
    pub vector_weight: f32,
    pub bm25_weight: f32,
}

impl Default for HybridSearchCoordinator {
    fn default() -> Self {
        Self {
            k_constant: 60.0,
            vector_weight: 0.5,
            bm25_weight: 0.5,
        }
    }
}

pub type ChunkScoreEntry = (DocumentChunk, f32, Option<f32>, Option<f32>);
// ── Query-Intent Adaptive RRF: rule-based intent detection ──────────────

static TITLE_PAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"《[^》]+》"#).unwrap());
static YEAR_PAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\b(1\d{3}|20[0-2]\d)\b"#).unwrap());

/// Detect query intent and return `(vector_weight, bm25_weight)`.
///
/// ## Rules (no training cost, < 50 lines):
/// - **Exact-match heavy** (BM25 ↑): contains `《》` title markers, year tokens, or
///   proper-noun markers like "第X卷" / "第X章" → `(0.2, 0.8)`.
/// - **Abstract / conceptual heavy** (Vector ↑): starts with "如何" / "为什么" /
///   "怎样" / "是否" / "能不能" → `(0.8, 0.2)`.
/// - **Default**: `(0.5, 0.5)` (unchanged).
pub fn detect_query_intent(query: &str) -> (f32, f32) {
    let q = query.trim();
    // Abstract pattern: 如何/为什么/怎样/是否/能不能 at the start of the query
    if q.starts_with("如何")
        || q.starts_with("为什么")
        || q.starts_with("怎样")
        || q.starts_with("是否")
        || q.starts_with("能不能")
        || q.starts_with("为何")
        || q.starts_with("何以")
        || q.starts_with("怎么")
    {
        // Semantic / abstract → vector-heavy
        return (0.8, 0.2);
    }

    // Title / proper-noun pattern: 《》, year, 第X卷/章
    if TITLE_PAT.is_match(q) || YEAR_PAT.is_match(q) {
        return (0.2, 0.8);
    }
    if q.contains("第") && (q.contains("卷") || q.contains("章") || q.contains("节")) {
        return (0.2, 0.8);
    }

    // Default balanced
    (0.5, 0.5)
}

impl HybridSearchCoordinator {
    pub fn new(k_constant: f32, vector_weight: f32, bm25_weight: f32) -> Self {
        Self {
            k_constant,
            vector_weight,
            bm25_weight,
        }
    }

    /// Merge vector search results and BM25 full-text results using Reciprocal Rank Fusion (RRF).
    pub fn fuse(
        &self,
        vector_results: Vec<VectorSearchResult>,
        bm25_results: Vec<FullTextSearchResult>,
        top_k: usize,
    ) -> Vec<HybridSearchResult> {
        // Map from chunk_id to (DocumentChunk, rrf_score, Option<bm25_score>, Option<vector_score>)
        let mut score_map: HashMap<String, ChunkScoreEntry> = HashMap::new();

        // 1. Accumulate Vector scores
        for (rank, res) in vector_results.into_iter().enumerate() {
            let rrf = self.vector_weight / (self.k_constant + (rank + 1) as f32);
            let entry = score_map
                .entry(res.chunk_id.clone())
                .or_insert_with(|| (res.chunk, 0.0, None, None));
            entry.1 += rrf;
            entry.3 = Some(res.score);
        }

        // 2. Accumulate BM25 scores
        for (rank, res) in bm25_results.into_iter().enumerate() {
            let rrf = self.bm25_weight / (self.k_constant + (rank + 1) as f32);
            let entry = score_map
                .entry(res.chunk_id.clone())
                .or_insert_with(|| (res.chunk, 0.0, None, None));
            entry.1 += rrf;
            entry.2 = Some(res.score);
        }

        // 3. Sort by RRF score descending
        let mut merged: Vec<(String, ChunkScoreEntry)> = score_map.into_iter().collect();
        merged.sort_by(|a, b| {
            b.1.1
                .partial_cmp(&a.1.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        merged.truncate(top_k);

        merged
            .into_iter()
            .enumerate()
            .map(
                |(rank, (chunk_id, (chunk, rrf_score, bm25_score, vector_score)))| {
                    HybridSearchResult {
                        chunk_id,
                        rrf_score,
                        bm25_score,
                        vector_score,
                        rerank_score: None,
                        graph_paths: None,
                        rank: rank + 1,
                        chunk,
                    }
                },
            )
            .collect()
    }

      /// Merge with explicit weights so callers can supply adaptive values.
      pub fn fuse_with_weights(
          &self,
          vector_results: Vec<VectorSearchResult>,
          bm25_results: Vec<FullTextSearchResult>,
          top_k: usize,
          vector_weight: f32,
          bm25_weight: f32,
      ) -> Vec<HybridSearchResult> {
          let mut score_map: HashMap<String, ChunkScoreEntry> = HashMap::new();

          for (rank, res) in vector_results.into_iter().enumerate() {
              let rrf = vector_weight / (self.k_constant + (rank + 1) as f32);
              let entry = score_map
                  .entry(res.chunk_id.clone())
                  .or_insert_with(|| (res.chunk, 0.0, None, None));
              entry.1 += rrf;
              entry.3 = Some(res.score);
          }

          for (rank, res) in bm25_results.into_iter().enumerate() {
              let rrf = bm25_weight / (self.k_constant + (rank + 1) as f32);
              let entry = score_map
                  .entry(res.chunk_id.clone())
                  .or_insert_with(|| (res.chunk, 0.0, None, None));
              entry.1 += rrf;
              entry.2 = Some(res.score);
          }

          let mut merged: Vec<(String, ChunkScoreEntry)> = score_map.into_iter().collect();
          merged.sort_by(|a, b| {
              b.1.1
                  .partial_cmp(&a.1.1)
                  .unwrap_or(std::cmp::Ordering::Equal)
          });
          merged.truncate(top_k);

          merged
              .into_iter()
              .enumerate()
              .map(
                  |(rank, (chunk_id, (chunk, rrf_score, bm25_score, vector_score)))| {
                      HybridSearchResult {
                          chunk_id,
                          rrf_score,
                          bm25_score,
                          vector_score,
                          rerank_score: None,
                          graph_paths: None,
                          rank: rank + 1,
                          chunk,
                      }
                  },
              )
              .collect()
      }

      /// Query-intent aware fusion: detect intent and use adaptive weights.
      pub fn fuse_adaptive(
          &self,
          query: &str,
          vector_results: Vec<VectorSearchResult>,
          bm25_results: Vec<FullTextSearchResult>,
          top_k: usize,
      ) -> Vec<HybridSearchResult> {
          let (vector_weight, bm25_weight) = detect_query_intent(query);
          self.fuse_with_weights(
              vector_results,
              bm25_results,
              top_k,
              vector_weight,
              bm25_weight,
          )
      }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::Domain;
    use crate::model::HistoricalPeriod;

    fn make_chunk(id: &str, title: &str) -> DocumentChunk {
        DocumentChunk {
            chunk_id: id.to_string(),
            doc_id: format!("doc_{}", id),
            doc_title: title.to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::WarOfResistance,
            date: "1938-05".to_string(),
            volume: "第二卷".to_string(),
            category: "军事".to_string(),
            domain: Domain::Any,
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 50,
            raw_text: "战略防御与相持".to_string(),
            contextualized_text: "战略防御与相持".to_string(),
            section_path: vec![],
parent_text: None,
        }
    }

    #[test]
    fn test_rrf_fusion() {
        let coordinator = HybridSearchCoordinator::default();

        let v_res = vec![
            VectorSearchResult {
                chunk_id: "c1".to_string(),
                score: 0.9,
                rank: 1,
                chunk: make_chunk("c1", "论持久战"),
            },
            VectorSearchResult {
                chunk_id: "c2".to_string(),
                score: 0.7,
                rank: 2,
                chunk: make_chunk("c2", "矛盾论"),
            },
        ];

        let b_res = vec![
            FullTextSearchResult {
                chunk_id: "c2".to_string(),
                score: 5.2,
                rank: 1,
                chunk: make_chunk("c2", "矛盾论"),
            },
            FullTextSearchResult {
                chunk_id: "c1".to_string(),
                score: 3.1,
                rank: 2,
                chunk: make_chunk("c1", "论持久战"),
            },
        ];

        let fused = coordinator.fuse(v_res, b_res, 5);
        assert_eq!(fused.len(), 2);
        // Both items scored, ranks fused
        assert!(fused[0].bm25_score.is_some());
        assert!(fused[0].vector_score.is_some());
        assert!(fused.iter().all(|h| h.graph_paths.is_none()));
    }

    #[test]
    fn test_detect_query_intent_abstract() {
        let (vw, bw) = detect_query_intent("为什么抗日战争是持久战？");
        assert_eq!(vw, 0.8);
        assert_eq!(bw, 0.2);
    }

    #[test]
    fn test_detect_query_intent_title() {
        let (vw, bw) = detect_query_intent("《论持久战》中的战略防御阶段");
        assert_eq!(vw, 0.2);
        assert_eq!(bw, 0.8);
    }

    #[test]
    fn test_detect_query_intent_default() {
        let (vw, bw) = detect_query_intent("抗日战争战略分析");
        assert_eq!(vw, 0.5);
        assert_eq!(bw, 0.5);
    }
}
