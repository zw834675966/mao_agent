use crate::error::{Result, VectorError};
use crate::vector::math::normalize_in_place;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tracing::info;

mod cache;
pub(crate) use cache::{CachedEmbedder, embed_cache_path};

/// Default dimension for the local Chinese BGE-small-zh-v1.5 embedder (CLI path).
pub const LOCAL_EMBEDDING_DIM: usize = 512;

/// Cohere OpenAI-compatible Compatibility API (embeddings + chat).
pub const COHERE_COMPAT_BASE_URL: &str = "https://api.cohere.ai/compatibility/v1";
/// `embed-v4.0` default width. Compatibility API does not accept `dimensions`.
pub const COHERE_EMBEDDING_DIM: usize = 1536;
pub const COHERE_EMBED_MODEL: &str = "embed-v4.0";
pub const COHERE_CHAT_MODEL: &str = "command-r7b-12-2024";

/// Join an OpenAI-compatible base URL with an endpoint like `embeddings`.
///
/// Accepts both `https://api.openai.com` and bases that already end in `/v1`
/// (e.g. Cohere `https://api.cohere.ai/compatibility/v1`).
pub fn join_openai_path(base_url: &str, endpoint: &str) -> String {
    let base = base_url.trim().trim_end_matches('/');
    let endpoint = endpoint.trim_start_matches('/');
    if base.ends_with("/v1") {
        format!("{base}/{endpoint}")
    } else {
        format!("{base}/v1/{endpoint}")
    }
}

/// Trait defining the interface for vector embedding models.
#[async_trait]
pub trait Embedder: Send + Sync {
    /// Embed a single text string into a float vector.
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;

    /// Embed a batch of text strings into vectors.
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;

    /// The dimension of the generated embedding vectors.
    fn dimension(&self) -> usize;

    /// The model name or identifier.
    fn model_name(&self) -> &str;
}

// ============================================================================
// 1. Deterministic Hash / N-gram Embedder (For instant tests and offline CI)
// ============================================================================

/// Fast, deterministic embedder that generates unit-normalized semantic-like vectors
/// using character n-grams and hashing. Requires zero model downloads or GPU.
#[derive(Debug, Clone)]
pub struct DeterministicEmbedder {
    dimension: usize,
    model_name: String,
}

impl DeterministicEmbedder {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            model_name: format!("deterministic-hash-{}", dimension),
        }
    }
}

impl Default for DeterministicEmbedder {
    fn default() -> Self {
        Self::new(384)
    }
}

#[async_trait]
impl Embedder for DeterministicEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let mut vec = vec![0.0f32; self.dimension];
        if text.is_empty() {
            return Ok(vec);
        }

        // Generate features from character n-grams (1-gram, 2-gram, 3-gram)
        let chars: Vec<char> = text.chars().collect();
        for n in 1..=3 {
            for window in chars.windows(n) {
                let gram: String = window.iter().collect();
                let hash = hash_str(&gram);
                let idx = (hash as usize) % self.dimension;
                let sign = if (hash >> 32) & 1 == 0 { 1.0 } else { -1.0 };
                vec[idx] += sign;
            }
        }

        normalize_in_place(&mut vec);
        Ok(vec)
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut results = Vec::with_capacity(texts.len());
        for t in texts {
            results.push(self.embed(t).await?);
        }
        Ok(results)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }
}

fn hash_str(s: &str) -> u64 {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    u64::from_le_bytes(result[..8].try_into().expect("SHA-256 digest is 32 bytes"))
}

// ============================================================================
// 2. OpenAI / DeepSeek / Ollama Compatible API Embedder
// ============================================================================

/// Embedder connecting to any OpenAI-compatible `/v1/embeddings` endpoint.
pub struct OpenAIEmbedder {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
    model: String,
    dimension: usize,
}

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    input: &'a [String],
    model: &'a str,
    /// Cohere Compatibility API requires this; OpenAI also accepts it.
    encoding_format: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

