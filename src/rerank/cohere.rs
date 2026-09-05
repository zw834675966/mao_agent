//! Cohere Rerank API client (`POST /v2/rerank`).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::error::{Result, VectorError};
use crate::index::HybridSearchResult;
use crate::rerank::Reranker;
use crate::retry::RetryPolicy;

/// Official Cohere v2 rerank endpoint (not the OpenAI-compat base).
pub const COHERE_RERANK_URL: &str = "https://api.cohere.com/v2/rerank";

/// Default Cohere rerank model.
pub const COHERE_RERANK_MODEL: &str = "rerank-v3.5";

/// Cohere HTTP reranker (`POST {base_url}` with bearer auth).
#[derive(Debug, Clone)]
pub struct CohereReranker {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
    retry: RetryPolicy,
}

#[derive(Serialize)]
struct RerankRequest<'a> {
    model: &'a str,
    query: &'a str,
    documents: Vec<&'a str>,
    top_n: usize,
}

#[derive(Deserialize)]
struct RerankResponse {
    results: Vec<RerankResultItem>,
}

#[derive(Deserialize)]
struct RerankResultItem {
    index: usize,
    relevance_score: f32,
}

impl CohereReranker {
    /// Create a Cohere reranker. `model` defaults to [`COHERE_RERANK_MODEL`];
    /// `base_url` defaults to [`COHERE_RERANK_URL`] (override for mock servers).
    pub fn new(api_key: String, model: Option<String>, base_url: Option<String>) -> Self {
        Self::with_retry(api_key, model, base_url, RetryPolicy::cohere_http())
    }

