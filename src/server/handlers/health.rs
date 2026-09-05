use axum::{Json, extract::State, http::StatusCode};

use crate::server::dto::{HealthResponse, StatsResponse};
use crate::server::state::AppState;

pub async fn handle_health(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let stats = state.store.stats().await;
    let body = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        index_loaded: stats.total_vectors > 0,
        tantivy_loaded: state.tantivy.is_some(),
        total_vectors: stats.total_vectors,
        total_documents: stats.total_documents,
        vector_dimension: stats.vector_dimension,
    };
    (StatusCode::OK, Json(body))
}

pub async fn handle_stats(State(state): State<AppState>) -> (StatusCode, Json<StatsResponse>) {
    let stats = state.store.stats().await;
    let body = StatsResponse {
        stats,
        tantivy_loaded: state.tantivy.is_some(),
    };
    (StatusCode::OK, Json(body))
}
