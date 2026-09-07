//! Transport-agnostic MCP dispatcher implementing JSON-RPC 2.0 tool execution.
//!
//! Exposes:
//! - `query_dialectical_principles`: Hybrid RRF + Knowledge Graph triad retrieval + optional DialecticalAgent synthesis.
//! - `verify_historical_citation`: Character-level citation verification with automatic local corpus fallback.

use std::sync::Arc;

use crate::agent::DialecticalAgent;
use crate::graph::GraphStore;
use crate::index::{FullTextIndex, HybridSearchService};
use crate::mcp::types::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, MCP_PROTOCOL_VERSION, McpCallToolResult,
    McpInitializeResult, McpServerCapabilities, McpServerInfo, McpToolsCapability, SERVER_NAME,
    SERVER_VERSION, list_all_tools,
};
use crate::rerank::Reranker;
use crate::vector::VectorStore;

#[derive(Clone)]
pub struct McpDispatcher {
    store: Arc<VectorStore>,
    search_service: Arc<HybridSearchService>,
    chat_base_url: Option<String>,
    chat_api_key: Option<String>,
    chat_model: Option<String>,
    agent: Option<Arc<DialecticalAgent>>,
}

impl McpDispatcher {
    pub fn new(
        store: Arc<VectorStore>,
        tantivy: Option<Arc<FullTextIndex>>,
        graph: Option<Arc<GraphStore>>,
        reranker: Option<Arc<dyn Reranker>>,
    ) -> Self {
        let search_service = HybridSearchService::new(
            store.clone(),
            tantivy.clone(),
            Default::default(),
            graph.clone(),
            reranker.clone(),
        );
        Self {
            store,
            search_service: Arc::new(search_service),
            chat_base_url: None,
            chat_api_key: None,
            chat_model: None,
            agent: None,
        }
    }

    #[must_use]
    pub fn with_chat_overrides(
        mut self,
        base_url: Option<String>,
        api_key: Option<String>,
        model: Option<String>,
    ) -> Self {
        self.chat_base_url = base_url;
        self.chat_api_key = api_key;
        self.chat_model = model;
        self
    }

    /// Attach a reusable, already-initialized `DialecticalAgent` so that
    /// `synthesize: true` requests reuse it instead of re-instantiating per call.
    #[must_use]
    pub fn with_agent(mut self, agent: DialecticalAgent) -> Self {
        self.agent = Some(Arc::new(agent));
        self
    }

    #[must_use]
    pub fn from_app_state(state: &crate::server::state::AppState) -> Self {
        Self {
            store: Arc::clone(&state.store),
            search_service: Arc::clone(&state.search_service),
            chat_base_url: Some(state.chat_base_url.clone()),
            chat_api_key: state.chat_api_key.clone(),
            chat_model: Some(state.chat_model.clone()),
            agent: None,
        }
    }

    /// Primary JSON-RPC 2.0 message handler. Returns None for notifications.
    pub async fn handle_request(&self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
        if req.is_notification() {
            self.handle_notification(&req);
            return None;
        }

        let id = req.id.clone();
        let resp_result = match req.method.as_str() {
            "initialize" => self.handle_initialize(&req),
            "ping" => Ok(serde_json::json!({})),
            "tools/list" => self.handle_tools_list(&req),
            "tools/call" => self.handle_tools_call(req).await,
            other => Err(JsonRpcError::method_not_found(other)),
        };

        match resp_result {
            Ok(result) => Some(JsonRpcResponse::success(id, result)),
            Err(err) => Some(JsonRpcResponse::error(id, err)),
        }
    }

    fn handle_notification(&self, req: &JsonRpcRequest) {
        if req.method == "notifications/initialized" {
            tracing::info!("MCP client initialized notification received");
        } else {
            tracing::debug!("Unhandled MCP notification: {}", req.method);
        }
    }

    fn handle_initialize(
        &self,
        _req: &JsonRpcRequest,
    ) -> std::result::Result<serde_json::Value, JsonRpcError> {
        let result = McpInitializeResult {
            protocol_version: MCP_PROTOCOL_VERSION.to_string(),
            capabilities: McpServerCapabilities {
                tools: Some(McpToolsCapability {
                    list_changed: Some(false),
                }),
            },
            server_info: McpServerInfo {
                name: SERVER_NAME.to_string(),
                version: SERVER_VERSION.to_string(),
            },
        };
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }

