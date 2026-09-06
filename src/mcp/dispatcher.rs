//! Transport-agnostic MCP dispatcher implementing JSON-RPC 2.0 tool execution.
//!
//! Exposes:
//! - `query_dialectical_principles`: Hybrid RRF + Knowledge Graph triad retrieval + optional DialecticalAgent synthesis.
//! - `verify_historical_citation`: Character-level citation verification with automatic local corpus fallback.

use std::sync::Arc;

use crate::agent::{CitationVerifier, DialecticalAgent};
use crate::graph::{GraphStore, ResolvedGraphChunk, union_graph_bonus};
use crate::index::{FullTextIndex, HybridSearchCoordinator};
use crate::mcp::types::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, MCP_PROTOCOL_VERSION, McpCallToolResult,
    McpInitializeResult, McpServerCapabilities, McpServerInfo, McpToolsCapability,
    QueryDialecticalArgs, SERVER_NAME, SERVER_VERSION, VerifyCitationArgs, list_all_tools,
};
use crate::model::{HistoricalPeriod, VectorFilter};
use crate::rerank::{Reranker, rerank_or_fallback};
use crate::vector::VectorStore;

#[derive(Clone)]
pub struct McpDispatcher {
    store: Arc<VectorStore>,
    tantivy: Option<Arc<FullTextIndex>>,
    graph: Option<Arc<GraphStore>>,
    reranker: Option<Arc<dyn Reranker>>,
    hybrid: Arc<HybridSearchCoordinator>,
    chat_base_url: Option<String>,
    chat_api_key: Option<String>,
    chat_model: Option<String>,
}

