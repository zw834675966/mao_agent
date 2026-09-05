use crate::corpus::chunker::{ChineseSemanticChunker, ChunkerConfig};
use crate::corpus::ingest::CorpusScanner;
use crate::error::{Result, VectorError};
use crate::model::{
    Document, DocumentChunk, VectorEntry, VectorFilter, VectorSearchResult, VectorStoreStats,
};
use crate::vector::embedder::{DeterministicEmbedder, Embedder, create_embedder_arc};
use crate::vector::index::VectorIndex;
use crate::vector::persist::{self, SnapshotIdentity};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// High-level VectorStore managing embedding models, vector index, and document chunking.
pub struct VectorStore {
    index: Arc<RwLock<VectorIndex>>,
    embedder: Arc<dyn Embedder>,
    chunker: ChineseSemanticChunker,
}

impl VectorStore {
    /// Create a new VectorStore with a given Embedder and Chunker configuration.
    pub fn new(embedder: Arc<dyn Embedder>, chunker_config: Option<ChunkerConfig>) -> Self {
        let dimension = embedder.dimension();
        Self {
            index: Arc::new(RwLock::new(VectorIndex::new(dimension))),
            embedder,
            chunker: ChineseSemanticChunker::new(chunker_config.unwrap_or_default()),
        }
    }

    /// Create an offline, lightweight VectorStore for testing (using DeterministicEmbedder).
    pub fn new_deterministic(dimension: usize) -> Self {
        let embedder = create_embedder_arc(DeterministicEmbedder::new(dimension));
        Self::new(embedder, None)
    }

    /// Number of vector entries in the store.
    pub async fn len(&self) -> usize {
        self.index.read().await.len()
    }

    /// Check if the store is empty.
    pub async fn is_empty(&self) -> bool {
        self.index.read().await.is_empty()
    }

    /// Embed a text query and search for top-K matching chunks.
    pub async fn search(
        &self,
        query: &str,
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<VectorSearchResult>> {
        self.search_with_force_brute(query, top_k, filter, false)
            .await
    }

    /// Embed + search, optionally forcing brute-force (for HNSW eval comparison).
    pub async fn search_with_force_brute(
        &self,
        query: &str,
        top_k: usize,
        filter: Option<&VectorFilter>,
        force_brute: bool,
    ) -> Result<Vec<VectorSearchResult>> {
        let query_vector = self.embedder.embed(query).await?;
        let index_guard = self.index.read().await;
        index_guard.search_with_force_brute(&query_vector, top_k, filter, force_brute)
    }

    /// Search directly using an embedding vector.
    pub async fn search_vector(
        &self,
        query_vector: &[f32],
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<VectorSearchResult>> {
        let index_guard = self.index.read().await;
        index_guard.search(query_vector, top_k, filter)
    }

    /// Vector search with optional brute-force force flag.
    pub async fn search_vector_with_force_brute(
        &self,
        query_vector: &[f32],
        top_k: usize,
        filter: Option<&VectorFilter>,
        force_brute: bool,
    ) -> Result<Vec<VectorSearchResult>> {
        let index_guard = self.index.read().await;
        index_guard.search_with_force_brute(query_vector, top_k, filter, force_brute)
    }

    /// Index a single document (splits into chunks, embeds in batch, inserts into index).
    pub async fn index_document(&self, doc: &Document) -> Result<usize> {
        let chunks = self.chunker.chunk_document(doc);
        if chunks.is_empty() {
            return Ok(0);
        }

        let count = chunks.len();
        self.index_chunks(chunks).await?;
        Ok(count)
    }

    /// Index a pre-chunked list of DocumentChunks.
    pub async fn index_chunks(&self, chunks: Vec<DocumentChunk>) -> Result<()> {
        if chunks.is_empty() {
            return Ok(());
        }

        // Collect contextualized texts for embedding
        let texts: Vec<String> = chunks
            .iter()
            .map(|c| c.contextualized_text.clone())
            .collect();
        let vectors = self.embedder.embed_batch(&texts).await?;

        if vectors.len() != chunks.len() {
            return Err(VectorError::EmbeddingError(format!(
                "Batch size mismatch: expected {} vectors, got {}",
                chunks.len(),
                vectors.len()
            )));
        }

        let entries: Vec<VectorEntry> = chunks
            .into_iter()
            .zip(vectors)
            .map(|(chunk, vector)| VectorEntry {
                id: chunk.chunk_id.clone(),
                vector,
                chunk,
            })
            .collect();

        let mut index_guard = self.index.write().await;
        index_guard.insert_batch(entries)?;
        Ok(())
    }

    /// Batch index multiple documents with a specified batch size.
    pub async fn index_documents(&self, docs: &[Document], batch_size: usize) -> Result<usize> {
        let mut all_chunks = Vec::new();
        for doc in docs {
            let chunks = self.chunker.chunk_document(doc);
            all_chunks.extend(chunks);
        }

        let total_chunks = all_chunks.len();
        info!(
            "Indexing {} total chunks across {} documents",
            total_chunks,
            docs.len()
        );

        for chunk_batch in all_chunks.chunks(batch_size) {
            self.index_chunks(chunk_batch.to_vec()).await?;
        }

        info!(
            "Successfully indexed all {} chunks into vector store",
            total_chunks
        );
        Ok(total_chunks)
    }

    /// Recursively scan and index an entire corpus directory of Markdown documents.
    pub async fn index_corpus_dir<P: AsRef<Path>>(
        &self,
        dir: P,
        batch_size: usize,
    ) -> Result<usize> {
        let docs = CorpusScanner::load_documents_from_dir(dir)?;
        if docs.is_empty() {
            warn!("No documents found to index");
            return Ok(0);
        }
        self.index_documents(&docs, batch_size).await
    }

    /// Compute statistics of the vector store.
    pub async fn stats(&self) -> VectorStoreStats {
        self.index.read().await.compute_stats()
    }

    /// Clear all data in the vector store.
    pub async fn clear(&self) {
        self.index.write().await.clear();
    }

    /// Save vector store index snapshot to a file atomically using bincode serialization.
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let target_path = path.as_ref();
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let index_guard = self.index.read().await;
        let identity = SnapshotIdentity {
            model: self.embedder.model_name().to_string(),
            dimension: index_guard.dimension(),
        };
        let encoded = persist::encode_snapshot(&identity, &index_guard)?;
        persist::atomic_replace(target_path, &encoded)?;

        info!(
            "Saved vector index snapshot ({} bytes) to {}",
            std::fs::metadata(target_path)?.len(),
            target_path.display()
        );
        Ok(())
    }

    /// Load vector store index snapshot from a file.
    pub fn load_from_file<P: AsRef<Path>>(path: P, embedder: Arc<dyn Embedder>) -> Result<Self> {
        let target_path = path.as_ref();
        if !target_path.exists() {
            return Err(VectorError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Index file not found: {}", target_path.display()),
            )));
        }

