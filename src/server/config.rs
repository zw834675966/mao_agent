//! Strongly-typed HTTP server configuration aggregate.

use std::net::SocketAddr;

use crate::server::cors::CorsAllowlist;
use crate::server::state::DEFAULT_MAX_CONCURRENT_ASKS;
use crate::vector::embedder::{COHERE_CHAT_MODEL, COHERE_COMPAT_BASE_URL};

/// Aggregate of HTTP server boot-time configuration.
///
/// Replaces the 10–12 positional arguments previously threaded through
/// `serve(...)` and `AppState::with_ops(...)`, eliminating parameter sprawl.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: SocketAddr,
    pub cors: CorsAllowlist,
    pub api_token: Option<String>,
    pub max_concurrent_asks: usize,
    pub chat_base_url: String,
    pub chat_api_key: Option<String>,
    pub chat_model: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:8080".parse().unwrap(),
            cors: CorsAllowlist::localhost_defaults(),
            api_token: None,
            max_concurrent_asks: DEFAULT_MAX_CONCURRENT_ASKS,
            chat_base_url: COHERE_COMPAT_BASE_URL.to_string(),
            chat_api_key: None,
            chat_model: COHERE_CHAT_MODEL.to_string(),
        }
    }
}
