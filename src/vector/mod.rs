pub mod embedder;
pub mod index;
pub mod math;
pub mod store;

#[cfg(feature = "local-embed")]
pub use embedder::FastEmbedder;
pub use embedder::{
    COHERE_CHAT_MODEL, COHERE_COMPAT_BASE_URL, COHERE_EMBED_MODEL, COHERE_EMBEDDING_DIM,
    DeterministicEmbedder, Embedder, LOCAL_EMBEDDING_DIM, OpenAIEmbedder, create_embedder_arc,
    join_openai_path,
};

pub use index::VectorIndex;
pub use store::VectorStore;
