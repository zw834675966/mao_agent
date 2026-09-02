pub mod agent;
pub mod cli;
pub mod config;
pub mod corpus;
pub mod error;
pub mod index;
pub mod model;
pub mod vector;

pub use agent::{
    AgentAnswer, CitationVerifier, DIALECTICAL_SYSTEM_PROMPT, DialecticalAgent, VerificationReport,
};
pub use corpus::{
    ChineseSemanticChunker, ChunkerConfig, CorpusScanner, MarkdownParser, clean_cjk_spaces,
};
pub use error::{Result, VectorError};
pub use index::{
    FullTextIndex, FullTextSearchResult, HybridSearchCoordinator, HybridSearchResult,
    JiebaTokenizer,
};
pub use model::{
    Document, DocumentChunk, DocumentMetadata, HistoricalPeriod, VectorEntry, VectorFilter,
    VectorSearchResult, VectorStoreStats,
};
#[cfg(feature = "local-embed")]
pub use vector::FastEmbedder;
pub use vector::{
    COHERE_CHAT_MODEL, COHERE_COMPAT_BASE_URL, COHERE_EMBED_MODEL, COHERE_EMBEDDING_DIM,
    DeterministicEmbedder, Embedder, LOCAL_EMBEDDING_DIM, OpenAIEmbedder, VectorIndex, VectorStore,
    create_embedder_arc, join_openai_path,
};
