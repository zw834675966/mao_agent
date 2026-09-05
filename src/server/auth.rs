//! Optional bearer API token for B-grade intranet (see ADR 0005).

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::server::state::AppState;

/// Paths that stay open for probes / local diagnostics even when a token is configured.
pub struct ApiAuth;

impl ApiAuth {
    pub fn is_public_path(path: &str) -> bool {
        matches!(
            path,
            "/live"
                | "/api/v1/live"
                | "/health"
                | "/api/v1/health"
                | "/metrics"
                | "/api/v1/metrics"
                | "/api/v1/stats"
        )
    }

    fn extract_bearer(req: &Request) -> Option<&str> {
        req.headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(str::trim)
            .filter(|s| !s.is_empty())
    }

    /// When `AppState.api_token` is set, require matching `Authorization: Bearer` on non-public routes.
    pub async fn middleware(State(state): State<AppState>, req: Request, next: Next) -> Response {
        let Some(expected) = state.api_token.as_deref() else {
            return next.run(req).await;
        };
        let path = req.uri().path();
        if Self::is_public_path(path) {
            return next.run(req).await;
        }
        match Self::extract_bearer(&req) {
            Some(got) if got == expected => next.run(req).await,
            _ => (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": "unauthorized",
                    "message": "Authorization: Bearer <token> required"
                })),
            )
                .into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_paths_include_live_and_health() {
        assert!(ApiAuth::is_public_path("/live"));
        assert!(ApiAuth::is_public_path("/health"));
        assert!(!ApiAuth::is_public_path("/api/v1/search"));
        assert!(!ApiAuth::is_public_path("/api/v1/ask"));
    }
}
