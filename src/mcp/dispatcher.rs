//! Transport-agnostic MCP dispatcher: pure JSON-RPC 2.0 protocol router.

use std::sync::Arc;

use crate::agent::DialecticalAgent;
use crate::graph::GraphStore;
use crate::index::{FullTextIndex, HybridSearchService};
use crate::mcp::tools::{citation, principles};
use crate::mcp::types::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpCallToolResult, MCP_PROTOCOL_VERSION,
    SERVER_NAME, SERVER_VERSION, list_all_tools,
};
use crate::rerank::Reranker;
use crate::vector::VectorStore;

#[derive(Clone)]
pub struct McpDispatcher {
    search_service: Arc<HybridSearchService>,
    store: Arc<VectorStore>,
    agent: Option<Arc<DialecticalAgent>>,
}

impl McpDispatcher {
    pub fn from_components(
        search_service: Arc<HybridSearchService>, store: Arc<VectorStore>,
        agent: Option<Arc<DialecticalAgent>>,
    ) -> Self {
        Self { search_service, store, agent }
    }

    pub fn new(
        store: Arc<VectorStore>, tantivy: Option<Arc<FullTextIndex>>,
        graph: Option<Arc<GraphStore>>, reranker: Option<Arc<dyn Reranker>>,
    ) -> Self {
        let search_service = Arc::new(HybridSearchService::new(
            Arc::clone(&store), tantivy, Default::default(), graph, reranker,
        ));
        Self::from_components(search_service, store, None)
    }

    pub fn with_chat_overrides(
        mut self, base_url: Option<String>, api_key: Option<String>, model: Option<String>,
    ) -> Self {
        let agent = DialecticalAgent::from_service_with_client(
            Arc::clone(&self.search_service), base_url, api_key, model,
            reqwest::Client::new(), None,
        );
        self.agent = Some(Arc::new(agent));
        self
    }

    pub fn with_agent(mut self, agent: DialecticalAgent) -> Self {
        self.agent = Some(Arc::new(agent));
        self
    }

    pub fn from_app_state(state: &crate::server::state::AppState) -> Self {
        Self::from_components(
            Arc::clone(&state.search_service), Arc::clone(&state.store), None,
        )
    }

    pub async fn handle_request(&self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
        if req.is_notification() {
            if req.method == "notifications/initialized" {
                tracing::info!("MCP client initialized");
            }
            return None;
        }
        let id = req.id.clone();
        let result = match req.method.as_str() {
            "initialize" => Ok(serde_json::json!({
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "capabilities": { "tools": { "listChanged": false } },
                "serverInfo": { "name": SERVER_NAME, "version": SERVER_VERSION },
            })),
            "ping" => Ok(serde_json::json!({})),
            "tools/list" => Ok(serde_json::json!({ "tools": list_all_tools() })),
            "tools/call" => self.handle_tools_call(req).await,
            other => Err(JsonRpcError::method_not_found(other)),
        };
        Some(match result {
            Ok(v) => JsonRpcResponse::success(id, v),
            Err(e) => JsonRpcResponse::error(id, e),
        })
    }

    async fn handle_tools_call(
        &self, req: JsonRpcRequest,
    ) -> std::result::Result<serde_json::Value, JsonRpcError> {
        let params = req.params.as_ref().ok_or_else(|| {
            JsonRpcError::invalid_params("Missing parameters object for tools/call")
        })?;
        let name = params.get("name").and_then(|v| v.as_str()).ok_or_else(|| {
            JsonRpcError::invalid_params("Missing tool name in tools/call")
        })?;
        let arguments = params.get("arguments").cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let result = match name {
            "query_dialectical_principles" => principles::execute_query_dialectical_principles(
                &self.search_service, self.agent.as_deref(), arguments,
            ).await?,
            "verify_historical_citation" => citation::execute_verify_historical_citation(
                &self.store, arguments,
            ).await?,
            unknown => McpCallToolResult::error(format!("Unknown tool: {unknown}")),
        };
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn call(id: i64, method: &str) -> JsonRpcResponse {
        McpDispatcher::new(Arc::new(VectorStore::new_deterministic(64)), None, None, None)
            .handle_request(JsonRpcRequest {
                jsonrpc: "2.0".into(), id: Some(serde_json::json!(id)),
                method: method.into(), params: None,
            }).await.unwrap()
    }

    #[tokio::test]
    async fn initialize() {
        let r = call(1, "initialize").await;
        assert!(r.error.is_none());
        assert_eq!(r.result.unwrap()["protocolVersion"], "2024-11-05");
    }

    #[tokio::test]
    async fn ping() {
        let r = call(2, "ping").await;
        assert!(r.error.is_none());
        assert_eq!(r.result, Some(serde_json::json!({})));
    }

    #[tokio::test]
    async fn tools_list() {
        let r = call(3, "tools/list").await;
        assert_eq!(r.result.unwrap()["tools"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn unknown_method() {
        let r = call(4, "unknown/method").await;
        assert_eq!(r.error.unwrap().code, -32601);
    }
}
