pub mod embedder;
pub mod index;
pub mod math;
pub(crate) mod persist;
pub mod store;

#[cfg(feature = "local-embed")]
pub use embedder::FastEmbedder;
pub use embedder::{
    COHERE_CHAT_MODEL, COHERE_COMPAT_BASE_URL, COHERE_EMBED_MODEL, COHERE_EMBEDDING_DIM,
    DeterministicEmbedder, Embedder, EmbedderSelection, GEMINI_DEFAULT_BASE_URL,
    GEMINI_DEFAULT_DIMENSION, GEMINI_DEFAULT_MODEL, GeminiEmbedder, LOCAL_EMBEDDING_DIM,
    OpenAIEmbedder, SILICONFLOW_DEFAULT_BASE_URL, SILICONFLOW_DEFAULT_DIMENSION,
    SILICONFLOW_DEFAULT_MODEL, create_embedder_arc, join_openai_path, preferred_embed_provider,
    resolve_embed_dimension, resolve_embed_dimension_with_provider, resolve_embedder,
};

pub use index::VectorIndex;
pub use store::VectorStore;
