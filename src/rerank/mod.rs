//! Cross-encoder / API reranking over hybrid fusion candidates.

pub mod cohere;

use async_trait::async_trait;

use crate::error::Result;
use crate::index::HybridSearchResult;

pub use cohere::{COHERE_RERANK_MODEL, COHERE_RERANK_URL, CohereReranker};

/// Reranks hybrid search candidates by relevance to a query.
#[async_trait]
pub trait Reranker: Send + Sync {
    fn model_name(&self) -> &str;

    async fn rerank(
        &self,
        query: &str,
        candidates: &[HybridSearchResult],
        top_k: usize,
    ) -> Result<Vec<HybridSearchResult>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DocumentChunk, HistoricalPeriod};
    use std::sync::Arc;

    fn make_chunk(id: &str) -> DocumentChunk {
        DocumentChunk {
            chunk_id: id.to_string(),
            doc_id: format!("doc_{id}"),
            doc_title: id.to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::WarOfResistance,
            date: "1938-05".to_string(),
            volume: "第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 10,
            raw_text: format!("text-{id}"),
            contextualized_text: format!("text-{id}"),
            section_path: vec![],
        }
    }

    fn make_result(id: &str, rank: usize) -> HybridSearchResult {
        HybridSearchResult {
            chunk_id: id.to_string(),
            rrf_score: 1.0 / rank as f32,
            bm25_score: None,
            vector_score: None,
            rerank_score: None,
            rank,
            chunk: make_chunk(id),
        }
    }

    struct MockReranker;

    #[async_trait]
    impl Reranker for MockReranker {
        fn model_name(&self) -> &str {
            "mock"
        }

        async fn rerank(
            &self,
            _query: &str,
            candidates: &[HybridSearchResult],
            top_k: usize,
        ) -> Result<Vec<HybridSearchResult>> {
            let mut out = candidates.to_vec();
            out.reverse();
            for (i, item) in out.iter_mut().enumerate() {
                item.rank = i + 1;
            }
            out.truncate(top_k);
            Ok(out)
        }
    }

    #[tokio::test]
    async fn mock_reranker_via_trait_object() {
        let candidates = vec![
            make_result("a", 1),
            make_result("b", 2),
            make_result("c", 3),
        ];
        let reranker: Arc<dyn Reranker> = Arc::new(MockReranker);
        let out = reranker.rerank("q", &candidates, 2).await.unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].chunk_id, "c");
        assert_eq!(out[1].chunk_id, "b");
        assert_eq!(reranker.model_name(), "mock");
    }
}