    pub fn with_retry(
        api_key: String,
        model: Option<String>,
        base_url: Option<String>,
        retry: RetryPolicy,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self {
            client,
            api_key,
            model: model.unwrap_or_else(|| COHERE_RERANK_MODEL.to_string()),
            base_url: base_url
                .unwrap_or_else(|| COHERE_RERANK_URL.to_string())
                .trim_end_matches('/')
                .to_string(),
            retry,
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[async_trait]
impl Reranker for CohereReranker {
    fn model_name(&self) -> &str {
        &self.model
    }

    async fn rerank(
        &self,
        query: &str,
        candidates: &[HybridSearchResult],
        top_k: usize,
    ) -> Result<Vec<HybridSearchResult>> {
        if candidates.is_empty() || top_k == 0 {
            return Ok(Vec::new());
        }

        let top_n = top_k.min(candidates.len());
        let documents: Vec<&str> = candidates
            .iter()
            .map(|c| c.chunk.raw_text.as_str())
            .collect();

        enum AttemptErr {
            Retryable(VectorError),
            Fatal(VectorError),
        }
        impl std::fmt::Display for AttemptErr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Retryable(e) | Self::Fatal(e) => write!(f, "{e}"),
                }
            }
        }

        let parsed = self
            .retry
            .run(
                |_| async {
                    let body = RerankRequest {
                        model: &self.model,
                        query,
                        documents: documents.clone(),
                        top_n,
                    };
                    let resp = self
                        .client
                        .post(&self.base_url)
                        .bearer_auth(&self.api_key)
                        .json(&body)
                        .send()
                        .await
                        .map_err(|e| {
                            AttemptErr::Retryable(VectorError::RerankError(format!(
                                "HTTP transport: {e}"
                            )))
                        })?;

                    let status = resp.status();
                    if !status.is_success() {
                        let body = resp.text().await.unwrap_or_default();
                        let err = VectorError::RerankError(format!(
                            "Cohere rerank HTTP {status}: {body}"
                        ));
                        if RetryPolicy::should_retry_status(status) {
                            return Err(AttemptErr::Retryable(err));
                        }
                        return Err(AttemptErr::Fatal(err));
                    }

                    resp.json::<RerankResponse>().await.map_err(|e| {
                        AttemptErr::Fatal(VectorError::RerankError(format!("invalid JSON: {e}")))
                    })
                },
                |e| matches!(e, AttemptErr::Retryable(_)),
            )
            .await
            .map_err(|e| match e {
                AttemptErr::Retryable(err) | AttemptErr::Fatal(err) => err,
            })?;

        let mut reranked: Vec<HybridSearchResult> = Vec::with_capacity(parsed.results.len());
        for item in parsed.results {
            let Some(mut candidate) = candidates.get(item.index).cloned() else {
                warn!(
                    "Cohere rerank returned out-of-range index {} (candidates={})",
                    item.index,
                    candidates.len()
                );
                continue;
            };
            candidate.rerank_score = Some(item.relevance_score);
            reranked.push(candidate);
        }

        reranked.sort_by(|a, b| {
            b.rerank_score
                .unwrap_or(f32::NEG_INFINITY)
                .partial_cmp(&a.rerank_score.unwrap_or(f32::NEG_INFINITY))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        reranked.truncate(top_k);
        for (i, item) in reranked.iter_mut().enumerate() {
            item.rank = i + 1;
        }
        Ok(reranked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DocumentChunk, HistoricalPeriod};
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn make_chunk(id: &str, text: &str) -> DocumentChunk {
        DocumentChunk {
            chunk_id: id.to_string(),
            doc_id: format!("doc_{id}"),
            doc_title: id.to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::WarOfResistance,
            date: "1938-05".to_string(),
            volume: "第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: text.chars().count(),
            raw_text: text.to_string(),
            contextualized_text: text.to_string(),
            section_path: vec![],
        }
    }

    fn make_result(id: &str, text: &str, rank: usize) -> HybridSearchResult {
        HybridSearchResult {
            chunk_id: id.to_string(),
            rrf_score: 0.1,
            bm25_score: None,
            vector_score: None,
            rerank_score: None,
            rank,
            chunk: make_chunk(id, text),
        }
    }

    #[tokio::test]
    async fn cohere_reranker_reorders_by_mock_scores() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    {"index": 1, "relevance_score": 0.95},
                    {"index": 0, "relevance_score": 0.70},
                    {"index": 2, "relevance_score": 0.40}
                ]
            })))
            .mount(&server)
            .await;

        let reranker = CohereReranker::new(
            "test-key".to_string(),
            Some("rerank-v3.5".to_string()),
            Some(server.uri()),
        );

        let candidates = vec![
            make_result("c0", "first candidate", 1),
            make_result("c1", "second candidate", 2),
            make_result("c2", "third candidate", 3),
        ];

        let out = reranker
            .rerank("persistent war", &candidates, 3)
            .await
            .unwrap();
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].chunk_id, "c1");
        assert_eq!(out[1].chunk_id, "c0");
        assert_eq!(out[2].chunk_id, "c2");
        assert_eq!(out[0].rerank_score, Some(0.95));
        assert_eq!(out[1].rerank_score, Some(0.70));
        assert_eq!(out[2].rerank_score, Some(0.40));
        assert_eq!(out[0].rank, 1);
        assert!(out.iter().all(|r| r.rerank_score.is_some()));
    }

    #[tokio::test]
    async fn cohere_reranker_respects_top_k() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    {"index": 1, "relevance_score": 0.95},
                    {"index": 0, "relevance_score": 0.70}
                ]
            })))
            .mount(&server)
            .await;

        let reranker = CohereReranker::new("k".into(), None, Some(server.uri()));
        let candidates = vec![
            make_result("c0", "a", 1),
            make_result("c1", "b", 2),
            make_result("c2", "c", 3),
        ];
        let out = reranker.rerank("q", &candidates, 2).await.unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].chunk_id, "c1");
        assert_eq!(out[1].chunk_id, "c0");
    }

    #[tokio::test]
    async fn cohere_reranker_http_error_is_rerank_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(401).set_body_string("unauthorized"))
            .mount(&server)
            .await;

        let reranker = CohereReranker::new("bad".into(), None, Some(server.uri()));
        let candidates = vec![make_result("c0", "a", 1)];
        let err = reranker.rerank("q", &candidates, 1).await.unwrap_err();
        match err {
            VectorError::RerankError(msg) => {
                assert!(msg.contains("401"), "msg={msg}");
            }
            other => panic!("expected RerankError, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn cohere_reranker_retries_then_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(503).set_body_string("busy"))
            .up_to_n_times(2)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    {"index": 0, "relevance_score": 0.9}
                ]
            })))
            .mount(&server)
            .await;

        let reranker = CohereReranker::with_retry(
            "k".into(),
            None,
            Some(server.uri()),
            crate::retry::RetryPolicy::fast_test(),
        );
        let candidates = vec![make_result("c0", "a", 1)];
        let out = reranker.rerank("q", &candidates, 1).await.unwrap();
        assert_eq!(out[0].chunk_id, "c0");
        assert_eq!(out[0].rerank_score, Some(0.9));
    }
}
