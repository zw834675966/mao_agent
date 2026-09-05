use crate::error::{Result, VectorError};
use crate::vector::embedder::Embedder;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// On-disk embed cache: `{index_file}.embedcache` (suffix, not `with_extension`).
pub(crate) fn embed_cache_path(index_file: &Path) -> PathBuf {
    let mut os = index_file.as_os_str().to_os_string();
    os.push(".embedcache");
    PathBuf::from(os)
}

#[derive(Serialize, Deserialize)]
struct EmbedCacheFile {
    model: String,
    dimension: usize,
    entries: HashMap<String, Vec<f32>>,
}

/// SHA-256 disk cache decorator. Identity (`model_name` / `dimension`) delegates to `inner`.
pub(crate) struct CachedEmbedder {
    inner: Arc<dyn Embedder>,
    path: PathBuf,
    entries: Mutex<HashMap<String, Vec<f32>>>,
}

impl CachedEmbedder {
    pub fn new(inner: Arc<dyn Embedder>, cache_path: impl AsRef<Path>) -> Result<Self> {
        let path = cache_path.as_ref().to_path_buf();
        let entries = load_entries(&path, inner.model_name(), inner.dimension());
        Ok(Self {
            inner,
            path,
            entries: Mutex::new(entries),
        })
    }

    fn lock_entries(&self) -> Result<std::sync::MutexGuard<'_, HashMap<String, Vec<f32>>>> {
        self.entries
            .lock()
            .map_err(|_| VectorError::EmbeddingError("embed cache lock poisoned".into()))
    }

    fn persist(
        path: &Path,
        model: &str,
        dimension: usize,
        entries: &HashMap<String, Vec<f32>>,
    ) -> Result<()> {
        let file = EmbedCacheFile {
            model: model.to_string(),
            dimension,
            entries: entries.clone(),
        };
        let encoded = bincode::serialize(&file)?;
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }
        crate::vector::persist::atomic_replace(path, &encoded)
    }
}

fn load_entries(path: &Path, model: &str, dimension: usize) -> HashMap<String, Vec<f32>> {
    let Ok(bytes) = std::fs::read(path) else {
        return HashMap::new();
    };
    let Ok(file) = bincode::deserialize::<EmbedCacheFile>(&bytes) else {
        return HashMap::new();
    };
    if file.model != model || file.dimension != dimension {
        return HashMap::new();
    }
    file.entries
}

