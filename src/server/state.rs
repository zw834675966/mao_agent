use std::sync::Arc;

use crate::index::{FullTextIndex, HybridSearchCoordinator};
use crate::rerank::Reranker;
use crate::server::metrics::HttpMetrics;
use crate::vector::VectorStore;

/// Shared read-mostly state; handlers clone via Arc fields.
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<VectorStore>,
    pub tantivy: Option<Arc<FullTextIndex>>,
    pub hybrid: Arc<HybridSearchCoordinator>,
    /// Optional Cohere (or other) reranker for hybrid search.
    pub reranker: Option<Arc<dyn Reranker>>,
    /// LLM compat config (CLI/env/config at boot; request may override).
    pub chat_base_url: String,
    pub chat_api_key: Option<String>,
    pub chat_model: String,
    pub metrics: Arc<HttpMetrics>,
}

impl AppState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        store: Arc<VectorStore>,
        tantivy: Option<Arc<FullTextIndex>>,
        hybrid: HybridSearchCoordinator,
        reranker: Option<Arc<dyn Reranker>>,
        chat_base_url: String,
        chat_api_key: Option<String>,
        chat_model: String,
    ) -> Self {
        Self::with_metrics(
            store,
            tantivy,
            hybrid,
            reranker,
            chat_base_url,
            chat_api_key,
            chat_model,
            HttpMetrics::new(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_metrics(
        store: Arc<VectorStore>,
        tantivy: Option<Arc<FullTextIndex>>,
        hybrid: HybridSearchCoordinator,
        reranker: Option<Arc<dyn Reranker>>,
        chat_base_url: String,
        chat_api_key: Option<String>,
        chat_model: String,
        metrics: Arc<HttpMetrics>,
    ) -> Self {
        Self {
            store,
            tantivy,
            hybrid: Arc::new(hybrid),
            reranker,
            chat_base_url,
            chat_api_key,
            chat_model,
            metrics,
        }
    }
}
