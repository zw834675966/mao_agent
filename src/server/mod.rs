pub mod auth;
pub mod cors;
pub mod dto;
pub mod error;
pub mod handlers;
pub mod metrics;
pub mod request_id;
pub mod state;

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

use crate::index::HybridSearchCoordinator;
use crate::rerank::Reranker;

use self::auth::ApiAuth;
use self::cors::CorsAllowlist;
use self::request_id::RequestId;
use self::state::AppState;

/// Build the Axum router with CORS allowlist, request-id, optional auth, metrics, and tracing.
pub fn build_router(state: AppState) -> Router {
    build_router_with_cors(state, CorsAllowlist::localhost_defaults())
}

pub fn build_router_with_cors(state: AppState, cors: CorsAllowlist) -> Router {
    let auth_state = state.clone();
    Router::new()
        .route("/live", get(handlers::health::handle_live))
        .route("/api/v1/live", get(handlers::health::handle_live))
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
        .route("/mcp", post(handlers::mcp::handle_mcp))
        .route("/api/v1/mcp", post(handlers::mcp::handle_mcp))
        .with_state(state)
        .layer(middleware::from_fn_with_state(
            auth_state,
            ApiAuth::middleware,
        ))
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

/// Serve with a custom shutdown future (testable seam).
pub async fn serve_with_shutdown<F>(
    app: Router,
    addr: SocketAddr,
    shutdown: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Future<Output = ()> + Send + 'static,
{
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Mao Agent API listening on http://{}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;
    Ok(())
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
    api_token: Option<String>,
    max_concurrent_asks: usize,
    graph: Option<std::sync::Arc<crate::graph::GraphStore>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::with_ops(
        store,
        tantivy,
        hybrid,
        reranker,
        chat_base_url,
        chat_api_key,
        chat_model,
        metrics::HttpMetrics::new(),
        api_token,
        max_concurrent_asks,
    );
    if let Some(g) = graph {
        state = state.with_graph(g);
    }
    let app = build_router_with_cors(state, cors);
    tracing::info!("  GET  /live  /health");
    tracing::info!("  GET  /metrics  /api/v1/metrics");
    tracing::info!("  GET  /api/v1/health  /api/v1/stats");
    tracing::info!("  POST /api/v1/search");
    tracing::info!("  POST /api/v1/ask          (blocking JSON)");
    tracing::info!("  POST /api/v1/ask/stream   (SSE)");
    tracing::info!("  POST /api/v1/verify  (/citation/verify)");
    serve_with_shutdown(app, addr, GracefulShutdown::wait()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn graceful_shutdown_stops_serve() {
        let store = Arc::new(crate::vector::VectorStore::new_deterministic(32));
        let state = AppState::new(
            store,
            None,
            HybridSearchCoordinator::default(),
            None,
            "http://127.0.0.1:9".into(),
            None,
            "m".into(),
        );
        let app = build_router(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let server = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = rx.await;
                })
                .await
        });
        // Smoke: process answers while up
        let client = reqwest::Client::new();
        let resp = client
            .get(format!("http://{addr}/live"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
        tx.send(()).unwrap();
        server.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn live_always_200_when_index_empty() {
        let store = Arc::new(crate::vector::VectorStore::new_deterministic(32));
        let state = AppState::new(
            store,
            None,
            HybridSearchCoordinator::default(),
            None,
            "http://127.0.0.1:9".into(),
            None,
            "m".into(),
        );
        let app = build_router(state);
        let live = app
            .clone()
            .oneshot(Request::builder().uri("/live").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(live.status(), axum::http::StatusCode::OK);
        let health = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(health.status(), axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }
}
