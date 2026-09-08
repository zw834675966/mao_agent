use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::agent::DialecticalAgent;
use crate::graph::GraphStore;
use crate::index::{FullTextIndex, HybridSearchCoordinator, HybridSearchService};
use crate::mcp::dispatcher::McpDispatcher;
use crate::rerank::Reranker;
use crate::server::config::ServerConfig;
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
    /// Optional knowledge graph for hybrid candidate expansion.
    pub graph: Option<Arc<GraphStore>>,
    /// Unified hybrid search service (vector + BM25 + RRF + graph + rerank).
    pub search_service: Arc<HybridSearchService>,
    /// Shared reqwest client reused by all agents (no per-request pool rebuild).
    pub http_client: reqwest::Client,
    /// Pre-built shared dialectical agent (fast path for default config).
    pub agent: Arc<DialecticalAgent>,
    /// Pre-built shared MCP dispatcher (zero per-request allocation).
    pub mcp_dispatcher: Arc<McpDispatcher>,
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

    /// Construct `AppState` from a pre-built `HybridSearchService` and a
    /// `ServerConfig` aggregate. This is the config-driven constructor.
    pub fn with_config(
        search_service: Arc<HybridSearchService>,
        config: ServerConfig,
        metrics: Arc<HttpMetrics>,
    ) -> Self {
        let limit = config.max_concurrent_asks.max(1);
        let graph = search_service.graph.clone();
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_default();
        let agent = Arc::new(DialecticalAgent::from_service_with_client(
            Arc::clone(&search_service),
            Some(config.chat_base_url.clone()),
            config.chat_api_key.clone(),
            Some(config.chat_model.clone()),
            http_client.clone(),
            Some(metrics.fallback_counter()),
        ));
        let mcp_dispatcher = Arc::new(McpDispatcher::from_components(
            Arc::clone(&search_service),
            search_service.store.clone(),
            Some(Arc::clone(&agent)),
        ));
        Self {
            store: search_service.store.clone(),
            tantivy: search_service.fulltext.clone(),
            hybrid: search_service.coordinator.clone(),
            reranker: search_service.reranker.clone(),
            search_service,
            chat_base_url: config.chat_base_url,
            chat_api_key: config.chat_api_key,
            chat_model: config.chat_model,
            metrics,
            api_token: config.api_token.and_then(|t| {
                let t = t.trim().to_string();
                if t.is_empty() { None } else { Some(t) }
            }),
            ask_semaphore: Arc::new(Semaphore::new(limit)),
            graph,
            http_client,
            agent,
            mcp_dispatcher,
        }
    }

    /// Backward-compatible constructor: assembles a `ServerConfig` and
    /// `HybridSearchService` from positional arguments, then delegates to
    /// [`Self::with_config`].
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
        let config = ServerConfig {
            addr: ServerConfig::default().addr,
            cors: ServerConfig::default().cors,
            api_token,
            max_concurrent_asks,
            chat_base_url,
            chat_api_key,
            chat_model,
        };
        let search_service = HybridSearchService::new(store, tantivy, hybrid, None, reranker);
        Self::with_config(Arc::new(search_service), config, metrics)
    }

    pub fn with_graph(mut self, graph: Arc<GraphStore>) -> Self {
        self.graph = Some(graph.clone());
        self.search_service = Arc::new(HybridSearchService {
            store: self.store.clone(),
            fulltext: self.tantivy.clone(),
            coordinator: self.hybrid.clone(),
            graph: Some(graph),
            reranker: self.reranker.clone(),
        });
        // Refresh agent and dispatcher to reference the updated search service.
        self.agent = Arc::new(DialecticalAgent::from_service_with_client(
            Arc::clone(&self.search_service),
            Some(self.chat_base_url.clone()),
            self.chat_api_key.clone(),
            Some(self.chat_model.clone()),
            self.http_client.clone(),
            Some(self.metrics.fallback_counter()),
        ));
        self.mcp_dispatcher = Arc::new(McpDispatcher::from_components(
            Arc::clone(&self.search_service),
            Arc::clone(&self.store),
            Some(Arc::clone(&self.agent)),
        ));
        self
    }

    /// Try to acquire an ask slot; returns 429 when the limit is exceeded.
    pub fn try_acquire_ask(&self) -> ApiResult<OwnedSemaphorePermit> {
        self.ask_semaphore
            .clone()
            .try_acquire_owned()
            .map_err(|_| ApiError::too_many_requests("ask concurrency limit exceeded"))
    }
}
