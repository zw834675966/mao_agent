pub mod dto;
pub mod error;
pub mod handlers;
pub mod state;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router,
    http::Method,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::index::HybridSearchCoordinator;
use crate::rerank::Reranker;

use self::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health::handle_health))
        .route("/api/v1/health", get(handlers::health::handle_health))
        .route("/api/v1/stats", get(handlers::health::handle_stats))
        .route("/api/v1/search", post(handlers::search::handle_search))
        .route("/api/v1/ask", post(handlers::ask::handle_ask))
        .route("/api/v1/ask/stream", post(handlers::ask::handle_ask_stream))
        .route("/api/v1/verify", post(handlers::verify::handle_verify))
        // alias for citation verify
        .route(
            "/api/v1/citation/verify",
            post(handlers::verify::handle_verify),
        )
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
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
    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Mao Agent API listening on http://{}", addr);
    tracing::info!("  GET  /health");
    tracing::info!("  GET  /api/v1/health  /api/v1/stats");
    tracing::info!("  POST /api/v1/search");
    tracing::info!("  POST /api/v1/ask          (blocking JSON)");
    tracing::info!("  POST /api/v1/ask/stream   (SSE)");
    tracing::info!("  POST /api/v1/verify  (/citation/verify)");
    axum::serve(listener, app).await?;
    Ok(())
}