fn sha256_hex(text: &str) -> String {
    use sha2::{Digest, Sha256};
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(text.as_bytes());
    let mut out = String::with_capacity(digest.len() * 2);
    for &b in digest.as_slice() {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}

#[async_trait]
impl Embedder for CachedEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let mut batch = self.embed_batch(&[text.to_string()]).await?;
        batch.pop().ok_or(VectorError::EmptyVector)
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let keys: Vec<String> = texts.iter().map(|t| sha256_hex(t)).collect();
        let mut results: Vec<Option<Vec<f32>>> = vec![None; texts.len()];
        let mut miss_idx: Vec<usize> = Vec::new();

        {
            let cache = self.lock_entries()?;
            for (i, key) in keys.iter().enumerate() {
                if let Some(v) = cache.get(key) {
                    results[i] = Some(v.clone());
                } else {
                    miss_idx.push(i);
                }
            }
        }

        if !miss_idx.is_empty() {
            let miss_texts: Vec<String> = miss_idx.iter().map(|&i| texts[i].clone()).collect();
            let miss_vecs = self.inner.embed_batch(&miss_texts).await?;
            if miss_vecs.len() != miss_texts.len() {
                return Err(VectorError::EmbeddingError(format!(
                    "embed cache inner returned {} vectors for {} texts",
                    miss_vecs.len(),
                    miss_texts.len()
                )));
            }
            {
                let mut cache = self.lock_entries()?;
                for (i, vec) in miss_idx.iter().zip(miss_vecs) {
                    cache.insert(keys[*i].clone(), vec.clone());
                    results[*i] = Some(vec);
                }
                Self::persist(
                    &self.path,
                    self.inner.model_name(),
                    self.inner.dimension(),
                    &cache,
                )?;
            }
        }

        results
            .into_iter()
            .map(|v| v.ok_or(VectorError::EmptyVector))
            .collect()
    }

    fn dimension(&self) -> usize {
        self.inner.dimension()
    }

    fn model_name(&self) -> &str {
        self.inner.model_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingEmbedder {
        calls: AtomicUsize,
        dimension: usize,
        model_name: String,
    }

    impl CountingEmbedder {
        fn new(dimension: usize, model_name: &str) -> Self {
            Self {
                calls: AtomicUsize::new(0),
                dimension,
                model_name: model_name.to_string(),
            }
        }

        fn calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl Embedder for CountingEmbedder {
        async fn embed(&self, text: &str) -> Result<Vec<f32>> {
            let mut batch = self.embed_batch(&[text.to_string()]).await?;
            batch.pop().ok_or(VectorError::EmptyVector)
        }

        async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let mut unit = vec![0.0f32; self.dimension];
            if self.dimension > 0 {
                unit[0] = 1.0;
            }
            Ok(texts.iter().map(|_| unit.clone()).collect())
        }

        fn dimension(&self) -> usize {
            self.dimension
        }

        fn model_name(&self) -> &str {
            &self.model_name
        }
    }

    #[tokio::test]
    async fn test_embed_batch_cache_hit_skips_inner() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("store.bin.embedcache");
        let inner = Arc::new(CountingEmbedder::new(8, "embed-v4.0"));
        let cached = CachedEmbedder::new(inner.clone(), &path).unwrap();

        let batch = vec!["捉住主要矛盾".to_string()];
        let first = cached.embed_batch(&batch).await.unwrap();
        assert_eq!(inner.calls(), 1);
        assert_eq!(first[0].len(), 8);
        assert_eq!(first[0][0], 1.0);

        let second = cached.embed_batch(&batch).await.unwrap();
        assert_eq!(inner.calls(), 1);
        assert_eq!(first, second);
        assert_eq!(cached.model_name(), "embed-v4.0");

        let inner2 = Arc::new(CountingEmbedder::new(8, "embed-v4.0"));
        let reloaded = CachedEmbedder::new(inner2.clone(), &path).unwrap();
        let third = reloaded.embed_batch(&batch).await.unwrap();
        assert_eq!(inner2.calls(), 0);
        assert_eq!(third, first);
    }

    #[tokio::test]
    async fn test_cache_model_or_dim_mismatch_is_discarded() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("store.bin.embedcache");

        let mut entries = HashMap::new();
        entries.insert(sha256_hex("捉住主要矛盾"), vec![1.0f32; 8]);
        let stale = EmbedCacheFile {
            model: "other-model".into(),
            dimension: 8,
            entries,
        };
        std::fs::write(&path, bincode::serialize(&stale).unwrap()).unwrap();

        let inner = Arc::new(CountingEmbedder::new(8, "embed-v4.0"));
        let cached = CachedEmbedder::new(inner.clone(), &path).unwrap();
        let _ = cached
            .embed_batch(&["捉住主要矛盾".to_string()])
            .await
            .unwrap();
        assert_eq!(
            inner.calls(),
            1,
            "stale model header must not satisfy a hit"
        );

        let dim_path = tmp.path().join("dim.bin.embedcache");
        let mut dim_entries = HashMap::new();
        dim_entries.insert(sha256_hex("捉住主要矛盾"), vec![1.0f32; 4]);
        let wrong_dim = EmbedCacheFile {
            model: "embed-v4.0".into(),
            dimension: 4,
            entries: dim_entries,
        };
        std::fs::write(&dim_path, bincode::serialize(&wrong_dim).unwrap()).unwrap();

        let inner_dim = Arc::new(CountingEmbedder::new(8, "embed-v4.0"));
        let cached_dim = CachedEmbedder::new(inner_dim.clone(), &dim_path).unwrap();
        let _ = cached_dim
            .embed_batch(&["捉住主要矛盾".to_string()])
            .await
            .unwrap();
        assert_eq!(
            inner_dim.calls(),
            1,
            "stale dimension header must not satisfy a hit"
        );
    }
}
