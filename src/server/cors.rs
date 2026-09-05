//! Configurable CORS origin allowlist for B-grade intranet.

use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

/// CORS allowlist resolved from CLI / env / config / localhost defaults.
#[derive(Debug, Clone)]
pub struct CorsAllowlist {
    origins: Vec<HeaderValue>,
}

impl CorsAllowlist {
    /// B-grade localhost defaults (SPA + API common ports).
    pub fn localhost_defaults() -> Self {
        Self::from_origin_strs(&[
            "http://localhost:3000",
            "http://127.0.0.1:3000",
            "http://localhost:5173",
            "http://127.0.0.1:5173",
            "http://localhost:8080",
            "http://127.0.0.1:8080",
        ])
    }

    pub fn from_origin_strs(origins: &[&str]) -> Self {
        let origins = origins
            .iter()
            .filter_map(|s| {
                let t = s.trim();
                if t.is_empty() {
                    return None;
                }
                HeaderValue::from_str(t).ok()
            })
            .collect();
        Self { origins }
    }

    /// Parse a comma-separated origin list (CLI / `MAO_CORS_ORIGINS`).
    pub fn from_csv(csv: &str) -> Self {
        let parts: Vec<&str> = csv
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if parts.is_empty() {
            return Self::localhost_defaults();
        }
        Self::from_origin_strs(&parts)
    }

    pub fn from_string_list(list: &[String]) -> Self {
        let refs: Vec<&str> = list.iter().map(String::as_str).collect();
        if refs.is_empty() {
            return Self::localhost_defaults();
        }
        Self::from_origin_strs(&refs)
    }

    pub fn origins(&self) -> &[HeaderValue] {
        &self.origins
    }

    pub fn contains_origin(&self, origin: &str) -> bool {
        self.origins
            .iter()
            .any(|o| o.as_bytes() == origin.as_bytes())
    }

    /// Resolve: CLI/env CSV → config list → localhost defaults.
    pub fn resolve(cli_or_env: Option<&str>, config_list: Option<&[String]>) -> Self {
        if let Some(csv) = cli_or_env.map(str::trim).filter(|s| !s.is_empty()) {
            return Self::from_csv(csv);
        }
        if let Some(list) = config_list.filter(|l| !l.is_empty()) {
            return Self::from_string_list(list);
        }
        Self::localhost_defaults()
    }

    pub fn layer(&self) -> CorsLayer {
        let allow_origin = if self.origins.is_empty() {
            AllowOrigin::list(Self::localhost_defaults().origins.clone())
        } else {
            AllowOrigin::list(self.origins.clone())
        };
        CorsLayer::new()
            .allow_origin(allow_origin)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                RequestIdHeader::header(),
            ])
    }
}

/// Local alias so cors.rs does not circular-import request_id at type level for header name.
struct RequestIdHeader;

impl RequestIdHeader {
    fn header() -> axum::http::HeaderName {
        axum::http::HeaderName::from_static("x-request-id")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_include_localhost() {
        let c = CorsAllowlist::localhost_defaults();
        assert!(c.contains_origin("http://localhost:3000"));
        assert!(c.contains_origin("http://127.0.0.1:5173"));
        assert!(!c.contains_origin("https://evil.example"));
    }

    #[test]
    fn csv_overrides_defaults() {
        let c = CorsAllowlist::from_csv("http://intranet.local:3000, http://app.local");
        assert!(c.contains_origin("http://intranet.local:3000"));
        assert!(c.contains_origin("http://app.local"));
        assert!(!c.contains_origin("http://localhost:3000"));
    }

    #[test]
    fn resolve_prefers_cli() {
        let cfg = vec!["http://from-config".to_string()];
        let c = CorsAllowlist::resolve(Some("http://from-cli"), Some(&cfg));
        assert!(c.contains_origin("http://from-cli"));
        assert!(!c.contains_origin("http://from-config"));
    }
}