impl OpenAIEmbedder {
    pub fn new(base_url: String, api_key: Option<String>, model: String, dimension: usize) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            model,
            dimension,
        }
    }
}

#[async_trait]
impl Embedder for OpenAIEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let batch = vec![text.to_string()];
        let mut res = self.embed_batch(&batch).await?;
        res.pop().ok_or(VectorError::EmptyVector)
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let url = join_openai_path(&self.base_url, "embeddings");
        let req_body = EmbeddingRequest {
            input: texts,
            model: &self.model,
            encoding_format: "float",
        };

        let mut req = self.client.post(&url).json(&req_body);
        if let Some(ref key) = self.api_key {
            req = req.bearer_auth(key);
        }

        let resp = req.send().await.map_err(VectorError::HttpError)?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(VectorError::EmbeddingError(format!(
                "Embedding API returned HTTP {status}: {body}"
            )));
        }

        let mut parsed: EmbeddingResponse = resp.json().await.map_err(VectorError::HttpError)?;
        parsed.data.sort_by_key(|d| d.index);

        if parsed.data.len() != texts.len() {
            return Err(VectorError::EmbeddingError(format!(
                "Embedding API returned {} vectors for {} input texts",
                parsed.data.len(),
                texts.len()
            )));
        }

        let mut embeddings: Vec<Vec<f32>> = parsed.data.into_iter().map(|d| d.embedding).collect();
        for emb in &mut embeddings {
            if emb.len() != self.dimension {
                return Err(VectorError::DimensionMismatch {
                    expected: self.dimension,
                    actual: emb.len(),
                });
            }
            normalize_in_place(emb);
        }

        Ok(embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

// ============================================================================
// 3. FastEmbed (Local ONNX BGE Embedding)
// ============================================================================

#[cfg(feature = "local-embed")]
pub struct FastEmbedder {
    model: std::sync::Mutex<fastembed::TextEmbedding>,
    dimension: usize,
    model_name: String,
}

#[cfg(feature = "local-embed")]
impl FastEmbedder {
    /// Initialize with BGE-small-zh-v1.5 (512-dim) for Chinese corpus retrieval.
    pub fn try_new() -> Result<Self> {
        Self::try_new_with_model(fastembed::EmbeddingModel::BGESmallZHV15)
    }

    pub fn try_new_with_model(model_type: fastembed::EmbeddingModel) -> Result<Self> {
        info!(
            "Initializing local FastEmbed ONNX embedding model: {:?}",
            model_type
        );
        let options = fastembed::InitOptions::new(model_type.clone());
        let model = fastembed::TextEmbedding::try_new(options).map_err(|e| {
            VectorError::EmbeddingError(format!("Failed to load FastEmbed model: {e}"))
        })?;

        let dimension = match model_type {
            fastembed::EmbeddingModel::BGESmallZHV15 => LOCAL_EMBEDDING_DIM,
            fastembed::EmbeddingModel::BGESmallENV15 => 384,
            _ => LOCAL_EMBEDDING_DIM,
        };

        Ok(Self {
            model: std::sync::Mutex::new(model),
            dimension,
            model_name: format!("{model_type:?}"),
        })
    }
}

#[cfg(feature = "local-embed")]
#[async_trait]
impl Embedder for FastEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let texts = vec![text.to_string()];
        let mut res = self.embed_batch(&texts).await?;
        res.pop().ok_or(VectorError::EmptyVector)
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let model_guard = self.model.lock().map_err(|_| {
            VectorError::EmbeddingError("Failed to acquire lock on FastEmbed model".to_string())
        })?;

        let raw_embeddings = model_guard
            .embed(texts.to_vec(), None)
            .map_err(|e| VectorError::EmbeddingError(format!("FastEmbed embedding failed: {e}")))?;

        let mut results = Vec::with_capacity(raw_embeddings.len());
        for mut emb in raw_embeddings {
            normalize_in_place(&mut emb);
            results.push(emb);
        }

        Ok(results)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }
}