    fn handle_tools_list(
        &self,
        _req: &JsonRpcRequest,
    ) -> std::result::Result<serde_json::Value, JsonRpcError> {
        let tools = list_all_tools();
        Ok(serde_json::json!({
            "tools": tools,
        }))
    }

    async fn handle_tools_call(
        &self,
        req: JsonRpcRequest,
    ) -> std::result::Result<serde_json::Value, JsonRpcError> {
        let params = req.params.ok_or_else(|| {
            JsonRpcError::invalid_params("Missing parameters object for tools/call")
        })?;

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError::invalid_params("Missing tool name in tools/call"))?;

        let arguments = params
            .get("arguments")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));

        let call_result = match name {
            "query_dialectical_principles" => {
                self.execute_query_dialectical_principles(arguments).await?
            }
            "verify_historical_citation" => {
                self.execute_verify_historical_citation(arguments).await?
            }
            unknown => McpCallToolResult::error(format!("Unknown tool: {unknown}")),
        };

        serde_json::to_value(call_result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }

    async fn execute_query_dialectical_principles(
        &self,
        args_val: serde_json::Value,
    ) -> std::result::Result<McpCallToolResult, JsonRpcError> {
        crate::mcp::tools::principles::execute_query_dialectical_principles(
            &self.search_service,
            self.agent.as_deref(),
            args_val,
        )
        .await
    }

    async fn execute_verify_historical_citation(
        &self,
        args_val: serde_json::Value,
    ) -> std::result::Result<McpCallToolResult, JsonRpcError> {
        crate::mcp::tools::citation::execute_verify_historical_citation(&self.store, args_val).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Document, DocumentMetadata, HistoricalPeriod};

    async fn create_test_dispatcher() -> McpDispatcher {
        let store = Arc::new(VectorStore::new_deterministic(64));
        let doc = Document {
            id: "doc1".to_string(),
            metadata: DocumentMetadata {
                title: "《反对本本主义》".to_string(),
                author: "毛泽东".to_string(),
                date: "1930-05-01".to_string(),
                period: "土地革命战争时期".to_string(),
                volume: "第一卷".to_string(),
                category: "哲学著作".to_string(),
                tags: vec!["调查研究".to_string(), "实事求是".to_string()],
                ..Default::default()
            },
            period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
            headnote: None,
            content: "没有调查，就没有发言权。你对于那个问题不能解决么？那末，你就去调查那个问题的现状和它的历史吧！".to_string(),
            footnotes: vec![],
            file_path: None,
        };
        store.index_document(&doc).await.expect("index doc");

        McpDispatcher::new(store, None, None, None)
    }

    #[tokio::test]
    async fn test_mcp_initialize() {
        let dispatcher = create_test_dispatcher().await;
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: "initialize".to_string(),
            params: None,
        };
        let resp = dispatcher
            .handle_request(req)
            .await
            .expect("should return response");
        assert_eq!(resp.id, Some(serde_json::json!(1)));
        assert!(resp.error.is_none());
        let result = resp.result.expect("result should exist");
        assert_eq!(result["protocolVersion"], "2024-11-05");
        assert_eq!(result["serverInfo"]["name"], "mao_agent");
    }

    #[tokio::test]
    async fn test_mcp_tools_list() {
        let dispatcher = create_test_dispatcher().await;
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(2)),
            method: "tools/list".to_string(),
            params: None,
        };
        let resp = dispatcher
            .handle_request(req)
            .await
            .expect("should return response");
        let result = resp.result.expect("result should exist");
        let tools = result["tools"].as_array().expect("tools array");
        assert_eq!(tools.len(), 2);
    }

    #[tokio::test]
    async fn test_mcp_unknown_method_returns_32601() {
        let dispatcher = create_test_dispatcher().await;
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(6)),
            method: "unknown/method".to_string(),
            params: None,
        };
        let resp = dispatcher
            .handle_request(req)
            .await
            .expect("should return response");
        let err = resp.error.expect("should be error");
        assert_eq!(err.code, -32601);
    }
}
