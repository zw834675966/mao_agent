//! `verify_historical_citation` tool implementation.
//!
//! Character-level citation verification with automatic local corpus fallback.
//! The corpus is the only authoritative verification source; caller-supplied
//! `context_chunks` are never used for self-attestation.

use crate::agent::CitationVerifier;
use crate::mcp::types::{JsonRpcError, McpCallToolResult, VerifyCitationArgs};

const MAX_QUOTE_CHARS: usize = 4000;
const MAX_TITLE_CHARS: usize = 200;

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

/// Execute the `verify_historical_citation` tool against the local corpus.
pub async fn execute_verify_historical_citation(
    store: &crate::vector::VectorStore,
    args_val: serde_json::Value,
) -> Result<crate::mcp::types::McpCallToolResult, crate::mcp::types::JsonRpcError> {
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
    reject_if_too_long("quote", &args.quote, MAX_QUOTE_CHARS)?;
    reject_if_too_long("claimed_title", &args.claimed_title, MAX_TITLE_CHARS)?;

    let min_confidence = args.min_confidence.unwrap_or(0.85).clamp(0.0, 1.0) as f32;

    // Corpus is the only grounding source. Caller-supplied `context_chunks`
    // cannot self-attest a quote (plan risk: 引文反查跨篇混淆).
    let matching = store.chunks_matching_title(&args.claimed_title).await;
    if matching.is_empty() {
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

    let verifier = CitationVerifier::new(min_confidence, 6);
    let report = verifier.verify_quote(&args.quote, &args.claimed_title, &matching);

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
        "auto_retrieved": true,
    });

    let formatted = serde_json::to_string_pretty(&result_json)
        .map_err(|e| JsonRpcError::internal_error(e.to_string()))?;

    Ok(McpCallToolResult::text(formatted))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::types::McpContent;
    use crate::model::{Document, DocumentMetadata, HistoricalPeriod};
    use crate::vector::VectorStore;
    use std::sync::Arc;

    async fn create_test_store() -> Arc<VectorStore> {
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
        store
    }

    fn extract_text(result: &McpCallToolResult) -> String {
        match &result.content[0] {
            McpContent::Text { text } => text.clone(),
        }
    }

    #[tokio::test]
    async fn test_citation_auto_retrieval_success() {
        let store = create_test_store().await;
        let args = serde_json::json!({
            "quote": "没有调查，就没有发言权",
            "claimed_title": "反对本本主义"
        });
        let result = execute_verify_historical_citation(&store, args)
            .await
            .expect("should succeed");
        let text = extract_text(&result);
        let report: serde_json::Value = serde_json::from_str(&text).expect("parse report json");
        assert_eq!(report["is_valid"], true);
        assert_eq!(report["verdict"], "ExactMatch");
        assert_eq!(report["auto_retrieved"], true);
    }

    #[tokio::test]
    async fn test_citation_auto_retrieval_doc_not_found() {
        let store = create_test_store().await;
        let args = serde_json::json!({
            "quote": "天地不仁，以万物为刍狗",
            "claimed_title": "道德经"
        });
        let result = execute_verify_historical_citation(&store, args)
            .await
            .expect("should succeed");
        let text = extract_text(&result);
        let report: serde_json::Value = serde_json::from_str(&text).expect("parse report json");
        assert_eq!(report["is_valid"], false);
        assert_eq!(report["verdict"], "DocNotFound");
    }

    #[tokio::test]
    async fn test_citation_rejects_self_attested_context_chunks() {
        let store = create_test_store().await;
        let args = serde_json::json!({
            "quote": "这是调用方塞进对照正文的伪造引言。",
            "claimed_title": "反对本本主义",
            "context_chunks": ["这是调用方塞进对照正文的伪造引言。"]
        });
        let result = execute_verify_historical_citation(&store, args)
            .await
            .expect("should succeed");
        let text = extract_text(&result);
        let report: serde_json::Value = serde_json::from_str(&text).expect("parse report json");
        assert_eq!(report["is_valid"], false);
        assert_eq!(report["verdict"], "UnverifiedOrFabricated");
    }

    #[tokio::test]
    async fn test_citation_doc_not_found_ignores_context_chunks() {
        let store = create_test_store().await;
        let args = serde_json::json!({
            "quote": "天地不仁，以万物为刍狗",
            "claimed_title": "道德经",
            "context_chunks": ["天地不仁，以万物为刍狗"]
        });
        let result = execute_verify_historical_citation(&store, args)
            .await
            .expect("should succeed");
        let text = extract_text(&result);
        let report: serde_json::Value = serde_json::from_str(&text).expect("parse report json");
        assert_eq!(report["is_valid"], false);
        assert_eq!(report["verdict"], "DocNotFound");
        assert_eq!(report["confidence"], 0.0);
    }
}
