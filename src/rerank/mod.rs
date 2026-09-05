//! Cross-encoder / API reranking over hybrid fusion candidates.

pub mod cohere;

use async_trait::async_trait;
use tracing::warn;

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

/// Apply rerank when a reranker is present; on `None` or `Err`, truncate candidates to `top_k`
/// in original fused order (with a warn on error).
pub async fn rerank_or_fallback(
    candidates: Vec<HybridSearchResult>,
    reranker: Option<&dyn Reranker>,
    query: &str,
    top_k: usize,
) -> Vec<HybridSearchResult> {
    let Some(reranker) = reranker else {
        return candidates.into_iter().take(top_k).collect();
    };

    match reranker.rerank(query, &candidates, top_k).await {
        Ok(reranked) => reranked,
        Err(e) => {
            warn!("Rerank failed ({e}); falling back to fused order truncated to top_k={top_k}");
            candidates.into_iter().take(top_k).collect()
        }
    }
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
                item.rerank_score = Some(1.0 - i as f32 * 0.1);
                item.rank = i + 1;
            }
            out.truncate(top_k);
            Ok(out)
        }
    }

    struct FailingReranker;

    #[async_trait]
    impl Reranker for FailingReranker {
        fn model_name(&self) -> &str {
            "fail"
        }

        async fn rerank(
            &self,
            _query: &str,
            _candidates: &[HybridSearchResult],
            _top_k: usize,
        ) -> Result<Vec<HybridSearchResult>> {
            Err(crate::error::VectorError::RerankError("boom".into()))
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
        assert!(out[0].rerank_score.is_some());
    }

    #[tokio::test]
    async fn rerank_or_fallback_none_truncates() {
        let candidates = vec![
            make_result("a", 1),
            make_result("b", 2),
            make_result("c", 3),
        ];
        let out = rerank_or_fallback(candidates, None, "q", 2).await;
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].chunk_id, "a");
        assert_eq!(out[1].chunk_id, "b");
        assert!(out[0].rerank_score.is_none());
    }

    #[tokio::test]
    async fn rerank_or_fallback_err_keeps_fused_order() {
        let candidates = vec![
            make_result("a", 1),
            make_result("b", 2),
            make_result("c", 3),
        ];
        let r = FailingReranker;
        let out = rerank_or_fallback(candidates, Some(&r as &dyn Reranker), "q", 2).await;
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].chunk_id, "a");
        assert!(out[0].rerank_score.is_none());
    }

    #[tokio::test]
    async fn rerank_or_fallback_ok_uses_reranker() {
        let candidates = vec![
            make_result("a", 1),
            make_result("b", 2),
            make_result("c", 3),
        ];
        let r = MockReranker;
        let out = rerank_or_fallback(candidates, Some(&r as &dyn Reranker), "q", 2).await;
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].chunk_id, "c");
        assert!(out[0].rerank_score.is_some());
    }
}