        let (identity, index) = persist::load_snapshot(target_path)?;

        if let Some(identity) = identity
            && (identity.model != embedder.model_name()
                || identity.dimension != embedder.dimension())
        {
            return Err(VectorError::IdentityMismatch {
                snapshot_model: identity.model,
                snapshot_dimension: identity.dimension,
                source_model: embedder.model_name().to_string(),
                source_dimension: embedder.dimension(),
            });
        }

        if index.dimension() != embedder.dimension() {
            return Err(VectorError::DimensionMismatch {
                expected: embedder.dimension(),
                actual: index.dimension(),
            });
        }

        info!(
            "Loaded vector index with {} vectors from {}",
            index.len(),
            target_path.display()
        );

        Ok(Self {
            index: Arc::new(RwLock::new(index)),
            embedder,
            chunker: ChineseSemanticChunker::new(ChunkerConfig::default()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DocumentMetadata, HistoricalPeriod};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_vector_store_e2e() {
        let store = VectorStore::new_deterministic(128);

        let doc1 = Document {
            id: "doc_1".to_string(),
            metadata: DocumentMetadata {
                title: "论持久战".to_string(),
                author: "毛泽东".to_string(),
                date: "1938-05".to_string(),
                period: "抗日战争时期".to_string(),
                volume: "毛泽东选集第二卷".to_string(),
                category: "军事".to_string(),
                tags: vec!["持久战".to_string(), "抗战".to_string()],
                ..Default::default()
            },
            period_enum: HistoricalPeriod::WarOfResistance,
            headnote: None,
            content: "中日战争是持久战，最后的胜利是中国的。战争的三个阶段：战略防御、战略相持、战略反攻。".to_string(),
            footnotes: vec![],
            file_path: None,
        };

        let doc2 = Document {
            id: "doc_2".to_string(),
            metadata: DocumentMetadata {
                title: "矛盾论".to_string(),
                author: "毛泽东".to_string(),
                date: "1937-08".to_string(),
                period: "土地革命战争时期".to_string(),
                volume: "毛泽东选集第一卷".to_string(),
                category: "哲学".to_string(),
                tags: vec!["唯物辩证法".to_string(), "对立统一".to_string()],
                ..Default::default()
            },
            period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
            headnote: None,
            content: "事物的矛盾法则，即对立统一的法则，是唯物辩证法的最根本的法则。主要矛盾和主要矛盾方面。".to_string(),
            footnotes: vec![],
            file_path: None,
        };

        let indexed = store.index_documents(&[doc1, doc2], 10).await.unwrap();
        assert_eq!(indexed, 2);
        assert_eq!(store.len().await, 2);

        // Search query matching doc1
        let results = store.search("持久战的三个阶段", 5, None).await.unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].chunk.doc_title, "论持久战");

        // Filter search matching doc2
        let filter = VectorFilter::new().with_period(HistoricalPeriod::AgrarianRevolutionaryWar);
        let results_filtered = store
            .search("矛盾与唯物辩证法", 5, Some(&filter))
            .await
            .unwrap();
        assert_eq!(results_filtered.len(), 1);
        assert_eq!(results_filtered[0].chunk.doc_title, "矛盾论");

        // Persistence test
        let tmp = tempdir().unwrap();
        let index_path = tmp.path().join("test_index.bin");
        store.save_to_file(&index_path).await.unwrap();

        let embedder = create_embedder_arc(DeterministicEmbedder::new(128));
        let loaded_store = VectorStore::load_from_file(&index_path, embedder).unwrap();
        assert_eq!(loaded_store.len().await, 2);

        let stats = loaded_store.stats().await;
        assert_eq!(stats.total_vectors, 2);
        assert_eq!(stats.total_documents, 2);
    }
}
