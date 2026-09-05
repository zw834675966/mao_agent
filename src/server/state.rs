use std::sync::Arc;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::index::{FullTextIndex, HybridSearchCoordinator};
use crate::rerank::Reranker;
use crate::server::error::{ApiError, ApiResult};
use crate::server::metrics::HttpMetrics;
use crate::vector::VectorStore;

/// Default max concurrent `/ask` + `/ask/stream` handlers.
pub const DEFAULT_MAX_CONCURRENT_ASKS: usize = 32;

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
    /// When set, non-public API routes require `Authorization: Bearer <token>` (ADR 0005).
    pub api_token: Option<String>,
    /// Limits concurrent ask / ask-stream work.
    pub ask_semaphore: Arc<Semaphore>,
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
        Self::with_ops(
            store,
            tantivy,
            hybrid,
            reranker,
            chat_base_url,
            chat_api_key,
            chat_model,
            metrics,
            None,
            DEFAULT_MAX_CONCURRENT_ASKS,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_ops(
        store: Arc<VectorStore>,
        tantivy: Option<Arc<FullTextIndex>>,
        hybrid: HybridSearchCoordinator,
        reranker: Option<Arc<dyn Reranker>>,
        chat_base_url: String,
        chat_api_key: Option<String>,
        chat_model: String,
        metrics: Arc<HttpMetrics>,
        api_token: Option<String>,
        max_concurrent_asks: usize,
    ) -> Self {
        let limit = max_concurrent_asks.max(1);
        Self {
            store,
            tantivy,
            hybrid: Arc::new(hybrid),
            reranker,
            chat_base_url,
            chat_api_key,
            chat_model,
            metrics,
            api_token: api_token.and_then(|t| {
                let t = t.trim().to_string();
                if t.is_empty() { None } else { Some(t) }
            }),
            ask_semaphore: Arc::new(Semaphore::new(limit)),
        }
    }

    /// Try to acquire an ask slot; returns 429 when the limit is exceeded.
    pub fn try_acquire_ask(&self) -> ApiResult<OwnedSemaphorePermit> {
        self.ask_semaphore
            .clone()
            .try_acquire_owned()
            .map_err(|_| ApiError::too_many_requests("ask concurrency limit exceeded"))
    }
}
