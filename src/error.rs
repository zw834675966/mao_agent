use thiserror::Error;

/// Core error types for vector database and corpus operations.
#[derive(Error, Debug)]
pub enum VectorError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Frontmatter parse error: {0}")]
    FrontmatterError(String),

    #[error("Embedding error: {0}")]
    EmbeddingError(String),

    #[error("Vector dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("vector snapshot identity mismatch: snapshot model `{snapshot_model}` dim {snapshot_dimension} vs embedder `{source_model}` dim {source_dimension}; re-run ingest to rebuild the index")]
    IdentityMismatch {
        snapshot_model: String,
        snapshot_dimension: usize,
        source_model: String,
        source_dimension: usize,
    },

    #[error("Empty vector provided for operation")]
    EmptyVector,

    #[error("Chunk not found: {0}")]
    ChunkNotFound(String),

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Index corrupted or invalid: {0}")]
    IndexCorrupted(String),

    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("General vector error: {0}")]
    Other(String),
}

impl From<serde_json::Error> for VectorError {
    fn from(err: serde_json::Error) -> Self {
        VectorError::Serialization(err.to_string())
    }
}

impl From<serde_yaml::Error> for VectorError {
    fn from(err: serde_yaml::Error) -> Self {
        VectorError::FrontmatterError(err.to_string())
    }
}

impl From<bincode::Error> for VectorError {
    fn from(err: bincode::Error) -> Self {
        VectorError::Serialization(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, VectorError>;
