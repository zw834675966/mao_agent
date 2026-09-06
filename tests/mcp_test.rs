//! Hermetic integration test suite for the Model Context Protocol (MCP 2024-11-05) implementation.
//!
//! Tests:
//! - JSON-RPC 2.0 handshake (`initialize`, `tools/list`)
//! - Strict schema conformance for DSH / frontier agents
//! - `query_dialectical_principles` with deterministic embedder
//! - Knowledge Graph contradiction triad expansion
//! - `verify_historical_citation` grounded vs adversarial rejection
//! - Automated corpus retrieval when `context_chunks` is omitted
//! - Error handling (unknown methods, invalid params, missing arguments)
//! - HTTP transport over Axum router (`/api/v1/mcp` and `/mcp`)

use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::http::Request;
use serde_json::{Value, json};
use tower::ServiceExt;

use mao_agent::corpus::ChineseSemanticChunker;
use mao_agent::graph::GraphStore;
use mao_agent::index::{FullTextIndex, HybridSearchCoordinator};
use mao_agent::mcp::dispatcher::McpDispatcher;
use mao_agent::mcp::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, MCP_PROTOCOL_VERSION};
use mao_agent::model::{Document, DocumentMetadata, HistoricalPeriod};
use mao_agent::server::build_router;
use mao_agent::server::state::AppState;
use mao_agent::vector::VectorStore;

fn sample_doc_mdl() -> Document {
    Document {
        id: "doc_mdl".to_string(),
        metadata: DocumentMetadata {
            title: "矛盾论".to_string(),
            author: "毛泽东".to_string(),
            date: "1937-08".to_string(),
            period: "土地革命战争时期".to_string(),
            volume: "毛泽东选集第一卷".to_string(),
            category: "哲学".to_string(),
            tags: vec!["唯物辩证法".to_string(), "矛盾".to_string()],
            ..Default::default()
        },
        period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
        headnote: None,
        content: "事物的矛盾法则，即对立统一的法则，是唯物辩证法的最根本的法则。主要矛盾和主要矛盾方面的转化决定了事物的性质。".to_string(),
        footnotes: vec![],
        file_path: None,
    }
}

fn sample_doc_cjz() -> Document {
    Document {
        id: "doc_cjz".to_string(),
        metadata: DocumentMetadata {
            title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            date: "1938-05".to_string(),
            period: "抗日战争时期".to_string(),
            volume: "毛泽东选集第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec!["持久战".to_string()],
            ..Default::default()
        },
        period_enum: HistoricalPeriod::WarOfResistance,
        headnote: None,
        content:
            "中日战争是持久战，最后的胜利是中国的。战争的三个阶段：战略防御、战略相持、战略反攻。"
                .to_string(),
        footnotes: vec![],
        file_path: None,
    }
}

const GRAPH_FIXTURE: &str = r#"
{
  "entities": [
    {
      "id": "ent:principal_contradiction",
      "name": "主要矛盾",
      "aliases": ["主要矛盾", "principal contradiction"],
      "domain": "mao",
      "source_refs": [{"doc_title": "矛盾论"}]
    },
    {
      "id": "ent:unity_of_opposites",
      "name": "对立统一",
      "aliases": ["对立统一规律"],
      "domain": "mao",
      "source_refs": [{"doc_title": "矛盾论"}]
    }
  ],
  "relationships": [
    {
      "id": "rel:contradiction-unity",
      "source": "ent:principal_contradiction",
      "target": "ent:unity_of_opposites",
      "rel_type": "dialectical_triad",
      "weight": 1.0,
      "source_refs": [{"doc_title": "矛盾论"}]
    }
  ]
}
"#;

async fn setup_test_context() -> (
    Arc<VectorStore>,
    Option<Arc<FullTextIndex>>,
    Option<Arc<GraphStore>>,
    AppState,
) {
    let docs = [sample_doc_mdl(), sample_doc_cjz()];
    let store = Arc::new(VectorStore::new_deterministic(128));
    for d in &docs {
        store.index_document(d).await.unwrap();
    }

    let chunker = ChineseSemanticChunker::new(Default::default());
    let ft = FullTextIndex::new_in_ram().unwrap();
    for d in &docs {
        ft.insert_batch(&chunker.chunk_document(d)).unwrap();
    }
    let ft_arc = Arc::new(ft);

    let graph = Arc::new(GraphStore::from_json_str(GRAPH_FIXTURE).unwrap());

    let state = AppState::new(
        Arc::clone(&store),
        Some(Arc::clone(&ft_arc)),
        HybridSearchCoordinator::default(),
        None,
        "http://127.0.0.1:9999".to_string(),
        None,
        "command-r7b-12-2024".to_string(),
    )
    .with_graph(Arc::clone(&graph));

    (store, Some(ft_arc), Some(graph), state)
}

