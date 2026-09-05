pub mod cors;
pub mod dto;
pub mod error;
pub mod handlers;
pub mod metrics;
pub mod request_id;
pub mod state;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

use crate::index::HybridSearchCoordinator;
use crate::rerank::Reranker;

use self::cors::CorsAllowlist;
use self::request_id::RequestId;
use self::state::AppState;

/// Build the Axum router with CORS allowlist, request-id, metrics, and tracing.
pub fn build_router(state: AppState) -> Router {
    build_router_with_cors(state, CorsAllowlist::localhost_defaults())
}

pub fn build_router_with_cors(state: AppState, cors: CorsAllowlist) -> Router {
    Router::new()
        .route("/health", get(handlers::health::handle_health))
        .route("/api/v1/health", get(handlers::health::handle_health))
        .route("/api/v1/stats", get(handlers::health::handle_stats))
        .route("/metrics", get(metrics::handle_metrics_prometheus))
        .route("/api/v1/metrics", get(metrics::handle_metrics_json))
        .route("/api/v1/search", post(handlers::search::handle_search))
        .route("/api/v1/ask", post(handlers::ask::handle_ask))
        .route("/api/v1/ask/stream", post(handlers::ask::handle_ask_stream))
        .route("/api/v1/verify", post(handlers::verify::handle_verify))
        .route(
            "/api/v1/citation/verify",
            post(handlers::verify::handle_verify),
        )
        .with_state(state)
        .layer(cors.layer())
        .layer(middleware::from_fn(RequestId::middleware))
        .layer(TraceLayer::new_for_http())
}

/// Signal-driven graceful shutdown for `axum::serve`.
pub struct GracefulShutdown;

impl GracefulShutdown {
    /// Wait until Ctrl+C (and SIGTERM on Unix) is received.
    pub async fn wait() {
        let ctrl_c = async {
            if let Err(e) = tokio::signal::ctrl_c().await {
                tracing::error!("failed to install Ctrl+C handler: {e}");
                std::future::pending::<()>().await;
            }
        };

        #[cfg(unix)]
        let terminate = async {
            match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                Ok(mut stream) => {
                    stream.recv().await;
                }
                Err(e) => {
                    tracing::error!("failed to install SIGTERM handler: {e}");
                    std::future::pending::<()>().await;
                }
            }
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
        tracing::info!("shutdown signal received, draining connections");
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn serve(
    store: Arc<crate::vector::VectorStore>,
    tantivy: Option<Arc<crate::index::FullTextIndex>>,
    hybrid: HybridSearchCoordinator,
    reranker: Option<Arc<dyn Reranker>>,
    chat_base_url: String,
    chat_api_key: Option<String>,
    chat_model: String,
    addr: SocketAddr,
    cors: CorsAllowlist,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::new(
        store,
        tantivy,
        hybrid,
        reranker,
        chat_base_url,
        chat_api_key,
        chat_model,
    );
    let app = build_router_with_cors(state, cors);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Mao Agent API listening on http://{}", addr);
    tracing::info!("  GET  /health");
    tracing::info!("  GET  /metrics  /api/v1/metrics");
    tracing::info!("  GET  /api/v1/health  /api/v1/stats");
    tracing::info!("  POST /api/v1/search");
    tracing::info!("  POST /api/v1/ask          (blocking JSON)");
    tracing::info!("  POST /api/v1/ask/stream   (SSE)");
    tracing::info!("  POST /api/v1/verify  (/citation/verify)");
    axum::serve(listener, app)
        .with_graceful_shutdown(GracefulShutdown::wait())
        .await?;
    Ok(())
}
