//! Generate / propagate `X-Request-Id` and attach it to the tracing span.

use axum::extract::Request;
use axum::http::{HeaderName, HeaderValue};
use axum::middleware::Next;
use axum::response::Response;
use tracing::Instrument;

/// Request-id helpers used by the Axum middleware stack.
pub struct RequestId;

impl RequestId {
    pub const HEADER: HeaderName = HeaderName::from_static("x-request-id");

    /// Generate a unique request id without pulling in uuid (counter + time).
    pub fn generate() -> String {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        format!("{nanos:x}-{n:x}")
    }

    fn extract_or_generate(req: &Request) -> String {
        req.headers()
            .get(Self::HEADER)
            .and_then(|v| v.to_str().ok())
            .map(str::trim)
            .filter(|s| !s.is_empty() && s.len() <= 128)
            .map(str::to_owned)
            .unwrap_or_else(Self::generate)
    }

    /// Axum middleware: propagate inbound id (or mint one) onto the response + span.
    pub async fn middleware(mut req: Request, next: Next) -> Response {
        let id = Self::extract_or_generate(&req);
        if let Ok(val) = HeaderValue::from_str(&id) {
            req.headers_mut().insert(Self::HEADER, val);
        }

        let method = req.method().clone();
        let path = req.uri().path().to_owned();
        let span = tracing::info_span!(
            "http_request",
            request_id = %id,
            method = %method,
            path = %path
        );

        let mut response = next.run(req).instrument(span).await;
        if let Ok(val) = HeaderValue::from_str(&id) {
            response.headers_mut().insert(Self::HEADER, val);
        }
        response
    }
}
