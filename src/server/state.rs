use std::sync::Arc;

use crate::index::{FullTextIndex, HybridSearchCoordinator};
use crate::vector::VectorStore;

/// 共享只读状态，所有 handler 通过 Arc 克隆访问。
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<VectorStore>,
    pub tantivy: Option<Arc<FullTextIndex>>,
    pub hybrid: Arc<HybridSearchCoordinator>,
    /// LLM 兼容配置（启动时从 CLI/env/config 解析得到，请求级别可覆盖）
    pub chat_base_url: String,
    pub chat_api_key: Option<String>,
    pub chat_model: String,
}

impl AppState {
    pub fn new(
        store: Arc<VectorStore>,
        tantivy: Option<Arc<FullTextIndex>>,
        hybrid: HybridSearchCoordinator,
        chat_base_url: String,
        chat_api_key: Option<String>,
        chat_model: String,
    ) -> Self {
        Self {
            store,
            tantivy,
            hybrid: Arc::new(hybrid),
            chat_base_url,
            chat_api_key,
            chat_model,
        }
    }
}