/// Helper function to create an Arc-wrapped Embedder.
pub fn create_embedder_arc(embedder: impl Embedder + 'static) -> Arc<dyn Embedder> {
    Arc::new(embedder)
}

/// CLI/lib selection for [`resolve_embedder`]. Empty `api_key` is treated as unset.
#[derive(Clone)]
pub struct EmbedderSelection {
    pub offline: bool,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub dimension: usize,
}

impl std::fmt::Debug for EmbedderSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbedderSelection")
            .field("offline", &self.offline)
            .field("api_key", &self.api_key.as_ref().map(|_| "[redacted]"))
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("dimension", &self.dimension)
            .finish()
    }
}

/// Default embedding width when the CLI omits `--embed-dim`.
/// `--offline` matches local BGE (512); otherwise Cohere embed-v4.0 (1536).
pub fn resolve_embed_dimension(offline: bool, explicit: Option<usize>) -> usize {
    explicit.unwrap_or(if offline {
        LOCAL_EMBEDDING_DIM
    } else {
        COHERE_EMBEDDING_DIM
    })
}

/// Fail-closed embedder construction. Only the remote OpenAI path is cache-wrapped.
///
/// `cache_path` is the vector index file; the cache is `{index_file}.embedcache`.
pub fn resolve_embedder(
    selection: &EmbedderSelection,
    cache_path: Option<&Path>,
) -> Result<Arc<dyn Embedder>> {
    if selection.offline {
        info!(
            "Using offline Deterministic Embedder ({}-dim)",
            selection.dimension
        );
        return Ok(create_embedder_arc(DeterministicEmbedder::new(
            selection.dimension,
        )));
    }

    let api_key = selection
        .api_key
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let base_url = selection
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| api_key.as_ref().map(|_| COHERE_COMPAT_BASE_URL.to_string()));

    if let Some(base_url) = base_url {
        info!(
            "Using remote OpenAI-compatible embedder {} at {} ({}-dim)",
            selection.model, base_url, selection.dimension
        );
        let inner: Arc<dyn Embedder> = create_embedder_arc(OpenAIEmbedder::new(
            base_url,
            api_key,
            selection.model.clone(),
            selection.dimension,
        ));
        if let Some(index_file) = cache_path {
            let cache_file = embed_cache_path(index_file);
            return Ok(create_embedder_arc(CachedEmbedder::new(inner, cache_file)?));
        }
        return Ok(inner);
    }

    #[cfg(feature = "local-embed")]
    {
        let fe = FastEmbedder::try_new().map_err(|e| {
            VectorError::EmbeddingError(format!(
                "FastEmbed init failed: {e}. Pass --offline or provide a Cohere API key (no silent hash fallback)."
            ))
        })?;
        info!(
            "Using local FastEmbed ONNX BGE-small-zh-v1.5 ({}-dim)",
            fe.dimension()
        );
        Ok(create_embedder_arc(fe))
    }

    #[cfg(not(feature = "local-embed"))]
    {
        Err(VectorError::EmbeddingError(
            "No embedding backend: pass --offline for the deterministic hash embedder, or provide a Cohere API key (--embed-api-key / COHERE_API_KEY / config.toml)".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_deterministic_embedder() {
        let embedder = DeterministicEmbedder::new(384);
        let v1 = embedder.embed("中国革命战争的战略问题").await.unwrap();
        let v2 = embedder.embed("中国革命战争的战略问题").await.unwrap();
        let v3 = embedder.embed("抗日游击战争的战略问题").await.unwrap();

        assert_eq!(v1.len(), 384);
        assert_eq!(v1, v2);

        // Compute similarity
        let sim = crate::vector::math::cosine_similarity(&v1, &v3);
        assert!(
            sim > 0.0,
            "Similar texts should have positive cosine similarity"
        );
    }

    #[test]
    fn test_resolve_embed_dimension_defaults_and_explicit() {
        assert_eq!(resolve_embed_dimension(true, None), LOCAL_EMBEDDING_DIM);
        assert_eq!(resolve_embed_dimension(false, None), COHERE_EMBEDDING_DIM);
        assert_eq!(resolve_embed_dimension(true, Some(1536)), 1536);
        assert_eq!(resolve_embed_dimension(false, Some(512)), 512);
    }

    #[test]
    fn test_join_openai_path_accepts_cohere_groq_and_openai_bases() {
        assert_eq!(
            join_openai_path("https://api.cohere.ai/compatibility/v1", "embeddings"),
            "https://api.cohere.ai/compatibility/v1/embeddings"
        );
        assert_eq!(
            join_openai_path(
                "https://api.cohere.ai/compatibility/v1/",
                "chat/completions"
            ),
            "https://api.cohere.ai/compatibility/v1/chat/completions"
        );
        assert_eq!(
            join_openai_path("https://api.groq.com/openai/v1", "embeddings"),
            "https://api.groq.com/openai/v1/embeddings"
        );
        assert_eq!(
            join_openai_path("https://api.openai.com", "embeddings"),
            "https://api.openai.com/v1/embeddings"
        );
        assert_eq!(
            join_openai_path("https://api.openai.com/v1", "embeddings"),
            "https://api.openai.com/v1/embeddings"
        );
    }

    #[test]
    fn test_embedding_request_sends_encoding_format_float() {
        let input = ["捉住主要矛盾".to_string()];
        let req = EmbeddingRequest {
            input: &input,
            model: "embed-v4.0",
            encoding_format: "float",
        };
        let json = serde_json::to_value(&req).expect("serialize embedding request");
        assert_eq!(json["encoding_format"], "float");
        assert_eq!(json["model"], "embed-v4.0");
        assert_eq!(json["input"][0], "捉住主要矛盾");
        assert!(json.get("dimensions").is_none());
    }

    #[tokio::test]
    async fn test_resolve_offline_uses_deterministic_and_skips_cache_file() {
        let tmp = tempfile::tempdir().unwrap();
        let index_file = tmp.path().join("vector_store.bin");
        let selection = EmbedderSelection {
            offline: true,
            api_key: None,
            base_url: None,
            model: "embed-v4.0".into(),
            dimension: 64,
        };
        let embedder = resolve_embedder(&selection, Some(&index_file)).unwrap();
        assert_eq!(embedder.model_name(), "deterministic-hash-64");
        assert_eq!(embedder.dimension(), 64);
        let _ = embedder.embed("捉住主要矛盾").await.unwrap();
        assert!(
            !embed_cache_path(&index_file).exists(),
            "offline deterministic path must not create .embedcache"
        );
    }

    #[cfg(not(feature = "local-embed"))]
    #[test]
    fn test_resolve_no_key_no_offline_is_error_without_local_embed() {
        let selection = EmbedderSelection {
            offline: false,
            api_key: None,
            base_url: None,
            model: "embed-v4.0".into(),
            dimension: 1536,
        };
        let err = match resolve_embedder(&selection, None) {
            Err(e) => e,
            Ok(_) => panic!("expected resolve_embedder to fail without local-embed or key"),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("--offline") && msg.contains("Cohere"),
            "error should ask for --offline or a Cohere key, got: {msg}"
        );
    }

    #[test]
    fn test_resolve_remote_delegates_model_name() {
        let tmp = tempfile::tempdir().unwrap();
        let index_file = tmp.path().join("vector_store.bin");
        let selection = EmbedderSelection {
            offline: false,
            api_key: Some("test-key".into()),
            base_url: None,
            model: "embed-v4.0".into(),
            dimension: 1536,
        };
        let embedder = resolve_embedder(&selection, Some(&index_file)).unwrap();
        assert_eq!(embedder.model_name(), "embed-v4.0");
        assert_eq!(embedder.dimension(), 1536);
        assert!(
            !embed_cache_path(&index_file).exists(),
            "cache file is created on miss, not at construction"
        );
    }
}
