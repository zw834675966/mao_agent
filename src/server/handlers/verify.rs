use axum::{Json, extract::State, http::StatusCode};

use crate::agent::CitationVerifier;
use crate::server::dto::{VerifyRequest, VerifyResponse};
use crate::server::error::{ApiError, ApiResult};
use crate::server::state::AppState;

pub async fn handle_verify(
    State(_state): State<AppState>,
    Json(req): Json<VerifyRequest>,
) -> ApiResult<(StatusCode, Json<VerifyResponse>)> {
    if req.quote.trim().is_empty() {
        return Err(ApiError::bad_request("quote must not be empty"));
    }
    if req.claimed_title.trim().is_empty() {
        return Err(ApiError::bad_request("claimed_title must not be empty"));
    }
    if req.context_chunks.is_empty() {
        return Err(ApiError::bad_request(
            "context_chunks must not be empty: provide at least one chunk to verify against (use /api/v1/search to retrieve them)",
        ));
    }
    let min_conf = req.min_confidence.unwrap_or(0.85).clamp(0.0, 1.0);
    let verifier = CitationVerifier::new(min_conf, 6);
    let report = verifier.verify_quote(&req.quote, &req.claimed_title, &req.context_chunks);
    Ok((StatusCode::OK, Json(VerifyResponse { report })))
}
