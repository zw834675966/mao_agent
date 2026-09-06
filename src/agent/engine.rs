use crate::agent::llm::{FallbackLlmClient, LlmClient};
use crate::agent::prompt::build_rag_user_prompt_with_triples;
use crate::agent::verifier::{CitationVerifier, VerificationReport};
use crate::error::Result;
use crate::graph::{GraphStore, ResolvedGraphChunk, union_graph_bonus};
use crate::index::fulltext::FullTextIndex;
use crate::index::hybrid::{HybridSearchCoordinator, HybridSearchResult};
use crate::model::{DocumentChunk, VectorFilter};
use crate::rerank::{Reranker, rerank_or_fallback};
use crate::vector::store::VectorStore;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU64;
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
    /// True iff at least one hybrid result after rerank_or_fallback has a stamped rerank_score.
    pub rerank_applied: bool,
    /// Per-chunk rerank scores in the same order as retrieved_chunks; None when not applied.
    pub rerank_scores: Option<Vec<f32>>,
}

/// Dialectical Reasoning Agent orchestrating hybrid retrieval, LLM synthesis, and citation verification.
pub struct DialecticalAgent {
    store: Arc<VectorStore>,
    fulltext_index: Option<Arc<FullTextIndex>>,
    hybrid_coordinator: HybridSearchCoordinator,
    verifier: CitationVerifier,
    llm: Arc<dyn LlmClient>,
    reranker: Option<Arc<dyn Reranker>>,
    graph: Option<Arc<GraphStore>>,
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
        Self::new_with_fallback_counter(
            store,
            fulltext_index,
            base_url,
            api_key,
            model_name,
            reranker,
            None,
        )
    }

    pub fn new_with_fallback_counter(
        store: Arc<VectorStore>,
        fulltext_index: Option<Arc<FullTextIndex>>,
        base_url: Option<String>,
        api_key: Option<String>,
        model_name: Option<String>,
        reranker: Option<Arc<dyn Reranker>>,
        fallback_counter: Option<Arc<AtomicU64>>,
    ) -> Self {
        let base_url = base_url
            .unwrap_or_else(|| crate::vector::embedder::COHERE_COMPAT_BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string();
        let model_name =
            model_name.unwrap_or_else(|| crate::vector::embedder::COHERE_CHAT_MODEL.to_string());
        let mut fallback = FallbackLlmClient::from_api_key(base_url, api_key, model_name);
        if let Some(counter) = fallback_counter {
            fallback = fallback.with_fallback_counter(counter);
        }
        let llm: Arc<dyn LlmClient> = Arc::new(fallback);
        Self {
            store,
            fulltext_index,
            hybrid_coordinator: HybridSearchCoordinator::default(),
            verifier: CitationVerifier::default(),
            llm,
            reranker,
            graph: None,
        }
    }

    pub fn with_graph(mut self, graph: Arc<GraphStore>) -> Self {
        self.graph = Some(graph);
        self
    }

    async fn expand_fused(
        &self,
        fused: Vec<HybridSearchResult>,
        question: &str,
        top_k: usize,
    ) -> Vec<HybridSearchResult> {
        let Some(graph) = self.graph.as_ref() else {
            return fused;
        };
        let hits = graph.expand(question, 2);
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
        let final_k = if self.reranker.is_some() {
            None
        } else {
            Some(top_k)
        };
        union_graph_bonus(fused, &resolved, final_k)
    }

    fn graph_triples(&self, question: &str) -> Vec<String> {
        let Some(graph) = self.graph.as_ref() else {
            return Vec::new();
        };
        graph
            .expand(question, 2)
            .into_iter()
            .flat_map(|h| h.paths)
            .take(16)
            .collect()
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
        let (retrieved_chunks, rerank_applied, rerank_scores): (
            Vec<DocumentChunk>,
            bool,
            Option<Vec<f32>>,
        ) = if let Some(ref ft) = self.fulltext_index {
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
            let fused = self.expand_fused(fused, question, top_k).await;
            let reranked =
                rerank_or_fallback(fused, self.reranker.as_deref(), question, top_k).await;
            let applied = reranked.iter().any(|r| r.rerank_score.is_some());
            let scores = if applied {
                Some(
                    reranked
                        .iter()
                        .map(|r| r.rerank_score.unwrap_or(0.0))
                        .collect(),
                )
            } else {
                None
            };
            let chunks = reranked.into_iter().map(|r| r.chunk).collect();
            (chunks, applied, scores)
        } else {
            let search_results = self.store.search(question, top_k, filter).await?;
            let chunks = search_results.into_iter().map(|r| r.chunk).collect();
            (chunks, false, None)
        };

        if retrieved_chunks.is_empty() {
            return Ok(AgentAnswer {
                question: question.to_string(),
                content: "【未检索到相关历史文献】未能从语料库中检索到能够支撑该问题的文献段落。按照“没有调查就没有发言权”原则，不作主观推测。".to_string(),
                retrieved_chunks: Vec::new(),
                citation_reports: Vec::new(),
                is_fully_grounded: false,
                rerank_applied: false,
                rerank_scores: None,
            });
        }

        // 2. Build prompt with evidence chunks
        let context_texts: Vec<String> = retrieved_chunks
            .iter()
            .map(|c| c.contextualized_text.clone())
            .collect();
        let triples = self.graph_triples(question);
        let user_prompt = build_rag_user_prompt_with_triples(question, &context_texts, &triples);

        // 3. Generate answer via LLM (online with offline fallback on API error / missing key)
        let raw_answer = self
            .llm
            .generate(question, &user_prompt, &retrieved_chunks)
            .await?;

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
            rerank_applied,
            rerank_scores,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DocumentMetadata, HistoricalPeriod};

    fn sample_doc() -> crate::model::Document {
        crate::model::Document {
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
        }
    }

    #[tokio::test]
    async fn test_dialectical_agent_offline_reasoning() {
        let store = Arc::new(VectorStore::new_deterministic(128));
        store.index_document(&sample_doc()).await.unwrap();

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
        store.index_document(&sample_doc()).await.unwrap();

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

    #[tokio::test]
    async fn test_llm_api_error_falls_back_offline() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(500).set_body_string("upstream boom"))
            .mount(&server)
            .await;

        let store = Arc::new(VectorStore::new_deterministic(128));
        store.index_document(&sample_doc()).await.unwrap();

        let agent = DialecticalAgent::new(
            store,
            None,
            Some(server.uri()),
            Some("test-key-present".to_string()),
            Some("test-model".to_string()),
            None,
        );
        let answer = agent
            .ask("抗日战争为什么是持久战？", 3, None)
            .await
            .unwrap();

        assert!(
            answer.content.contains("调查研究"),
            "API error with key set must fall back to offline dialectical template, got:\n{}",
            answer.content
        );
        assert!(answer.content.contains("主要矛盾"));
        assert!(!answer.retrieved_chunks.is_empty());
    }
}
