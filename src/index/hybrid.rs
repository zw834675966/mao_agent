use crate::index::fulltext::FullTextSearchResult;
use crate::model::{DocumentChunk, VectorSearchResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified result from dual-stream Hybrid Retrieval (BM25 + Dense Vector RRF Fusion).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridSearchResult {
    pub chunk_id: String,
    pub rrf_score: f32,
    pub bm25_score: Option<f32>,
    pub vector_score: Option<f32>,
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
                        rank: rank + 1,
                        chunk,
                    }
                },
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 50,
            raw_text: "战略防御与相持".to_string(),
            contextualized_text: "战略防御与相持".to_string(),
            section_path: vec![],
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
    }
}
