pub mod agent;
pub mod cli;
pub mod config;
pub mod corpus;
pub mod error;
pub mod eval;
pub mod graph;
pub mod index;
pub mod mcp;
pub mod model;
pub mod rerank;
pub mod retry;
pub mod server;
pub mod vector;

pub use agent::{
    AgentAnswer, CitationVerifier, DIALECTICAL_SYSTEM_PROMPT, DialecticalAgent, FallbackLlmClient,
    LlmClient, OfflineLlmClient, OnlineLlmClient, VerificationReport,
};
pub use corpus::{
    ChineseSemanticChunker, ChunkerConfig, CorpusScanner, MarkdownParser, clean_cjk_spaces,
};
pub use error::{Result, VectorError};
pub use graph::{
    Entity, GraphDocument, GraphExpandHit, GraphStore, Relationship, ResolvedGraphChunk, SourceRef,
    resolve_graph_chunks, union_graph_bonus,
};
pub use index::{
    FullTextIndex, FullTextSearchResult, HybridSearchCoordinator, HybridSearchResult,
    JiebaTokenizer,
};
pub use model::{
    Document, DocumentChunk, DocumentMetadata, HistoricalPeriod, VectorEntry, VectorFilter,
    VectorSearchResult, VectorStoreStats,
};
pub use rerank::{
    COHERE_RERANK_MODEL, COHERE_RERANK_URL, CohereReranker, Reranker, rerank_or_fallback,
};
#[cfg(feature = "local-embed")]
pub use vector::FastEmbedder;
pub use vector::{
    COHERE_CHAT_MODEL, COHERE_COMPAT_BASE_URL, COHERE_EMBED_MODEL, COHERE_EMBEDDING_DIM,
    DeterministicEmbedder, Embedder, EmbedderSelection, GEMINI_DEFAULT_BASE_URL,
    GEMINI_DEFAULT_DIMENSION, GEMINI_DEFAULT_MODEL, GeminiEmbedder, LOCAL_EMBEDDING_DIM,
    OpenAIEmbedder, VectorIndex, VectorStore, create_embedder_arc, join_openai_path,
    preferred_embed_provider, resolve_embed_dimension, resolve_embed_dimension_with_provider,
    resolve_embedder,
};
