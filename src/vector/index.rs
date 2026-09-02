use crate::error::{Result, VectorError};
use crate::model::{
    HistoricalPeriod, VectorEntry, VectorFilter, VectorSearchResult, VectorStoreStats,
};
use crate::vector::math::{dot_product, normalize_in_place};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::debug;

/// High-performance in-memory Vector Index with fast inverted indices for metadata filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorIndex {
    /// Dimension of the vectors in this index
    dimension: usize,
    /// Stored entries containing vector and document chunk
    entries: Vec<VectorEntry>,
    /// Map from entry ID (chunk_id) to index in `entries`
    id_to_idx: HashMap<String, usize>,
    /// Inverted index for historical periods
    period_index: HashMap<HistoricalPeriod, Vec<usize>>,
    /// Inverted index for volume/collection names
    volume_index: HashMap<String, Vec<usize>>,
    /// Inverted index for document IDs
    doc_index: HashMap<String, Vec<usize>>,
    /// Inverted index for tags
    tag_index: HashMap<String, Vec<usize>>,
}

impl VectorIndex {
    /// Create a new empty VectorIndex with expected vector dimension.
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            entries: Vec::new(),
            id_to_idx: HashMap::new(),
            period_index: HashMap::new(),
            volume_index: HashMap::new(),
            doc_index: HashMap::new(),
            tag_index: HashMap::new(),
        }
    }

    /// Number of vector entries currently stored.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Expected vector dimension.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Insert a single VectorEntry into the index.
    pub fn insert(&mut self, mut entry: VectorEntry) -> Result<()> {
        if entry.vector.len() != self.dimension {
            return Err(VectorError::DimensionMismatch {
                expected: self.dimension,
                actual: entry.vector.len(),
            });
        }

        // Ensure vector is unit-normalized for fast dot-product cosine similarity
        normalize_in_place(&mut entry.vector);

        let id = entry.id.clone();
        if let Some(&existing_idx) = self.id_to_idx.get(&id) {
            // Update existing entry
            self.entries[existing_idx] = entry;
            self.rebuild_inverted_indices();
        } else {
            let idx = self.entries.len();
            // Update inverted indices
            self.period_index
                .entry(entry.chunk.period)
                .or_default()
                .push(idx);
            if !entry.chunk.volume.is_empty() {
                self.volume_index
                    .entry(entry.chunk.volume.clone())
                    .or_default()
                    .push(idx);
            }
            self.doc_index
                .entry(entry.chunk.doc_id.clone())
                .or_default()
                .push(idx);
            for tag in &entry.chunk.tags {
                self.tag_index.entry(tag.clone()).or_default().push(idx);
            }

            self.id_to_idx.insert(id, idx);
            self.entries.push(entry);
        }

        Ok(())
    }

    /// Batch insert multiple VectorEntries.
    pub fn insert_batch(&mut self, entries: Vec<VectorEntry>) -> Result<()> {
        for entry in entries {
            self.insert(entry)?;
        }
        Ok(())
    }

    /// Search the index using a normalized query vector, returning top-k matching results.
    pub fn search(
        &self,
        query_vector: &[f32],
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<VectorSearchResult>> {
        if query_vector.len() != self.dimension {
            return Err(VectorError::DimensionMismatch {
                expected: self.dimension,
                actual: query_vector.len(),
            });
        }
        if self.entries.is_empty() || top_k == 0 {
            return Ok(Vec::new());
        }

        let mut norm_query = query_vector.to_vec();
        normalize_in_place(&mut norm_query);

        // Determine candidate indices using inverted index if filter is provided
        let candidate_indices = self.resolve_filter_candidates(filter);

        let mut scored_items: Vec<(usize, f32)> = match candidate_indices {
            Some(indices) => {
                let mut scores = Vec::with_capacity(indices.len());
                for &idx in &indices {
                    if let Some(entry) = self.entries.get(idx) {
                        // Apply additional fine-grained predicate checks
                        if let Some(f) = filter
                            && !f.matches(&entry.chunk)
                        {
                            continue;
                        }
                        let score = dot_product(&norm_query, &entry.vector);
                        scores.push((idx, score));
                    }
                }
                scores
            }
            None => {
                let mut scores = Vec::with_capacity(self.entries.len());
                for (idx, entry) in self.entries.iter().enumerate() {
                    if let Some(f) = filter
                        && !f.matches(&entry.chunk)
                    {
                        continue;
                    }
                    let score = dot_product(&norm_query, &entry.vector);
                    scores.push((idx, score));
                }
                scores
            }
        };

        // Sort descending by similarity score
        scored_items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored_items.truncate(top_k);

        let results: Vec<VectorSearchResult> = scored_items
            .into_iter()
            .enumerate()
            .map(|(rank, (idx, score))| {
                let entry = &self.entries[idx];
                VectorSearchResult {
                    chunk_id: entry.id.clone(),
                    score,
                    rank: rank + 1,
                    chunk: entry.chunk.clone(),
                }
            })
            .collect();

        debug!(
            "Vector search completed, retrieved {} results",
            results.len()
        );
        Ok(results)
    }

    /// Resolve initial candidate indices from inverted index.
    fn resolve_filter_candidates(&self, filter: Option<&VectorFilter>) -> Option<Vec<usize>> {
        let f = filter?;

        let mut candidate_set: Option<HashSet<usize>> = None;

        // Filter by period
        if let Some(period) = f.period {
            if let Some(indices) = self.period_index.get(&period) {
                candidate_set = Some(indices.iter().copied().collect());
            } else {
                return Some(Vec::new());
            }
        }

        // Filter by doc_id
        if let Some(ref doc_id) = f.doc_id {
            if let Some(indices) = self.doc_index.get(doc_id) {
                let set: HashSet<usize> = indices.iter().copied().collect();
                candidate_set = match candidate_set {
                    Some(existing) => Some(existing.intersection(&set).copied().collect()),
                    None => Some(set),
                };
            } else {
                return Some(Vec::new());
            }
        }

        candidate_set.map(|s| s.into_iter().collect())
    }

    /// Retrieve an entry by its ID.
    pub fn get(&self, id: &str) -> Option<&VectorEntry> {
        self.id_to_idx
            .get(id)
            .and_then(|&idx| self.entries.get(idx))
    }

    /// Delete an entry by its ID.
    pub fn delete(&mut self, id: &str) -> bool {
        if let Some(idx) = self.id_to_idx.remove(id) {
            self.entries.remove(idx);
            self.rebuild_inverted_indices();
            true
        } else {
            false
        }
    }

    /// Clear all entries and indices.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.id_to_idx.clear();
        self.period_index.clear();
        self.volume_index.clear();
        self.doc_index.clear();
        self.tag_index.clear();
    }

    /// Calculate comprehensive statistics for the vector index.
    pub fn compute_stats(&self) -> VectorStoreStats {
        let mut period_dist = HashMap::new();
        let mut volume_dist = HashMap::new();
        let mut doc_ids = HashSet::new();
        let mut total_chars = 0;

        for entry in &self.entries {
            *period_dist
                .entry(entry.chunk.period.as_str().to_string())
                .or_insert(0) += 1;
            if !entry.chunk.volume.is_empty() {
                *volume_dist.entry(entry.chunk.volume.clone()).or_insert(0) += 1;
            }
            doc_ids.insert(&entry.chunk.doc_id);
            total_chars += entry.chunk.char_count;
        }

        let vector_bytes = self.entries.len() * self.dimension * std::mem::size_of::<f32>();
        let approx_text_bytes = total_chars * 3; // UTF-8 Chinese characters approx 3 bytes
        let estimated_memory_bytes = vector_bytes + approx_text_bytes + std::mem::size_of::<Self>();

        VectorStoreStats {
            total_vectors: self.entries.len(),
            total_documents: doc_ids.len(),
            vector_dimension: self.dimension,
            period_distribution: period_dist,
            volume_distribution: volume_dist,
            total_characters_indexed: total_chars,
            estimated_memory_bytes,
        }
    }

    fn rebuild_inverted_indices(&mut self) {
        self.id_to_idx.clear();
        self.period_index.clear();
        self.volume_index.clear();
        self.doc_index.clear();
        self.tag_index.clear();

        for (idx, entry) in self.entries.iter().enumerate() {
            self.id_to_idx.insert(entry.id.clone(), idx);
            self.period_index
                .entry(entry.chunk.period)
                .or_default()
                .push(idx);
            if !entry.chunk.volume.is_empty() {
                self.volume_index
                    .entry(entry.chunk.volume.clone())
                    .or_default()
                    .push(idx);
            }
            self.doc_index
                .entry(entry.chunk.doc_id.clone())
                .or_default()
                .push(idx);
            for tag in &entry.chunk.tags {
                self.tag_index.entry(tag.clone()).or_default().push(idx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DocumentChunk;

    fn create_dummy_chunk(
        id: &str,
        title: &str,
        period: HistoricalPeriod,
        volume: &str,
    ) -> DocumentChunk {
        DocumentChunk {
            chunk_id: id.to_string(),
            doc_id: format!("doc_{}", id),
            doc_title: title.to_string(),
            author: "毛泽东".to_string(),
            period,
            date: "1938-05".to_string(),
            volume: volume.to_string(),
            category: "军事".to_string(),
            tags: vec!["战略".to_string()],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 50,
            raw_text: "战略问题是研究战争全局的规律性的东西".to_string(),
            contextualized_text: format!(
                "【文献】《{}》\n【正文】战略问题是研究战争全局的规律性的东西",
                title
            ),
            section_path: vec!["战略问题".to_string()],
        }
    }

    #[test]
    fn test_vector_index_crud_and_search() {
        let mut index = VectorIndex::new(3);

        let entry1 = VectorEntry {
            id: "c1".to_string(),
            vector: vec![1.0, 0.0, 0.0],
            chunk: create_dummy_chunk(
                "c1",
                "论持久战",
                HistoricalPeriod::WarOfResistance,
                "选集第二卷",
            ),
        };

        let entry2 = VectorEntry {
            id: "c2".to_string(),
            vector: vec![0.0, 1.0, 0.0],
            chunk: create_dummy_chunk(
                "c2",
                "矛盾论",
                HistoricalPeriod::AgrarianRevolutionaryWar,
                "选集第一卷",
            ),
        };

        index.insert(entry1).unwrap();
        index.insert(entry2).unwrap();
        assert_eq!(index.len(), 2);

        // Search with query vector closest to entry1
        let query = vec![0.9, 0.1, 0.0];
        let results = index.search(&query, 5, None).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].chunk_id, "c1");
        assert!(results[0].score > 0.9);

        // Search with period filter
        let filter = VectorFilter::new().with_period(HistoricalPeriod::AgrarianRevolutionaryWar);
        let filtered_results = index.search(&query, 5, Some(&filter)).unwrap();
        assert_eq!(filtered_results.len(), 1);
        assert_eq!(filtered_results[0].chunk_id, "c2");
    }
}
