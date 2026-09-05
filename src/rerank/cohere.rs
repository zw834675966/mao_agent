//! Cohere Rerank API client (`POST /v2/rerank`).

/// Official Cohere v2 rerank endpoint (not the OpenAI-compat base).
pub const COHERE_RERANK_URL: &str = "https://api.cohere.com/v2/rerank";

/// Default Cohere rerank model.
pub const COHERE_RERANK_MODEL: &str = "rerank-v3.5";

/// Cohere HTTP reranker. Implemented in a follow-up commit.
#[derive(Debug, Clone)]
pub struct CohereReranker {
    pub(crate) _placeholder: (),
}