#[tokio::test]
async fn test_mcp_handshake_and_capabilities() {
    let (store, ft, graph, _) = setup_test_context().await;
    let dispatcher = McpDispatcher::new(store, ft, graph, None);

    // 1. Handshake (initialize)
    let init_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "dsh-agent",
                "version": "1.0.0"
            }
        })),
    };

    let resp = dispatcher
        .handle_request(init_req)
        .await
        .expect("response expected");
    assert_eq!(resp.jsonrpc, "2.0");
    assert_eq!(resp.id, Some(json!(1)));
    assert!(resp.error.is_none());

    let result = resp.result.expect("result expected");
    assert_eq!(result["protocolVersion"], MCP_PROTOCOL_VERSION);
    assert_eq!(result["serverInfo"]["name"], "mao_agent");
    assert_eq!(result["capabilities"]["tools"]["listChanged"], false);

    // 2. tools/list
    let list_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/list".to_string(),
        params: None,
    };

    let resp2 = dispatcher
        .handle_request(list_req)
        .await
        .expect("response expected");
    assert_eq!(resp2.id, Some(json!(2)));
    let result2 = resp2.result.expect("tools result expected");
    let tools = result2["tools"].as_array().expect("tools array");
    assert_eq!(tools.len(), 2);

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"query_dialectical_principles"));
    assert!(tool_names.contains(&"verify_historical_citation"));

    // Verify root schema strictness: "type": "object" at top level
    for tool in tools {
        let schema = &tool["inputSchema"];
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].is_object());
        // Verify no property-level "required: true"
        for (_, prop_val) in schema["properties"].as_object().unwrap() {
            assert!(prop_val.get("required").is_none());
        }
    }
}

#[tokio::test]
async fn test_mcp_query_dialectical_principles() {
    let (store, ft, graph, _) = setup_test_context().await;
    let dispatcher = McpDispatcher::new(store, ft, graph, None);

    let call_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(10)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "query_dialectical_principles",
            "arguments": {
                "query": "矛盾的法则与转化",
                "top_k": 3
            }
        })),
    };

    let resp = dispatcher
        .handle_request(call_req)
        .await
        .expect("response expected");
    assert_eq!(resp.id, Some(json!(10)));
    assert!(resp.error.is_none());

    let result = resp.result.expect("result expected");
    let content = result["content"].as_array().expect("content array");
    assert!(!content.is_empty());
    assert_eq!(content[0]["type"], "text");

    let text = content[0]["text"].as_str().expect("text string");
    let parsed: Value = serde_json::from_str(text).expect("valid json in text field");
    assert_eq!(parsed["query"], "矛盾的法则与转化");
    let principles = parsed["principles"].as_array().expect("principles list");
    assert!(!principles.is_empty());
    assert_eq!(principles[0]["doc_title"], "矛盾论");
}

#[tokio::test]
async fn test_mcp_query_with_graph_expansion() {
    let (store, ft, graph, _) = setup_test_context().await;
    let dispatcher = McpDispatcher::new(store, ft, graph, None);

    let call_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(11)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "query_dialectical_principles",
            "arguments": {
                "query": "主要矛盾",
                "top_k": 2
            }
        })),
    };

    let resp = dispatcher
        .handle_request(call_req)
        .await
        .expect("response expected");
    assert!(resp.error.is_none());
    let result = resp.result.expect("result expected");
    let text = result["content"][0]["text"].as_str().unwrap();
    let parsed: Value = serde_json::from_str(text).unwrap();

    let principles = parsed["principles"].as_array().expect("principles array");
    assert!(!principles.is_empty());
    // The graph expansion links "主要矛盾" to "矛盾论"
    assert_eq!(principles[0]["doc_title"], "矛盾论");
}

#[tokio::test]
async fn test_mcp_verify_citation_grounded() {
    let (store, ft, graph, _) = setup_test_context().await;
    let dispatcher = McpDispatcher::new(store, ft, graph, None);

    let call_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(20)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "verify_historical_citation",
            "arguments": {
                "quote": "对立统一的法则，是唯物辩证法的最根本的法则。",
                "claimed_title": "矛盾论",
                "context_chunks": [
                    "事物的矛盾法则，即对立统一的法则，是唯物辩证法的最根本的法则。主要矛盾和主要矛盾方面的转化决定了事物的性质。"
                ]
            }
        })),
    };

    let resp = dispatcher
        .handle_request(call_req)
        .await
        .expect("response expected");
    assert!(resp.error.is_none());
    let result = resp.result.expect("result expected");

    let text = result["content"][0]["text"].as_str().unwrap();
    let parsed: Value = serde_json::from_str(text).unwrap();
    assert_eq!(parsed["is_valid"], true);
    assert_eq!(parsed["verdict"], "ExactMatch");
    assert!(parsed["confidence"].as_f64().unwrap() >= 0.99);
}

