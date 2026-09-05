use crate::agent::prompt::{DIALECTICAL_SYSTEM_PROMPT, build_rag_user_prompt};
use crate::agent::verifier::{CitationVerifier, VerificationReport};
use crate::error::{Result, VectorError};
use crate::index::fulltext::FullTextIndex;
use crate::index::hybrid::HybridSearchCoordinator;
use crate::model::{DocumentChunk, VectorFilter};
use crate::rerank::{Reranker, rerank_or_fallback};
use crate::vector::embedder::join_openai_path;
use crate::vector::store::VectorStore;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock};
use tracing::{info, warn};

static QUOTE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"["“]([^"”]{6,200})["”]"#).unwrap());

static TITLE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"《([^》]+)》"#).unwrap());

/// Complete dialectical reasoning answer with grounding verification metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAnswer {
    pub question: String,
    pub content: String,
    pub retrieved_chunks: Vec<DocumentChunk>,
    pub citation_reports: Vec<VerificationReport>,
    pub is_fully_grounded: bool,
}

/// Dialectical Reasoning Agent orchestrating hybrid retrieval, LLM synthesis, and citation verification.
pub struct DialecticalAgent {
    store: Arc<VectorStore>,
    fulltext_index: Option<Arc<FullTextIndex>>,
    hybrid_coordinator: HybridSearchCoordinator,
    verifier: CitationVerifier,
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
    model_name: String,
    reranker: Option<Arc<dyn Reranker>>,
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

impl DialecticalAgent {
    pub fn new(
        store: Arc<VectorStore>,
        fulltext_index: Option<Arc<FullTextIndex>>,
        base_url: Option<String>,
        api_key: Option<String>,
        model_name: Option<String>,
        reranker: Option<Arc<dyn Reranker>>,
    ) -> Self {
        Self {
            store,
            fulltext_index,
            hybrid_coordinator: HybridSearchCoordinator::default(),
            verifier: CitationVerifier::default(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            base_url: base_url
                .unwrap_or_else(|| crate::vector::embedder::COHERE_COMPAT_BASE_URL.to_string())
                .trim_end_matches('/')
                .to_string(),
            api_key,
            model_name: model_name
                .unwrap_or_else(|| crate::vector::embedder::COHERE_CHAT_MODEL.to_string()),
            reranker,
        }
    }

    /// Ask the agent a question using historical corpus grounding and dialectical reasoning.
    pub async fn ask(
        &self,
        question: &str,
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<AgentAnswer> {
        info!("DialecticalAgent processing query: {}", question);

        // 1. Retrieve evidence chunks (Hybrid search if FullTextIndex is configured, otherwise Vector search)
        let retrieved_chunks: Vec<DocumentChunk> = if let Some(ref ft) = self.fulltext_index {
            let vec_results = self.store.search(question, top_k * 2, filter).await?;
            let bm25_results = match ft.search(question, top_k * 2, filter) {
                Ok(results) => results,
                Err(e) => {
                    warn!("BM25 search failed: {e}, falling back to vector-only retrieval.");
                    Vec::new()
                }
            };
            let fused = self
                .hybrid_coordinator
                .fuse(vec_results, bm25_results, top_k * 2);
            let reranked =
                rerank_or_fallback(fused, self.reranker.as_deref(), question, top_k).await;
            reranked.into_iter().map(|r| r.chunk).collect()
        } else {
            let search_results = self.store.search(question, top_k, filter).await?;
            search_results.into_iter().map(|r| r.chunk).collect()
        };

        if retrieved_chunks.is_empty() {
            return Ok(AgentAnswer {
                question: question.to_string(),
                content: "【未检索到相关历史文献】未能从语料库中检索到能够支撑该问题的文献段落。按照“没有调查就没有发言权”原则，不作主观推测。".to_string(),
                retrieved_chunks: Vec::new(),
                citation_reports: Vec::new(),
                is_fully_grounded: false,
            });
        }

        // 2. Build prompt with evidence chunks
        let context_texts: Vec<String> = retrieved_chunks
            .iter()
            .map(|c| c.contextualized_text.clone())
            .collect();
        let user_prompt = build_rag_user_prompt(question, &context_texts);

        // 3. Generate answer via LLM (or deterministic dialectical template if no API key)
        let raw_answer = if self.api_key.is_some() {
            self.call_llm_api(&user_prompt).await?
        } else {
            self.generate_offline_dialectical_answer(question, &retrieved_chunks)
        };

        // 4. Extract quotes and verify them against retrieved chunks
        let citation_reports = self.verify_extracted_citations(&raw_answer, &retrieved_chunks);
        let is_fully_grounded =
            answer_is_fully_grounded(!retrieved_chunks.is_empty(), &citation_reports);

        Ok(AgentAnswer {
            question: question.to_string(),
            content: raw_answer,
            retrieved_chunks,
            citation_reports,
            is_fully_grounded,
        })
    }

    async fn call_llm_api(&self, user_prompt: &str) -> Result<String> {
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

        let mut req = self.client.post(&url).json(&req_body);
        if let Some(ref key) = self.api_key {
            req = req.bearer_auth(key);
        }

        let resp = req.send().await.map_err(VectorError::HttpError)?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(VectorError::Other(format!(
                "LLM API returned HTTP {status}: {body}"
            )));
        }

        let parsed: ChatCompletionResponse = resp.json().await.map_err(VectorError::HttpError)?;
        let answer = parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| {
                VectorError::Other("LLM API returned an empty choices list".to_string())
            })?;

        Ok(answer)
    }

