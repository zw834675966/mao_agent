use axum::{Json, extract::State, http::StatusCode};

use crate::server::dto::{HealthResponse, StatsResponse};
use crate::server::state::AppState;

async fn health_body(state: &AppState) -> HealthResponse {
    let stats = state.store.stats().await;
    let index_loaded = stats.total_vectors > 0;
    HealthResponse {
        status: if index_loaded {
            "ok".to_string()
        } else {
            "unavailable".to_string()
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
        index_loaded,
        tantivy_loaded: state.tantivy.is_some(),
        total_vectors: stats.total_vectors,
        total_documents: stats.total_documents,
        vector_dimension: stats.vector_dimension,
    }
}

/// Liveness probe: process is up. Always HTTP 200 (even if index empty).
pub async fn handle_live(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let mut body = health_body(&state).await;
    // Liveness status is always ok at HTTP layer; body still reports index_loaded.
    body.status = "ok".to_string();
    (StatusCode::OK, Json(body))
}

/// Readiness probe (`/health`, `/api/v1/health`): 503 when vector index empty.
pub async fn handle_health(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let body = health_body(&state).await;
    let code = if body.index_loaded {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (code, Json(body))
}

pub async fn handle_stats(State(state): State<AppState>) -> (StatusCode, Json<StatsResponse>) {
    let stats = state.store.stats().await;
    let body = StatsResponse {
        stats,
        tantivy_loaded: state.tantivy.is_some(),
    };
    (StatusCode::OK, Json(body))
}