#[tokio::test]
async fn test_mcp_verify_citation_adversarial_rejection() {
    let (store, ft, graph, _) = setup_test_context().await;
    let dispatcher = McpDispatcher::new(store, ft, graph, None);

    // Completely fabricated quote
    let call_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(21)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "verify_historical_citation",
            "arguments": {
                "quote": "任何工作只要有恒心一天之内必然可以彻底完满成功。",
                "claimed_title": "矛盾论",
                "context_chunks": [
                    "事物的矛盾法则，即对立统一的法则，是唯物辩证法的最根本的法则。"
                ]
            }
        })),
    };

    let resp = dispatcher
        .handle_request(call_req)
        .await
        .expect("response expected");
    assert!(resp.error.is_none());
    let result = resp.result.expect("result expected");

    let text = result["content"][0]["text"].as_str().unwrap();
    let parsed: Value = serde_json::from_str(text).unwrap();
    assert_eq!(parsed["is_valid"], false);
    assert_eq!(parsed["verdict"], "UnverifiedOrFabricated");
}

#[tokio::test]
async fn test_mcp_verify_citation_auto_retrieval() {
    let (store, ft, graph, _) = setup_test_context().await;
    let dispatcher = McpDispatcher::new(store, ft, graph, None);

    // OMIT context_chunks completely: dispatcher automatically retrieves chunks for claimed_title from VectorStore
    let call_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(22)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "verify_historical_citation",
            "arguments": {
                "quote": "战争的三个阶段：战略防御、战略相持、战略反攻。",
                "claimed_title": "论持久战"
            }
        })),
    };

    let resp = dispatcher
        .handle_request(call_req)
        .await
        .expect("response expected");
    assert!(resp.error.is_none());
    let result = resp.result.expect("result expected");

    let text = result["content"][0]["text"].as_str().unwrap();
    let parsed: Value = serde_json::from_str(text).unwrap();
    assert_eq!(parsed["is_valid"], true);
    assert_eq!(parsed["auto_retrieved"], true);
    assert_eq!(parsed["source_title"], "论持久战");
}

#[tokio::test]
async fn test_mcp_error_handling() {
    let (store, ft, graph, _) = setup_test_context().await;
    let dispatcher = McpDispatcher::new(store, ft, graph, None);

    // 1. Unknown method
    let bad_method_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(99)),
        method: "unknown/method".to_string(),
        params: None,
    };
    let resp = dispatcher.handle_request(bad_method_req).await.unwrap();
    let err = resp.error.unwrap();
    assert_eq!(err.code, JsonRpcError::METHOD_NOT_FOUND);

    // 2. Unknown tool name
    let bad_tool_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(100)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "non_existent_tool",
            "arguments": {}
        })),
    };
    let resp2 = dispatcher.handle_request(bad_tool_req).await.unwrap();
    let result2 = resp2.result.unwrap();
    assert_eq!(result2["isError"], true);
    assert!(
        result2["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("Unknown tool")
    );

    // 3. Missing required argument in query_dialectical_principles
    let missing_arg_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(101)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "query_dialectical_principles",
            "arguments": {
                "top_k": 3
            }
        })),
    };
    let resp3 = dispatcher.handle_request(missing_arg_req).await.unwrap();
    let err3 = resp3.error.unwrap();
    assert_eq!(err3.code, JsonRpcError::INVALID_PARAMS);
    assert!(err3.message.contains("missing field `query`"));
}

#[tokio::test]
async fn test_mcp_http_endpoint() {
    let (_, _, _, state) = setup_test_context().await;
    let app = build_router(state);

    // Send HTTP POST to /api/v1/mcp
    let init_payload = json!({
        "jsonrpc": "2.0",
        "id": 500,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "dsh-http-test", "version": "1.0.0" }
        }
    });

    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/mcp")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&init_payload).unwrap()))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), axum::http::StatusCode::OK);

    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let rpc_resp: JsonRpcResponse = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(rpc_resp.id, Some(json!(500)));
    assert_eq!(rpc_resp.result.unwrap()["serverInfo"]["name"], "mao_agent");

    // Also test alias route /mcp
    let tools_payload = json!({
        "jsonrpc": "2.0",
        "id": 501,
        "method": "tools/list"
    });

    let req2 = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&tools_payload).unwrap()))
        .unwrap();

    let resp2 = app.oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), axum::http::StatusCode::OK);

    let bytes2 = to_bytes(resp2.into_body(), usize::MAX).await.unwrap();
    let rpc_resp2: JsonRpcResponse = serde_json::from_slice(&bytes2).unwrap();
    assert_eq!(rpc_resp2.id, Some(json!(501)));
    let tools = rpc_resp2.result.unwrap()["tools"]
        .as_array()
        .unwrap()
        .clone();
    assert_eq!(tools.len(), 2);
}
