use crate::error::{Result, VectorError};
use crate::retry::RetryPolicy;
use crate::vector::embedder::Embedder;
use crate::vector::math::normalize_in_place;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Embedder connecting to Google Gemini Generative Language API (`gemini-embedding-2` / `gemini-embedding-001`).
pub struct GeminiEmbedder {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
    dimension: usize,
}

#[derive(Serialize)]
struct BatchEmbedRequest<'a> {
    requests: Vec<EmbedContentRequest<'a>>,
}

#[derive(Serialize)]
struct EmbedContentRequest<'a> {
    model: String,
    content: ContentPart<'a>,
    #[serde(
        rename = "outputDimensionality",
        skip_serializing_if = "Option::is_none"
    )]
    output_dimensionality: Option<usize>,
}

#[derive(Serialize)]
struct ContentPart<'a> {
    parts: Vec<TextPart<'a>>,
}

#[derive(Serialize)]
struct TextPart<'a> {
    text: &'a str,
}

#[derive(Deserialize)]
struct BatchEmbedResponse {
    #[serde(default)]
    embeddings: Vec<EmbeddingValues>,
}

#[derive(Deserialize)]
struct EmbeddingValues {
    #[serde(default)]
    values: Vec<f32>,
}

impl GeminiEmbedder {
    pub fn new(base_url: String, api_key: String, model: String, dimension: usize) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            model,
            dimension,
        }
    }

    fn model_path(&self) -> String {
        let clean = self.model.trim_start_matches("models/");
        format!("models/{clean}")
    }

    async fn send_sub_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let clean_model = self.model.trim_start_matches("models/");
        let url = format!(
            "{}/v1beta/models/{}:batchEmbedContents",
            self.base_url, clean_model
        );

        let model_id = self.model_path();
        let requests: Vec<EmbedContentRequest> = texts
            .iter()
            .map(|t| EmbedContentRequest {
                model: model_id.clone(),
                content: ContentPart {
                    parts: vec![TextPart { text: t.as_str() }],
                },
                output_dimensionality: Some(self.dimension),
            })
            .collect();

        let req_body = BatchEmbedRequest { requests };

        // Same canonical policy as the OpenAI-compatible path (5 attempts,
        // 2s→15s, no jitter). Transport errors stay fatal to preserve prior
        // observable behavior.
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

        let policy = RetryPolicy::embeddings_http();
        let this = &self;
        let url_ref = &url;
        let body_ref = &req_body;
        let expected = texts.len();
        policy
            .run(
                |_| async move {
                    let resp = this
                        .client
                        .post(url_ref)
                        .header("Content-Type", "application/json")
                        .header("x-goog-api-key", &this.api_key)
                        .json(body_ref)
                        .send()
                        .await
                        .map_err(|e| AttemptErr::Fatal(VectorError::HttpError(e.to_string())))?;

                    let status = resp.status();
                    if status.is_success() {
                        let parsed: BatchEmbedResponse = resp.json().await.map_err(|e| {
                            AttemptErr::Fatal(VectorError::HttpError(e.to_string()))
                        })?;

                        if parsed.embeddings.len() != expected {
                            return Err(AttemptErr::Fatal(VectorError::EmbeddingError(format!(
                                "Gemini Embedding API returned {} vectors for {} input texts",
                                parsed.embeddings.len(),
                                expected
                            ))));
                        }

                        let mut results = Vec::with_capacity(parsed.embeddings.len());
                        for item in parsed.embeddings {
                            let mut emb = item.values;
                            if emb.len() != this.dimension {
                                return Err(AttemptErr::Fatal(VectorError::DimensionMismatch {
                                    expected: this.dimension,
                                    actual: emb.len(),
                                }));
                            }
                            normalize_in_place(&mut emb);
                            results.push(emb);
                        }

                        return Ok(results);
                    }

                    let body = resp.text().await.unwrap_or_default();
                    let err = VectorError::EmbeddingError(format!(
                        "Gemini Embedding API returned HTTP {status}: {body}"
                    ));
                    if RetryPolicy::should_retry_status(status) {
                        return Err(AttemptErr::Retryable(err));
                    }
                    Err(AttemptErr::Fatal(err))
                },
                |e| matches!(e, AttemptErr::Retryable(_)),
            )
            .await
            .map_err(|e| match e {
                AttemptErr::Retryable(err) | AttemptErr::Fatal(err) => err,
            })
    }
}

#[async_trait]
impl Embedder for GeminiEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let batch = vec![text.to_string()];
        let mut res = self.embed_batch(&batch).await?;
        res.pop().ok_or(VectorError::EmptyVector)
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Sub-batch in chunks of 8 to avoid rate limits and oversized payloads on Gemini API
        const SUB_BATCH_LIMIT: usize = 8;
        let mut all_embeddings = Vec::with_capacity(texts.len());

        for chunk in texts.chunks(SUB_BATCH_LIMIT) {
            let chunk_embeddings = self.send_sub_batch(chunk).await?;
            all_embeddings.extend(chunk_embeddings);
        }

        Ok(all_embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_gemini_embedder_batch_and_normalize() {
        let server = MockServer::start().await;

        let mock_vec = vec![0.5f32; 768];
        let resp_body = serde_json::json!({
            "embeddings": [
                { "values": mock_vec }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/v1beta/models/gemini-embedding-2:batchEmbedContents"))
            .and(header("x-goog-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(resp_body))
            .mount(&server)
            .await;

        let embedder = GeminiEmbedder::new(
            server.uri(),
            "test-key".to_string(),
            "gemini-embedding-2".to_string(),
            768,
        );

        assert_eq!(embedder.dimension(), 768);
        assert_eq!(embedder.model_name(), "gemini-embedding-2");

        let res = embedder.embed("test text").await.unwrap();
        assert_eq!(res.len(), 768);

        // Check L2 unit normalization: sum of squares should be ~1.0
        let norm_sq: f32 = res.iter().map(|x| x * x).sum();
        assert!((norm_sq - 1.0).abs() < 1e-4, "Norm squared: {norm_sq}");
    }
}
