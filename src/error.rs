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

    #[error(
        "vector snapshot identity mismatch: snapshot model `{snapshot_model}` dim {snapshot_dimension} vs embedder `{source_model}` dim {source_dimension}; re-run ingest to rebuild the index"
    )]
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

    /// Opaque HTTP transport failure (reqwest details are not part of the public API).
    #[error("HTTP request error: {0}")]
    HttpError(String),

    #[error("Rerank error: {0}")]
    RerankError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("General vector error: {0}")]
    Other(String),
}

impl From<serde_json::Error> for VectorError {
    fn from(err: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => VectorError::Io(std::io::Error::other(err.to_string())),
            Category::Syntax | Category::Data | Category::Eof => {
                VectorError::Deserialization(err.to_string())
            }
        }
    }
}

impl From<serde_yaml::Error> for VectorError {
    fn from(err: serde_yaml::Error) -> Self {
        VectorError::FrontmatterError(err.to_string())
    }
}

impl From<bincode::Error> for VectorError {
    fn from(err: bincode::Error) -> Self {
        match &*err {
            bincode::ErrorKind::Io(io) => {
                VectorError::Io(std::io::Error::new(io.kind(), io.to_string()))
            }
            _ => VectorError::Deserialization(err.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, VectorError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json_deserialization_error_mapping() {
        let err: serde_json::Error =
            serde_json::from_str::<serde_json::Value>("{invalid json").unwrap_err();
        let ve: VectorError = err.into();
        assert!(matches!(ve, VectorError::Deserialization(_)));
        let msg = ve.to_string();
        assert!(
            msg.contains("Deserialization error"),
            "unexpected message: {msg}"
        );
    }

    #[test]
    fn test_bincode_deserialization_error_mapping() {
        // Invalid bool encoding (value 2 instead of 0 or 1) is a
        // deserialization error (not an I/O error).
        let bytes: &[u8] = &[2u8];
        let err: bincode::Error = bincode::deserialize::<bool>(bytes).unwrap_err();
        let ve: VectorError = err.into();
        assert!(matches!(ve, VectorError::Deserialization(_)));
        let msg = ve.to_string();
        assert!(
            msg.contains("Deserialization error"),
            "unexpected message: {msg}"
        );
    }
}
