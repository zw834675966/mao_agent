use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::agent::prompt::DIALECTICAL_SYSTEM_PROMPT;
use crate::error::{Result, VectorError};
use crate::model::DocumentChunk;
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

/// Online OpenAI-compatible chat completions client (Cohere Compat API by default).
pub struct OnlineLlmClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model_name: String,
}

impl OnlineLlmClient {
    pub fn new(base_url: String, api_key: String, model_name: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            model_name,
        }
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
            .map_err(VectorError::HttpError)?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(VectorError::Other(format!(
                "LLM API returned HTTP {status}: {body}"
            )));
        }

        let parsed: ChatCompletionResponse = resp.json().await.map_err(VectorError::HttpError)?;
        parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| VectorError::Other("LLM API returned an empty choices list".to_string()))
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
pub struct FallbackLlmClient {
    online: Option<OnlineLlmClient>,
}

impl FallbackLlmClient {
    pub fn from_api_key(base_url: String, api_key: Option<String>, model_name: String) -> Self {
        let online = api_key.map(|key| OnlineLlmClient::new(base_url, key, model_name));
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
                    warn!("LLM API failed, falling back to offline dialectical template: {e}");
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
