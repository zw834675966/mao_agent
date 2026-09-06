use std::time::Instant;

use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::mcp::dispatcher::McpDispatcher;
use crate::mcp::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::server::state::AppState;

/// Handles JSON-RPC 2.0 requests over HTTP POST (`/api/v1/mcp`).
///
/// Implements:
/// - JSON-RPC 2.0 specification conformance (including Parse Error on malformed JSON)
/// - Google SRE overload protection: verifies `ask_semaphore` concurrency before invoking LLM synthesis
/// - Metrics tracking (`mao_mcp_requests_total`, `mao_mcp_latency_ms_*`)
pub async fn handle_mcp(
    State(state): State<AppState>,
    payload: Result<Json<JsonRpcRequest>, JsonRejection>,
) -> Response {
    let started = Instant::now();

    let req = match payload {
        Ok(Json(req)) => req,
        Err(err) => {
            state.metrics.record_mcp(started, true);
            return (
                StatusCode::OK,
                Json(JsonRpcResponse::error(
                    None,
                    JsonRpcError::parse_error(err.to_string()),
                )),
            )
                .into_response();
        }
    };

    // Google SRE overload protection:
    // If the tool call requests dialectical synthesis (`synthesize: true`),
    // check and acquire a permit from `ask_semaphore`.
    let is_synthesize = if req.method == "tools/call" {
        if let Some(params) = &req.params {
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            if name == "query_dialectical_principles" {
                params
                    .get("arguments")
                    .and_then(|a| a.get("synthesize"))
                    .and_then(|s| s.as_bool())
                    .unwrap_or(false)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    let _permit = if is_synthesize {
        match state.ask_semaphore.clone().try_acquire_owned() {
            Ok(permit) => Some(permit),
            Err(_) => {
                state.metrics.record_mcp(started, true);
                return (
                    StatusCode::OK,
                    Json(JsonRpcResponse::error(
                        req.id,
                        JsonRpcError::resource_exhausted(
                            "Dialectical LLM synthesis concurrency limit exceeded",
                        ),
                    )),
                )
                    .into_response();
            }
        }
    } else {
        None
    };

    let dispatcher = McpDispatcher::from_app_state(&state);
    let maybe_resp = dispatcher.handle_request(req).await;

    let is_err = maybe_resp.as_ref().is_some_and(|r| r.error.is_some());
    state.metrics.record_mcp(started, is_err);

    match maybe_resp {
        Some(resp) => (StatusCode::OK, Json(resp)).into_response(),
        None => StatusCode::NO_CONTENT.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use serde_json::json;
    use std::sync::Arc;

    use crate::index::HybridSearchCoordinator;
    use crate::vector::VectorStore;

    fn test_state(max_concurrent_asks: usize) -> AppState {
        let store = Arc::new(VectorStore::new_deterministic(64));
        let metrics = crate::server::metrics::HttpMetrics::new();
        AppState::with_ops(
            store,
            None,
            HybridSearchCoordinator::default(),
            None,
            "http://127.0.0.1:9999".to_string(),
            None,
            "command-r7b-12-2024".to_string(),
            metrics,
            None,
            max_concurrent_asks,
        )
    }

    #[tokio::test]
    async fn test_mcp_initialize_and_tools_list() {
        let state = test_state(4);

        // 1. initialize
        let init_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "dsh-test", "version": "1.0.0" }
            })),
        };

        let resp = handle_mcp(State(state.clone()), Ok(Json(init_req))).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let rpc_resp: JsonRpcResponse = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(rpc_resp.id, Some(json!(1)));
        let result = rpc_resp.result.unwrap();
        assert_eq!(result["serverInfo"]["name"], "mao_agent");

        // 2. tools/list
        let list_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            method: "tools/list".to_string(),
            params: None,
        };
        let resp2 = handle_mcp(State(state.clone()), Ok(Json(list_req))).await;
        assert_eq!(resp2.status(), StatusCode::OK);
        let body_bytes2 = to_bytes(resp2.into_body(), usize::MAX).await.unwrap();
        let rpc_resp2: JsonRpcResponse = serde_json::from_slice(&body_bytes2).unwrap();
        let tools = rpc_resp2.result.unwrap()["tools"]
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(tools.len(), 2);
    }

    #[tokio::test]
    async fn test_mcp_overload_protection_on_synthesis() {
        // Limit max concurrent asks to 0, which gets clamped to 1 in with_ops
        let state = test_state(1);
        // Exhaust the only permit
        let _permit = state.try_acquire_ask().expect("acquired first slot");

        let synth_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(42)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "query_dialectical_principles",
                "arguments": {
                    "topic": "实践是检验真理的唯一标准",
                    "synthesize": true
                }
            })),
        };

        let resp = handle_mcp(State(state), Ok(Json(synth_req))).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let rpc_resp: JsonRpcResponse = serde_json::from_slice(&body_bytes).unwrap();
        let err = rpc_resp.error.expect("expected resource exhausted error");
        assert_eq!(err.code, JsonRpcError::RESOURCE_EXHAUSTED);
        assert!(err.message.contains("concurrency limit exceeded"));
    }
}
