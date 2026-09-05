use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::agent::prompt::DIALECTICAL_SYSTEM_PROMPT;
use crate::error::{Result, VectorError};
use crate::model::DocumentChunk;
use crate::retry::RetryPolicy;
use crate::vector::embedder::join_openai_path;

/// LLM backend used by [`crate::agent::DialecticalAgent`].
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(
        &self,
        question: &str,
        user_prompt: &str,
        chunks: &[DocumentChunk],
    ) -> Result<String>;
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: String,
}

enum LlmAttemptError {
    Retryable(VectorError),
    Fatal(VectorError),
}

impl std::fmt::Display for LlmAttemptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Retryable(e) | Self::Fatal(e) => write!(f, "{e}"),
        }
    }
}

impl LlmAttemptError {
    fn into_vector_error(self) -> VectorError {
        match self {
            Self::Retryable(e) | Self::Fatal(e) => e,
        }
    }
}

/// Online OpenAI-compatible chat completions client (Cohere Compat API by default).
pub struct OnlineLlmClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model_name: String,
    retry: RetryPolicy,
}

impl OnlineLlmClient {
    pub fn new(base_url: String, api_key: String, model_name: String) -> Self {
        Self::with_retry(base_url, api_key, model_name, RetryPolicy::cohere_http())
    }

    pub fn with_retry(
        base_url: String,
        api_key: String,
        model_name: String,
        retry: RetryPolicy,
    ) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            model_name,
            retry,
        }
    }

    async fn generate_once(
        &self,
        user_prompt: &str,
    ) -> std::result::Result<String, LlmAttemptError> {
        let url = join_openai_path(&self.base_url, "chat/completions");
        let req_body = ChatCompletionRequest {
            model: &self.model_name,
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: DIALECTICAL_SYSTEM_PROMPT,
                },
                ChatMessage {
                    role: "user",
                    content: user_prompt,
                },
            ],
            temperature: 0.3,
        };

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| LlmAttemptError::Retryable(VectorError::HttpError(e)))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            let err = VectorError::Other(format!("LLM API returned HTTP {status}: {body}"));
            if RetryPolicy::should_retry_status(status) {
                return Err(LlmAttemptError::Retryable(err));
            }
            return Err(LlmAttemptError::Fatal(err));
        }

        let parsed: ChatCompletionResponse = resp
            .json()
            .await
            .map_err(|e| LlmAttemptError::Fatal(VectorError::HttpError(e)))?;
        parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| {
                LlmAttemptError::Fatal(VectorError::Other(
                    "LLM API returned an empty choices list".to_string(),
                ))
            })
    }
}

#[async_trait]
impl LlmClient for OnlineLlmClient {
    async fn generate(
        &self,
        _question: &str,
        user_prompt: &str,
        _chunks: &[DocumentChunk],
    ) -> Result<String> {
        self.retry
            .run(
                |_| self.generate_once(user_prompt),
                |e| matches!(e, LlmAttemptError::Retryable(_)),
            )
            .await
            .map_err(LlmAttemptError::into_vector_error)
    }
}

/// Deterministic offline dialectical template (no network).
pub struct OfflineLlmClient;

impl OfflineLlmClient {
    /// Build the four-stage dialectical answer from retrieved chunks.
    pub fn generate(question: &str, chunks: &[DocumentChunk]) -> String {
        let first_chunk = &chunks[0];
        let title = &first_chunk.doc_title;
        let date = &first_chunk.date;
        let period = first_chunk.period.as_str();
        let quote_excerpt = extract_key_quote(&first_chunk.raw_text);

        format!(
            r#"### 一、 调查研究 (Fact-Finding & Evidence)
依据文献《{}》（{} · {}）的记载与论述：
“{}”

### 二、 主要矛盾分析 (Principal Contradiction)
针对【{}】的问题剖析，其核心主要矛盾表现为：客观环境的规律要求 与 主观认识及执行策略之间的矛盾。
在当前阶段，矛盾的主要方面在于必须坚持实事求是、具体问题具体分析，反对教条主义与本本主义。

### 三、 理论综合 (Dialectical Synthesis)
唯物辩证法指出，事物的内部矛盾是事物发展的根本动力。在《{}》中明确强调了必须从客观实际出发，抓住主要矛盾，一切问题才能迎刃而解。

### 四、 指导实践与方针策略 (Action Policy & Conclusions)
1. 深入实际调查，坚决贯彻群众路线；
2. 集中主要力量解决主要矛盾；
3. 遵循客观规律，根据时空条件的变化灵活制定战略方针。"#,
            title, date, period, quote_excerpt, question, title
        )
    }
}