impl McpDispatcher {
    pub fn new(
        store: Arc<VectorStore>,
        tantivy: Option<Arc<FullTextIndex>>,
        graph: Option<Arc<GraphStore>>,
        reranker: Option<Arc<dyn Reranker>>,
    ) -> Self {
        Self {
            store,
            tantivy,
            graph,
            reranker,
            hybrid: Arc::new(HybridSearchCoordinator::default()),
            chat_base_url: None,
            chat_api_key: None,
            chat_model: None,
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

    #[must_use]
    pub fn from_app_state(state: &crate::server::state::AppState) -> Self {
        Self {
            store: Arc::clone(&state.store),
            tantivy: state.tantivy.clone(),
            graph: state.graph.clone(),
            reranker: state.reranker.clone(),
            hybrid: Arc::clone(&state.hybrid),
            chat_base_url: Some(state.chat_base_url.clone()),
            chat_api_key: state.chat_api_key.clone(),
            chat_model: Some(state.chat_model.clone()),
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
        let args: QueryDialecticalArgs = serde_json::from_value(args_val)
            .map_err(|e| JsonRpcError::invalid_params(format!("Invalid arguments: {e}")))?;

        if args.query.trim().is_empty() {
            return Err(JsonRpcError::invalid_params("query must not be empty"));
        }

        let top_k = args.top_k.unwrap_or(3).clamp(1, 20);

        let mut filter = None;
        if args.period.is_some() || args.volume.is_some() {
            let mut f = VectorFilter::new();
            if let Some(ref p) = args.period {
                f.period = Some(HistoricalPeriod::from_str_or_date(p));
            }
            if let Some(ref v) = args.volume {
                f.volume = Some(v.clone());
            }
            filter = Some(f);
        }

        // 1. Vector Search
        let vec_results = self
            .store
            .search(&args.query, top_k * 2, filter.as_ref())
            .await
            .map_err(|e| JsonRpcError::internal_error(format!("Vector search failed: {e}")))?;

        // 2. BM25 Search
        let bm25_results = if let Some(ref ft) = self.tantivy {
            match ft.search(&args.query, top_k * 2, filter.as_ref()) {
                Ok(res) => res,
                Err(e) => {
                    tracing::warn!("BM25 search warning in MCP: {e}");
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        // 3. Hybrid RRF fusion
        let fused = self.hybrid.fuse(vec_results, bm25_results, top_k * 2);

        // 4. Knowledge graph expansion
        let fused = if let Some(ref graph) = self.graph {
            let hits = graph.expand(&args.query, 2);
            let mut resolved = Vec::new();
            for hit in &hits {
                for r in &hit.source_refs {
                    for chunk in self.store.chunks_matching_ref(r).await {
                        resolved.push(ResolvedGraphChunk {
                            chunk,
                            paths: hit.paths.clone(),
                        });
                    }
                }
            }
            union_graph_bonus(
                fused,
                &resolved,
                if self.reranker.is_none() {
                    Some(top_k)
                } else {
                    None
                },
            )
        } else {
            fused
        };

        // 5. Optional Reranking
        let final_hits =
            rerank_or_fallback(fused, self.reranker.as_deref(), &args.query, top_k).await;

        // 6. Optional Dialectical Agent Synthesis
        let synthesis_report = if args.synthesize == Some(true) {
            let mut agent = DialecticalAgent::new(
                Arc::clone(&self.store),
                self.tantivy.clone(),
                self.chat_base_url.clone(),
                self.chat_api_key.clone(),
                self.chat_model.clone(),
                self.reranker.clone(),
            );
            if let Some(ref g) = self.graph {
                agent = agent.with_graph(Arc::clone(g));
            }
            match agent.ask(&args.query, top_k, filter.as_ref()).await {
                Ok(ans) => Some(ans.content),
                Err(e) => {
                    tracing::warn!("Dialectical synthesis warning: {e}");
                    Some(format!("(Synthesis unavailable: {e})"))
                }
            }
        } else {
            None
        };

        let structured_hits: Vec<serde_json::Value> = final_hits
            .into_iter()
            .map(|h| {
                serde_json::json!({
                    "chunk_id": h.chunk_id,
                    "doc_title": h.chunk.doc_title,
                    "period": h.chunk.period.as_str(),
                    "volume": h.chunk.volume,
                    "section_path": h.chunk.section_path,
                    "text": h.chunk.raw_text,
                    "score": h.rerank_score.or(h.vector_score).unwrap_or(h.rrf_score),
                    "graph_paths": h.graph_paths,
                })
            })
            .collect();

        let output = serde_json::json!({
            "query": args.query,
            "hits_count": structured_hits.len(),
            "principles": structured_hits,
            "synthesis_report": synthesis_report,
        });

        let formatted = serde_json::to_string_pretty(&output)
            .map_err(|e| JsonRpcError::internal_error(e.to_string()))?;

        Ok(McpCallToolResult::text(formatted))
    }

    async fn execute_verify_historical_citation(
        &self,
        args_val: serde_json::Value,
    ) -> std::result::Result<McpCallToolResult, JsonRpcError> {
        let args: VerifyCitationArgs = serde_json::from_value(args_val)
            .map_err(|e| JsonRpcError::invalid_params(format!("Invalid arguments: {e}")))?;

        if args.quote.trim().is_empty() {
            return Err(JsonRpcError::invalid_params("quote must not be empty"));
        }
        if args.claimed_title.trim().is_empty() {
            return Err(JsonRpcError::invalid_params(
                "claimed_title must not be empty",
            ));
        }

        let min_confidence = args.min_confidence.unwrap_or(0.85).clamp(0.0, 1.0) as f32;

        // Resolve context chunks: user-provided or auto-retrieved from local store
        let (chunks, auto_retrieved) = match args.context_chunks {
            Some(raw_strings) if !raw_strings.is_empty() => {
                let synthesized = raw_strings
                    .into_iter()
                    .enumerate()
                    .map(|(i, s)| crate::model::DocumentChunk {
                        chunk_id: format!("mcp_ctx_{i}"),
                        doc_id: "mcp_doc".to_string(),
                        doc_title: args.claimed_title.clone(),
                        author: "毛泽东".to_string(),
                        period: HistoricalPeriod::Unknown,
                        date: "未知".to_string(),
                        volume: "未知".to_string(),
                        category: "文献".to_string(),
                        tags: vec![],
                        chunk_index: i,
                        total_chunks: 1,
                        char_count: s.chars().count(),
                        raw_text: s.clone(),
                        contextualized_text: s,
                        section_path: vec![],
                    })
                    .collect();
                (synthesized, false)
            }
            _ => {
                // Auto-retrieve by claimed title from VectorStore
                let matching = self.store.chunks_matching_title(&args.claimed_title).await;
                if matching.is_empty() {
                    // Claimed document does not exist in corpus
                    let not_found_report = serde_json::json!({
                        "is_valid": false,
                        "confidence": 0.0,
                        "verdict": "DocNotFound",
                        "matched_segment": null,
                        "mismatch_reason": format!("Claimed document '{}' was not found in the local historical corpus", args.claimed_title),
                        "source_title": args.claimed_title,
                        "auto_retrieved": true,
                    });
                    let formatted = serde_json::to_string_pretty(&not_found_report)
                        .map_err(|e| JsonRpcError::internal_error(e.to_string()))?;
                    return Ok(McpCallToolResult::text(formatted));
                }
                (matching, true)
            }
        };

        let verifier = CitationVerifier::new(min_confidence, 6);
        let report = verifier.verify_quote(&args.quote, &args.claimed_title, &chunks);

        let verdict = if report.is_verified {
            if report.match_confidence >= 0.999 {
                "ExactMatch"
            } else {
                "FuzzyMatch"
            }
        } else {
            "UnverifiedOrFabricated"
        };

        let result_json = serde_json::json!({
            "is_valid": report.is_verified,
            "confidence": report.match_confidence,
            "verdict": verdict,
            "matched_segment": report.matched_snippet,
            "matched_chunk_id": report.matched_chunk_id,
            "warning": report.warning,
            "source_title": report.claimed_doc_title,
            "auto_retrieved": auto_retrieved,
        });

        let formatted = serde_json::to_string_pretty(&result_json)
            .map_err(|e| JsonRpcError::internal_error(e.to_string()))?;

        Ok(McpCallToolResult::text(formatted))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Document, DocumentMetadata};

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
    async fn test_mcp_query_principles_execution() {
        let dispatcher = create_test_dispatcher().await;
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(3)),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "query_dialectical_principles",
                "arguments": {
                    "query": "调查研究",
                    "top_k": 1,
                    "synthesize": false
                }
            })),
        };
        let resp = dispatcher
            .handle_request(req)
            .await
            .expect("should return response");
        assert!(resp.error.is_none());
        let result = resp.result.expect("result should exist");
        let content = result["content"].as_array().expect("content array");
        assert_eq!(content.len(), 1);
        let text = content[0]["text"].as_str().expect("text");
        assert!(text.contains("没有调查，就没有发言权"));
    }

    #[tokio::test]
    async fn test_mcp_citation_auto_retrieval_success() {
        let dispatcher = create_test_dispatcher().await;
        // Without passing context_chunks, automatically look up 《反对本本主义》
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(4)),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "verify_historical_citation",
                "arguments": {
                    "quote": "没有调查，就没有发言权",
                    "claimed_title": "反对本本主义"
                }
            })),
        };
        let resp = dispatcher
            .handle_request(req)
            .await
            .expect("should return response");
        assert!(resp.error.is_none());
        let result = resp.result.expect("result should exist");
        let text = result["content"][0]["text"].as_str().expect("text");
        let report: serde_json::Value = serde_json::from_str(text).expect("parse report json");
        assert_eq!(report["is_valid"], true);
        assert_eq!(report["verdict"], "ExactMatch");
        assert_eq!(report["auto_retrieved"], true);
    }

    #[tokio::test]
    async fn test_mcp_citation_auto_retrieval_doc_not_found() {
        let dispatcher = create_test_dispatcher().await;
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(5)),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "verify_historical_citation",
                "arguments": {
                    "quote": "天地不仁，以万物为刍狗",
                    "claimed_title": "道德经"
                }
            })),
        };
        let resp = dispatcher
            .handle_request(req)
            .await
            .expect("should return response");
        assert!(resp.error.is_none());
        let result = resp.result.expect("result should exist");
        let text = result["content"][0]["text"].as_str().expect("text");
        let report: serde_json::Value = serde_json::from_str(text).expect("parse report json");
        assert_eq!(report["is_valid"], false);
        assert_eq!(report["verdict"], "DocNotFound");
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
