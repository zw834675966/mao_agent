//! `query_dialectical_principles` tool implementation.
//!
//! Hybrid RRF + Knowledge Graph triad retrieval with optional DialecticalAgent
//! synthesis. The agent is supplied by the caller (cached across requests) so
//! that `synthesize: true` never re-instantiates a `DialecticalAgent` or a
//! `reqwest::Client` per request.

use crate::corpus::Domain;
use crate::index::HybridSearchService;
use crate::mcp::types::{JsonRpcError, McpCallToolResult, QueryDialecticalArgs};
use crate::model::{HistoricalPeriod, VectorFilter};

const MAX_QUERY_CHARS: usize = 4000;

fn reject_if_too_long(
    label: &str,
    text: &str,
    max: usize,
) -> std::result::Result<(), JsonRpcError> {
    if text.chars().count() > max {
        Err(JsonRpcError::invalid_params(format!(
            "{label} exceeds maximum length of {max} characters"
        )))
    } else {
        Ok(())
    }
}

/// Execute the `query_dialectical_principles` tool.
///
/// `cached_agent` is an already-initialized, reusable `DialecticalAgent` owned
/// by the dispatcher. When `synthesize: true` and no agent is available, the
/// synthesis gracefully degrades to a placeholder message instead of crashing.
pub async fn execute_query_dialectical_principles(
    search_service: &HybridSearchService,
    cached_agent: Option<&crate::agent::DialecticalAgent>,
    args_val: serde_json::Value,
) -> Result<McpCallToolResult, JsonRpcError> {
    let args: QueryDialecticalArgs = serde_json::from_value(args_val)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid arguments: {e}")))?;

    if args.query.trim().is_empty() {
        return Err(JsonRpcError::invalid_params("query must not be empty"));
    }
    reject_if_too_long("query", &args.query, MAX_QUERY_CHARS)?;

    let top_k = args.top_k.unwrap_or(3).clamp(1, 20);

    let mut filter = None;
    if args.period.is_some() || args.volume.is_some() || args.domain.is_some() {
        let mut f = VectorFilter::new();
        if let Some(ref p) = args.period {
            f.period = Some(HistoricalPeriod::from_str_or_date(p));
        }
        if let Some(ref v) = args.volume {
            f.volume = Some(v.clone());
        }
        if let Some(ref d) = args.domain {
            match Domain::parse(d) {
                Ok(parsed) => f = f.with_domain(parsed),
                Err(e) => tracing::warn!("ignoring invalid MCP domain '{d}': {e}"),
            }
        }
        filter = Some(f);
    }

    let final_hits = search_service
        .search_hybrid(&args.query, top_k, filter.as_ref(), false)
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Hybrid search failed: {e}")))?;

    // Optional Dialectical Agent Synthesis — reuse the caller-provided cached
    // agent. Never instantiate a new agent or reqwest::Client here.
    let synthesis_report = if args.synthesize == Some(true) {
        match cached_agent {
            Some(agent) => match agent.ask(&args.query, top_k, filter.as_ref()).await {
                Ok(ans) => Some(ans.content),
                Err(e) => {
                    tracing::warn!("Dialectical synthesis warning: {e}");
                    Some(format!("(Synthesis unavailable: {e})"))
                }
            },
            None => Some("(Synthesis unavailable: DialecticalAgent not configured)".to_string()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Document, DocumentMetadata};

    async fn create_test_search_service() -> HybridSearchService {
        use crate::index::HybridSearchCoordinator;
        use crate::vector::VectorStore;
        use std::sync::Arc;

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

        HybridSearchService::new(
            store,
            None,
            HybridSearchCoordinator::default(),
            None,
            None,
        )
    }

    #[tokio::test]
    async fn test_query_principles_execution() {
        let service = create_test_search_service().await;
        let args = serde_json::json!({
            "query": "调查研究",
            "top_k": 1,
            "synthesize": false
        });
        let result = execute_query_dialectical_principles(&service, None, args)
            .await
            .expect("should succeed");
        let text_str = match &result.content[0] {
            crate::mcp::types::McpContent::Text { text } => text,
        };
        assert!(text_str.contains("没有调查，就没有发言权"));
    }

    #[tokio::test]
    async fn test_query_rejects_oversized_query() {
        let service = create_test_search_service().await;
        let args = serde_json::json!({
            "query": "调".repeat(MAX_QUERY_CHARS + 1)
        });
        let err = execute_query_dialectical_principles(&service, None, args)
            .await
            .expect_err("should be error");
        assert_eq!(err.code, JsonRpcError::INVALID_PARAMS);
        assert!(err.message.contains("exceeds maximum length"));
    }
}