#[async_trait]
impl LlmClient for OfflineLlmClient {
    async fn generate(
        &self,
        question: &str,
        _user_prompt: &str,
        chunks: &[DocumentChunk],
    ) -> Result<String> {
        Ok(Self::generate(question, chunks))
    }
}

/// Prefer online when a key is configured; on any API failure fall back to offline template.
/// Online path applies [`RetryPolicy`] first; fallback runs only after retries are exhausted.
pub struct FallbackLlmClient {
    online: Option<OnlineLlmClient>,
}

impl FallbackLlmClient {
    pub fn from_api_key(base_url: String, api_key: Option<String>, model_name: String) -> Self {
        let online = api_key.map(|key| OnlineLlmClient::new(base_url, key, model_name));
        Self { online }
    }

    pub fn from_online(online: Option<OnlineLlmClient>) -> Self {
        Self { online }
    }
}

#[async_trait]
impl LlmClient for FallbackLlmClient {
    async fn generate(
        &self,
        question: &str,
        user_prompt: &str,
        chunks: &[DocumentChunk],
    ) -> Result<String> {
        if let Some(ref online) = self.online {
            match online.generate(question, user_prompt, chunks).await {
                Ok(text) => return Ok(text),
                Err(e) => {
                    warn!(
                        "LLM API failed after retries, falling back to offline dialectical template: {e}"
                    );
                }
            }
        }
        Ok(OfflineLlmClient::generate(question, chunks))
    }
}

fn extract_key_quote(raw_text: &str) -> &str {
    let trimmed = raw_text.trim();
    for (idx, ch) in trimmed.char_indices() {
        if ch == '。' || ch == '！' || ch == '？' {
            let sentence = &trimmed[..idx + ch.len_utf8()];
            let char_count = sentence.chars().count();
            if (6..=180).contains(&char_count) {
                return sentence.trim();
            }
        }
    }
    let char_indices: Vec<(usize, char)> = trimmed.char_indices().collect();
    if char_indices.len() <= 120 {
        trimmed
    } else {
        for &(idx, ch) in char_indices[60..120].iter().rev() {
            if ch == '，' || ch == '；' || ch == '、' {
                return trimmed[..idx + ch.len_utf8()].trim();
            }
        }
        let end_byte = char_indices[120].0;
        trimmed[..end_byte].trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn online_retries_then_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
            .up_to_n_times(2)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "choices": [{"message": {"content": "ok-after-retry"}}]
            })))
            .mount(&server)
            .await;

        let client = OnlineLlmClient::with_retry(
            server.uri(),
            "k".into(),
            "m".into(),
            RetryPolicy::fast_test(),
        );
        let text = client.generate("q", "prompt", &[]).await.unwrap();
        assert_eq!(text, "ok-after-retry");
    }

    #[tokio::test]
    async fn fallback_after_retries_exhausted() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(500).set_body_string("still-down"))
            .expect(3)
            .mount(&server)
            .await;

        let online = OnlineLlmClient::with_retry(
            server.uri(),
            "k".into(),
            "m".into(),
            RetryPolicy::fast_test(),
        );
        let client = FallbackLlmClient::from_online(Some(online));
        let chunk = DocumentChunk {
            chunk_id: "c1".into(),
            doc_id: "d1".into(),
            doc_title: "论持久战".into(),
            author: "毛泽东".into(),
            period: crate::model::HistoricalPeriod::WarOfResistance,
            date: "1938-05".into(),
            volume: "第二卷".into(),
            category: "军事".into(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 20,
            raw_text: "中日战争是持久战，最后的胜利是中国的。".into(),
            contextualized_text: "中日战争是持久战，最后的胜利是中国的。".into(),
            section_path: vec![],
        };
        let text = client
            .generate("q", "prompt", std::slice::from_ref(&chunk))
            .await
            .unwrap();
        assert!(text.contains("调查研究"), "got: {text}");
    }
}