    fn generate_offline_dialectical_answer(
        &self,
        question: &str,
        chunks: &[DocumentChunk],
    ) -> String {
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

    fn verify_extracted_citations(
        &self,
        text: &str,
        chunks: &[DocumentChunk],
    ) -> Vec<VerificationReport> {
        let mut reports = Vec::new();

        for caps in QUOTE_REGEX.captures_iter(text) {
            if let Some(quote_match) = caps.get(1) {
                let quote_str = quote_match.as_str();
                let quote_pos = quote_match.start();

                // Look for closest preceding 《...》 title before this quote
                let prefix_text = &text[..quote_pos];
                let claimed_title = TITLE_REGEX
                    .captures_iter(prefix_text)
                    .last()
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str())
                    .unwrap_or_else(|| {
                        chunks
                            .first()
                            .map(|c| c.doc_title.as_str())
                            .unwrap_or("未知文献")
                    });

                let report = self.verifier.verify_quote(quote_str, claimed_title, chunks);
                reports.push(report);
            }
        }

        reports
    }
}

pub(crate) fn answer_is_fully_grounded(
    has_evidence: bool,
    citation_reports: &[VerificationReport],
) -> bool {
    has_evidence && !citation_reports.is_empty() && citation_reports.iter().all(|r| r.is_verified)
}

fn extract_key_quote(raw_text: &str) -> &str {
    let trimmed = raw_text.trim();
    // Prefer the first complete sentence ending in '。', '！', '？'
    for (idx, ch) in trimmed.char_indices() {
        if ch == '。' || ch == '！' || ch == '？' {
            let sentence = &trimmed[..idx + ch.len_utf8()];
            let char_count = sentence.chars().count();
            if (6..=180).contains(&char_count) {
                return sentence.trim();
            }
        }
    }
    // Fallback: take up to first 120 chars bounded by a clause punctuation
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
    use crate::model::{DocumentMetadata, HistoricalPeriod};

    #[tokio::test]
    async fn test_dialectical_agent_offline_reasoning() {
        let store = Arc::new(VectorStore::new_deterministic(128));

        let doc = crate::model::Document {
            id: "doc_1".to_string(),
            metadata: DocumentMetadata {
                title: "论持久战".to_string(),
                author: "毛泽东".to_string(),
                date: "1938-05-26".to_string(),
                period: "抗日战争时期".to_string(),
                volume: "毛泽东选集第二卷".to_string(),
                category: "军事".to_string(),
                tags: vec!["持久战".to_string()],
                ..Default::default()
            },
            period_enum: HistoricalPeriod::WarOfResistance,
            headnote: None,
            content: "中日战争是持久战，最后的胜利是中国的。战争将经历战略防御、战略相持、战略反攻三个阶段。".to_string(),
            footnotes: vec![],
            file_path: None,
        };

        store.index_document(&doc).await.unwrap();

        let agent = DialecticalAgent::new(store, None, None, None, None, None);
        let answer = agent
            .ask("抗日战争为什么是持久战？", 3, None)
            .await
            .unwrap();

        assert!(!answer.content.is_empty());
        assert!(answer.content.contains("调查研究"));
        assert!(answer.content.contains("主要矛盾分析"));
        assert!(answer.content.contains("论持久战"));
        assert_eq!(answer.retrieved_chunks.len(), 1);
        assert!(!answer.citation_reports.is_empty());
        assert!(answer.citation_reports[0].is_verified);
        assert!(answer.is_fully_grounded);
    }

    #[tokio::test]
    async fn test_empty_retrieval_is_not_fully_grounded() {
        let store = Arc::new(VectorStore::new_deterministic(64));
        let agent = DialecticalAgent::new(store, None, None, None, None, None);
        let answer = agent.ask("语料库里不存在的问题", 3, None).await.unwrap();

        assert!(answer.retrieved_chunks.is_empty());
        assert!(
            !answer.is_fully_grounded,
            "no retrieved evidence must not count as fully grounded"
        );
    }

    #[test]
    fn test_no_extracted_quotes_is_not_fully_grounded() {
        assert!(!answer_is_fully_grounded(false, &[]));
        assert!(!answer_is_fully_grounded(true, &[]));
    }

    #[tokio::test]
    async fn test_offline_dialectical_four_stage_structure() {
        let store = Arc::new(VectorStore::new_deterministic(128));

        let doc = crate::model::Document {
            id: "doc_structure".to_string(),
            metadata: DocumentMetadata {
                title: "论持久战".to_string(),
                author: "毛泽东".to_string(),
                date: "1938-05-26".to_string(),
                period: "抗日战争时期".to_string(),
                volume: "毛泽东选集第二卷".to_string(),
                category: "军事".to_string(),
                tags: vec!["持久战".to_string()],
                ..Default::default()
            },
            period_enum: HistoricalPeriod::WarOfResistance,
            headnote: None,
            content: "中日战争是持久战，最后的胜利是中国的。战争将经历战略防御、战略相持、战略反攻三个阶段。".to_string(),
            footnotes: vec![],
            file_path: None,
        };

        store.index_document(&doc).await.unwrap();

        let agent = DialecticalAgent::new(store, None, None, None, None, None);
        let answer = agent
            .ask("抗日战争为什么是持久战？", 3, None)
            .await
            .unwrap();

        let stages = ["调查研究", "主要矛盾", "理论综合", "指导实践"];
        let mut cursor = 0usize;
        for stage in stages {
            let found = answer.content[cursor..]
                .find(stage)
                .unwrap_or_else(|| panic!("missing dialectical stage `{stage}` in order"));
            cursor += found + stage.len();
        }
        assert!(
            !answer.citation_reports.is_empty(),
            "offline template should emit a verifiable quote"
        );
        assert!(answer.citation_reports.iter().all(|r| r.is_verified));
        assert!(answer.is_fully_grounded);
    }
}
