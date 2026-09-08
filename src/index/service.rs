use crate::error::Result;
use crate::graph::{GraphStore, expand_with_graph};
use crate::index::fulltext::{FullTextIndex, FullTextSearchResult};
use crate::index::hybrid::{HybridSearchCoordinator, HybridSearchResult};
use crate::model::{VectorFilter, VectorSearchResult};
use crate::rerank::{Reranker, rerank_or_fallback};
use crate::vector::VectorStore;
use std::sync::Arc;
use tracing::warn;

/// Single-responsibility service encapsulating the full hybrid retrieval pipeline:
/// vector search + BM25 + RRF fusion + graph expansion + rerank fallback.
#[derive(Clone)]
pub struct HybridSearchService {
    pub store: Arc<VectorStore>,
    pub fulltext: Option<Arc<FullTextIndex>>,
    pub coordinator: Arc<HybridSearchCoordinator>,
    pub graph: Option<Arc<GraphStore>>,
    pub reranker: Option<Arc<dyn Reranker>>,
}

impl HybridSearchService {
    pub fn new(
        store: Arc<VectorStore>,
        fulltext: Option<Arc<FullTextIndex>>,
        coordinator: HybridSearchCoordinator,
        graph: Option<Arc<GraphStore>>,
        reranker: Option<Arc<dyn Reranker>>,
    ) -> Self {
        Self {
            store,
            fulltext,
            coordinator: Arc::new(coordinator),
            graph,
            reranker,
        }
    }

    /// Pure vector search.
    pub async fn search_vector(
        &self,
        query: &str,
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<VectorSearchResult>> {
        self.store.search(query, top_k, filter).await
    }

    /// Pure BM25 search. Returns empty Vec when fulltext index is absent.
    pub fn search_bm25(
        &self,
        query: &str,
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<FullTextSearchResult>> {
        match &self.fulltext {
            Some(ft) => ft.search(query, top_k, filter),
            None => Ok(Vec::new()),
        }
    }

    /// Full hybrid pipeline:
    /// 1. vector search fetch_k = top_k * 2
    /// 2. BM25 search fetch_k (failure → warn + empty, non-fatal)
    /// 3. coordinator.fuse() RRF fusion
    /// 4. expand_with_graph (effective reranker → final_k=None, else Some(top_k))
    /// 5. rerank_or_fallback (skip_rerank → skip)
    pub async fn search_hybrid(
        &self,
        query: &str,
        top_k: usize,
        filter: Option<&VectorFilter>,
        skip_rerank: bool,
    ) -> Result<Vec<HybridSearchResult>> {
        let fetch_k = top_k * 2;

        // 1. Vector search
        let vector_results = self.store.search(query, fetch_k, filter).await?;

        // 2. BM25 search (non-fatal on failure)
        let bm25_results = match &self.fulltext {
            Some(ft) => match ft.search(query, fetch_k, filter) {
                Ok(v) => v,
                Err(e) => {
                    warn!("BM25 search failed: {e}, continuing vector-only");
                    Vec::new()
                }
            },
            None => Vec::new(),
        };

        // 3. RRF fusion
        let fused = self.coordinator.fuse(vector_results, bm25_results, fetch_k);

        // 4. Graph expansion
        let effective_reranker = self.reranker.is_some() && !skip_rerank;
        let final_k = if effective_reranker {
            None
        } else {
            Some(top_k)
        };
        let fused =
            expand_with_graph(self.graph.as_deref(), &self.store, fused, query, final_k).await;

        // 5. Rerank or fallback
        let reranker = if skip_rerank {
            None
        } else {
            self.reranker.as_deref()
        };
        Ok(rerank_or_fallback(fused, reranker, query, top_k).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::Domain;
    use crate::model::{DocumentChunk, HistoricalPeriod};

    fn make_chunk(id: &str, title: &str, period: HistoricalPeriod) -> DocumentChunk {
        DocumentChunk {
            chunk_id: id.to_string(),
            doc_id: format!("doc_{id}"),
            doc_title: title.to_string(),
            author: "毛泽东".to_string(),
            period,
            date: "1938-05".to_string(),
            volume: "第二卷".to_string(),
            category: "军事".to_string(),
            domain: Domain::Any,
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 50,
            raw_text: format!("text-{id}"),
            contextualized_text: format!("text-{id}"),
            section_path: vec![],
        }
    }

    fn make_service(
        store: Arc<VectorStore>,
        fulltext: Option<Arc<FullTextIndex>>,
        graph: Option<Arc<GraphStore>>,
        reranker: Option<Arc<dyn Reranker>>,
    ) -> HybridSearchService {
        HybridSearchService::new(
            store,
            fulltext,
            HybridSearchCoordinator::default(),
            graph,
            reranker,
        )
    }

    #[tokio::test]
    async fn dual_stream_hits_both_retrieval_paths() {
        let store = VectorStore::new_deterministic(8);
        let ft = FullTextIndex::new_in_ram().unwrap();

        let chunk1 = make_chunk("c1", "论持久战", HistoricalPeriod::WarOfResistance);
        let chunk2 = make_chunk("c2", "矛盾论", HistoricalPeriod::AgrarianRevolutionaryWar);

        store
            .index_chunks(vec![chunk1.clone(), chunk2.clone()])
            .await
            .unwrap();
        ft.insert_batch(&[chunk1, chunk2]).unwrap();

        let service = make_service(Arc::new(store), Some(Arc::new(ft)), None, None);
        let results = service
            .search_hybrid("持久战", 5, None, true)
            .await
            .unwrap();

        assert!(!results.is_empty(), "hybrid search should return results");
        let c1 = results.iter().find(|h| h.chunk_id == "c1");
        assert!(c1.is_some(), "c1 should be found via both paths");
        assert!(
            c1.unwrap().bm25_score.is_some(),
            "c1 should have bm25 score"
        );
        assert!(
            c1.unwrap().vector_score.is_some(),
            "c1 should have vector score"
        );
    }

    #[tokio::test]
    async fn fulltext_missing_falls_back_to_vector_only() {
        let store = VectorStore::new_deterministic(8);
        let chunk = make_chunk("c1", "论持久战", HistoricalPeriod::WarOfResistance);
        store.index_chunks(vec![chunk]).await.unwrap();

        let service = make_service(Arc::new(store), None, None, None);
        let results = service
            .search_hybrid("持久战", 5, None, true)
            .await
            .unwrap();

        assert!(
            !results.is_empty(),
            "vector-only fallback should still return results"
        );
        assert!(results.iter().all(|h| h.bm25_score.is_none()));
        assert!(results.iter().all(|h| h.vector_score.is_some()));
    }

    #[tokio::test]
    async fn graph_none_passes_through() {
        let store = VectorStore::new_deterministic(8);
        let ft = FullTextIndex::new_in_ram().unwrap();
        let chunk = make_chunk("c1", "论持久战", HistoricalPeriod::WarOfResistance);
        store.index_chunks(vec![chunk.clone()]).await.unwrap();
        ft.insert_batch(&[chunk]).unwrap();

        let service = make_service(Arc::new(store), Some(Arc::new(ft)), None, None);
        let results = service
            .search_hybrid("持久战", 5, None, true)
            .await
            .unwrap();

        assert!(!results.is_empty());
        assert!(
            results.iter().all(|h| h.graph_paths.is_none()),
            "graph=None must not inject graph_paths"
        );
    }
}
