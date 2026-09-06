use crate::error::{Result, VectorError};
use crate::model::{
    HistoricalPeriod, VectorEntry, VectorFilter, VectorSearchResult, VectorStoreStats,
};
use crate::vector::math::{dot_product, normalize_in_place};
use hnswlib_rs::{Cosine, Hnsw, HnswConfig, InMemoryVectorStore};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, info, warn};

/// Switch to ANN when the index reaches this many vectors (production default).
pub const HNSW_THRESHOLD: usize = 5000;
const HNSW_M: usize = 16;
const HNSW_EF_CONSTRUCTION: usize = 200;
const HNSW_EF_SEARCH: usize = 200;

/// Test-only override; `0` means use [`HNSW_THRESHOLD`].
static HNSW_THRESHOLD_OVERRIDE: AtomicUsize = AtomicUsize::new(0);

/// Lower/raise the ANN activation threshold for regression tests.
#[doc(hidden)]
pub fn set_hnsw_threshold_for_test(threshold: usize) {
    HNSW_THRESHOLD_OVERRIDE.store(threshold, Ordering::SeqCst);
}

/// Restore the production [`HNSW_THRESHOLD`] after tests.
#[doc(hidden)]
pub fn reset_hnsw_threshold_for_test() {
    HNSW_THRESHOLD_OVERRIDE.store(0, Ordering::SeqCst);
}

fn effective_hnsw_threshold() -> usize {
    let override_thr = HNSW_THRESHOLD_OVERRIDE.load(Ordering::SeqCst);
    if override_thr == 0 {
        HNSW_THRESHOLD
    } else {
        override_thr
    }
}

/// In-memory HNSW graph + dense vector store (not persisted; rebuilt on load).
struct HnswAnn {
    graph: Hnsw<String, Cosine>,
    vectors: InMemoryVectorStore<f32>,
}

impl HnswAnn {
    fn build(entries: &[VectorEntry], dimension: usize) -> Option<Self> {
        if entries.is_empty() {
            return None;
        }
        let n = entries.len();
        // Headroom for incremental inserts before a full rebuild is required.
        let max_nodes = n
            .saturating_mul(2)
            .max(n + 1024)
            .max(effective_hnsw_threshold());
        let cfg = HnswConfig::new(dimension, max_nodes)
            .m(HNSW_M)
            .ef_construction(HNSW_EF_CONSTRUCTION)
            .ef_search(HNSW_EF_SEARCH)
            .seed(42);
        let graph = Hnsw::new(Cosine::new(), cfg);
        let vectors = InMemoryVectorStore::<f32>::new(dimension, max_nodes);
        for entry in entries {
            if let Err(err) = graph.insert(&vectors, entry.id.clone(), entry.vector.as_slice()) {
                warn!(
                    chunk_id = %entry.id,
                    error = %err,
                    "HNSW insert failed during rebuild; continuing"
                );
            }
        }
        debug!(nodes = graph.len(), "HNSW graph rebuilt");
        Some(Self { graph, vectors })
    }

    fn insert_one(&self, id: &str, vector: &[f32]) -> bool {
        match self.graph.insert(&self.vectors, id.to_string(), vector) {
            Ok(()) => true,
            Err(err) => {
                warn!(chunk_id = %id, error = %err, "HNSW incremental insert failed");
                false
            }
        }
    }

    fn set_one(&self, id: &str, vector: &[f32]) -> bool {
        match self.graph.set(&self.vectors, id.to_string(), vector) {
            Ok(_) => true,
            Err(err) => {
                warn!(chunk_id = %id, error = %err, "HNSW set failed");
                false
            }
        }
    }

    fn search_keys(&self, query: &[f32], k: usize) -> Result<Vec<String>> {
        let ef = k.max(HNSW_EF_SEARCH);
        self.graph.set_ef_search(ef);
        let hits = self
            .graph
            .search(&self.vectors, query, k, None)
            .map_err(|e| VectorError::Other(format!("HNSW search failed: {e}")))?;
        Ok(hits.into_iter().map(|h| h.key).collect())
    }
}

/// High-performance in-memory Vector Index with fast inverted indices for metadata filtering.
#[derive(Serialize, Deserialize)]
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
    /// Memory-only HNSW ANN state (skipped in snapshots; rebuilt on load).
    #[serde(skip)]
    hnsw: Option<HnswAnn>,
}

impl fmt::Debug for VectorIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VectorIndex")
            .field("dimension", &self.dimension)
            .field("entries", &self.entries.len())
            .field("hnsw", &self.hnsw.is_some())
            .finish()
    }
}

impl Clone for VectorIndex {
    fn clone(&self) -> Self {
        let mut cloned = Self {
            dimension: self.dimension,
            entries: self.entries.clone(),
            id_to_idx: self.id_to_idx.clone(),
            period_index: self.period_index.clone(),
            volume_index: self.volume_index.clone(),
            doc_index: self.doc_index.clone(),
            tag_index: self.tag_index.clone(),
            hnsw: None,
        };
        cloned.rebuild_hnsw();
        cloned
    }
}
/// Extract canonical volume lookup keys (e.g. "第二卷", "选集第二卷", "毛泽东选集第二卷")
/// for O(1) inverted index resolution.
pub(crate) fn extract_volume_lookup_keys(volume: &str) -> Vec<String> {
    let trimmed = volume.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    let mut keys = HashSet::new();
    keys.insert(trimmed.to_string());

    if let Some(stripped) = trimmed.strip_prefix("毛泽东") {
        keys.insert(stripped.to_string());
        if let Some(s2) = stripped
            .strip_prefix("选集")
            .or_else(|| stripped.strip_prefix("文集"))
        {
            keys.insert(s2.to_string());
        }
    } else if let Some(s2) = trimmed
        .strip_prefix("选集")
        .or_else(|| trimmed.strip_prefix("文集"))
    {
        keys.insert(s2.to_string());
    }

    if !trimmed.contains("选集") && !trimmed.contains("文集") {
        keys.insert(format!("选集{trimmed}"));
        keys.insert(format!("毛泽东选集{trimmed}"));
    } else if !trimmed.starts_with("毛泽东") {
        keys.insert(format!("毛泽东{trimmed}"));
    }

    keys.into_iter().collect()
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
            hnsw: None,
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

    /// Whether an HNSW ANN graph is currently active.
    pub fn has_hnsw(&self) -> bool {
        self.hnsw.is_some()
    }

    /// Rebuild the in-memory HNSW graph when `len >= threshold`; otherwise clear it.
    pub fn rebuild_hnsw(&mut self) {
        if self.entries.len() < effective_hnsw_threshold() {
            self.hnsw = None;
            return;
        }
        let n = self.entries.len();
        let started = std::time::Instant::now();
        self.hnsw = HnswAnn::build(&self.entries, self.dimension);
        info!(
            vectors = n,
            elapsed_ms = started.elapsed().as_millis() as u64,
            "HNSW graph rebuilt"
        );
    }

    fn maintain_hnsw_after_inserts(&mut self, new_ids: &[String], had_update: bool) {
        let thr = effective_hnsw_threshold();
        if self.entries.len() < thr {
            self.hnsw = None;
            return;
        }
        if self.hnsw.is_none() || had_update {
            self.rebuild_hnsw();
            return;
        }
        let Some(ann) = self.hnsw.as_ref() else {
            return;
        };
        for id in new_ids {
            let Some(entry) = self.get(id) else {
                continue;
            };
            if !ann.insert_one(id, &entry.vector) {
                self.rebuild_hnsw();
                return;
            }
        }
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
            if self.entries.len() >= effective_hnsw_threshold() {
                // Prefer in-place set; fall back to full rebuild on failure.
                let ok = self
                    .hnsw
                    .as_ref()
                    .map(|ann| {
                        let v = &self.entries[existing_idx].vector;
                        ann.set_one(&id, v)
                    })
                    .unwrap_or(false);
                if !ok {
                    self.rebuild_hnsw();
                }
            } else {
                self.hnsw = None;
            }
        } else {
            let idx = self.entries.len();
            // Update inverted indices
            self.period_index
                .entry(entry.chunk.period)
                .or_default()
                .push(idx);
            for v_key in extract_volume_lookup_keys(&entry.chunk.volume) {
                self.volume_index.entry(v_key).or_default().push(idx);
            }
            self.doc_index
                .entry(entry.chunk.doc_id.clone())
                .or_default()
                .push(idx);
            for tag in &entry.chunk.tags {
                self.tag_index
                    .entry(tag.trim().to_string())
                    .or_default()
                    .push(idx);
            }

            self.id_to_idx.insert(id.clone(), idx);
            self.entries.push(entry);
            self.maintain_hnsw_after_inserts(&[id], false);
        }

        Ok(())
    }

    /// Batch insert multiple VectorEntries.
    pub fn insert_batch(&mut self, entries: Vec<VectorEntry>) -> Result<()> {
        self.insert_batch_inner(entries, true)
    }

    /// Batch insert without maintaining the HNSW graph per batch. Callers that
    /// insert many batches in a loop should use this and then call
    /// [`rebuild_hnsw`](Self::rebuild_hnsw) once after all batches, avoiding a
    /// full graph rebuild per batch once the threshold is crossed.
    pub fn insert_batch_deferred(&mut self, entries: Vec<VectorEntry>) -> Result<()> {
        self.insert_batch_inner(entries, false)
    }

    fn insert_batch_inner(&mut self, entries: Vec<VectorEntry>, maintain_hnsw: bool) -> Result<()> {
        let mut had_update = false;
        let mut new_ids = Vec::new();
        for mut entry in entries {
            if entry.vector.len() != self.dimension {
                return Err(VectorError::DimensionMismatch {
                    expected: self.dimension,
                    actual: entry.vector.len(),
                });
            }
            normalize_in_place(&mut entry.vector);

            let id = entry.id.clone();
            if let Some(&existing_idx) = self.id_to_idx.get(&id) {
                self.entries[existing_idx] = entry;
                had_update = true;
            } else {
                let idx = self.entries.len();
                self.period_index
                    .entry(entry.chunk.period)
                    .or_default()
                    .push(idx);
                for v_key in extract_volume_lookup_keys(&entry.chunk.volume) {
                    self.volume_index.entry(v_key).or_default().push(idx);
                }
                self.doc_index
                    .entry(entry.chunk.doc_id.clone())
                    .or_default()
                    .push(idx);
                for tag in &entry.chunk.tags {
                    self.tag_index
                        .entry(tag.trim().to_string())
                        .or_default()
                        .push(idx);
                }
                self.id_to_idx.insert(id.clone(), idx);
                self.entries.push(entry);
                new_ids.push(id);
            }
        }

        if had_update {
            self.rebuild_inverted_indices();
        }
        if maintain_hnsw {
            self.maintain_hnsw_after_inserts(&new_ids, had_update);
        }
        Ok(())
    }
    /// Search the index using a normalized query vector, returning top-k matching results.
    ///
    /// When an HNSW graph is active and `force_brute` is false, candidates are retrieved via
    /// ANN then re-scored with exact dot-product; if post-filter yields fewer than `top_k`
    /// hits, the implementation falls back to a full brute-force scan.
    pub fn search(
        &self,
        query_vector: &[f32],
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<VectorSearchResult>> {
        self.search_with_force_brute(query_vector, top_k, filter, false)
    }

    /// Like [`search`](Self::search) but forces the exact brute-force path when `force_brute`.
    pub fn search_with_force_brute(
        &self,
        query_vector: &[f32],
        top_k: usize,
        filter: Option<&VectorFilter>,
        force_brute: bool,
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

        let use_ann = !force_brute && self.hnsw.is_some();
        if use_ann {
            match self.search_ann(&norm_query, top_k, filter) {
                Ok(results) if results.len() >= top_k.min(self.count_matching(filter)) => {
                    return Ok(results);
                }
                Ok(partial) if !partial.is_empty() => {
                    // Fall back to brute to fill / correct under-filtered ANN results.
                    debug!(
                        ann_hits = partial.len(),
                        top_k, "ANN post-filter under-filled; falling back to brute-force"
                    );
                    return self.search_brute(&norm_query, top_k, filter);
                }
                Ok(_) => {
                    return self.search_brute(&norm_query, top_k, filter);
                }
                Err(err) => {
                    warn!(error = %err, "ANN search failed; falling back to brute-force");
                    return self.search_brute(&norm_query, top_k, filter);
                }
            }
        }

        self.search_brute(&norm_query, top_k, filter)
    }

    fn count_matching(&self, filter: Option<&VectorFilter>) -> usize {
        match filter {
            None => self.entries.len(),
            Some(f) => self.entries.iter().filter(|e| f.matches(&e.chunk)).count(),
        }
    }

    fn search_ann(
        &self,
        norm_query: &[f32],
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<VectorSearchResult>> {
        let ann = self
            .hnsw
            .as_ref()
            .ok_or_else(|| VectorError::Other("HNSW graph missing".into()))?;
        let candidate_k = top_k.saturating_mul(20).max(HNSW_EF_SEARCH * 2);
        let keys = ann.search_keys(norm_query, candidate_k)?;

        let mut scored: Vec<(usize, f32)> = Vec::with_capacity(keys.len());
        for key in keys {
            let Some(&idx) = self.id_to_idx.get(&key) else {
                continue;
            };
            let Some(entry) = self.entries.get(idx) else {
                continue;
            };
            if let Some(f) = filter
                && !f.matches(&entry.chunk)
            {
                continue;
            }
            let score = dot_product(norm_query, &entry.vector);
            scored.push((idx, score));
        }

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);

        let results: Vec<VectorSearchResult> = scored
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
            "Vector ANN search completed, retrieved {} results",
            results.len()
        );
        Ok(results)
    }

    fn search_brute(
        &self,
        norm_query: &[f32],
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<VectorSearchResult>> {
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
                        let score = dot_product(norm_query, &entry.vector);
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
                    let score = dot_product(norm_query, &entry.vector);
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
            "Vector brute search completed, retrieved {} results",
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

        // Filter by multiple periods (guard empty periods vec)
        if let Some(ref periods) = f.periods
            && !periods.is_empty()
        {
            let mut p_set = HashSet::new();
            for p in periods {
                if let Some(indices) = self.period_index.get(p) {
                    p_set.extend(indices.iter().copied());
                }
            }
            candidate_set = match candidate_set {
                Some(existing) => Some(existing.intersection(&p_set).copied().collect()),
                None => Some(p_set),
            };
        }

        // Filter by volume (O(1) hash lookup with bidirectional fallback)
        if let Some(ref vol) = f.volume {
            let vol_trimmed = vol.trim();
            let vol_matched: HashSet<usize> =
                if let Some(indices) = self.volume_index.get(vol_trimmed) {
                    indices.iter().copied().collect()
                } else {
                    let mut matched = HashSet::new();
                    for (v_key, indices) in &self.volume_index {
                        if v_key.contains(vol_trimmed) || vol_trimmed.contains(v_key) {
                            matched.extend(indices.iter().copied());
                        }
                    }
                    matched
                };
            candidate_set = match candidate_set {
                Some(existing) => Some(existing.intersection(&vol_matched).copied().collect()),
                None => Some(vol_matched),
            };
        }

        // Filter by required tags (O(1) hash lookup with bidirectional fallback)
        if let Some(ref tags) = f.tags {
            for tag in tags {
                let tag_trimmed = tag.trim();
                let tag_matched: HashSet<usize> =
                    if let Some(indices) = self.tag_index.get(tag_trimmed) {
                        indices.iter().copied().collect()
                    } else {
                        let mut matched = HashSet::new();
                        for (t_key, indices) in &self.tag_index {
                            if t_key.contains(tag_trimmed) || tag_trimmed.contains(t_key) {
                                matched.extend(indices.iter().copied());
                            }
                        }
                        matched
                    };
                candidate_set = match candidate_set {
                    Some(existing) => Some(existing.intersection(&tag_matched).copied().collect()),
                    None => Some(tag_matched),
                };
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

    /// Resolve graph `source_refs` to chunks. Prefer `doc_id`; else title match.
    /// `section_path` is a prefix filter; if it yields nothing, fall back to title-only.
    pub fn chunks_matching(&self, r: &crate::graph::SourceRef) -> Vec<crate::model::DocumentChunk> {
        let with_section = self.chunks_matching_inner(r, true);
        if with_section.is_empty() && r.section_path.as_ref().is_some_and(|p| !p.is_empty()) {
            self.chunks_matching_inner(r, false)
        } else {
            with_section
        }
    }

    fn chunks_matching_inner(
        &self,
        r: &crate::graph::SourceRef,
        use_section: bool,
    ) -> Vec<crate::model::DocumentChunk> {
        let section_ok = |path: &[String]| -> bool {
            if !use_section {
                return true;
            }
            let Some(prefix) = r.section_path.as_deref() else {
                return true;
            };
            if prefix.is_empty() {
                return true;
            }
            path.starts_with(prefix)
        };

        if let Some(doc_id) = r.doc_id.as_deref()
            && let Some(idxs) = self.doc_index.get(doc_id)
        {
            return idxs
                .iter()
                .filter_map(|&i| self.entries.get(i))
                .filter(|e| section_ok(&e.chunk.section_path))
                .map(|e| e.chunk.clone())
                .collect();
        }

        self.entries
            .iter()
            .filter(|e| e.chunk.doc_title == r.doc_title && section_ok(&e.chunk.section_path))
            .map(|e| e.chunk.clone())
            .collect()
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
            self.rebuild_hnsw();
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
        self.hnsw = None;
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

    pub(crate) fn rebuild_inverted_indices(&mut self) {
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
            for v_key in extract_volume_lookup_keys(&entry.chunk.volume) {
                self.volume_index.entry(v_key).or_default().push(idx);
            }
            self.doc_index
                .entry(entry.chunk.doc_id.clone())
                .or_default()
                .push(idx);
            for tag in &entry.chunk.tags {
                self.tag_index
                    .entry(tag.trim().to_string())
                    .or_default()
                    .push(idx);
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
        assert!(!index.has_hnsw());

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

    #[test]
    fn test_hnsw_activates_at_threshold() {
        reset_hnsw_threshold_for_test();
        set_hnsw_threshold_for_test(50);
        let mut index = VectorIndex::new(8);
        for i in 0..49 {
            let mut v = vec![0.0_f32; 8];
            v[i % 8] = 1.0;
            let entry = VectorEntry {
                id: format!("e{i}"),
                vector: v,
                chunk: create_dummy_chunk(
                    &format!("e{i}"),
                    "t",
                    HistoricalPeriod::WarOfResistance,
                    "选集第二卷",
                ),
            };
            index.insert(entry).unwrap();
        }
        assert!(!index.has_hnsw());
        let mut v = vec![0.0_f32; 8];
        v[0] = 1.0;
        index
            .insert(VectorEntry {
                id: "e49".into(),
                vector: v,
                chunk: create_dummy_chunk(
                    "e49",
                    "t",
                    HistoricalPeriod::WarOfResistance,
                    "选集第二卷",
                ),
            })
            .unwrap();
        assert!(index.has_hnsw());
        assert_eq!(index.len(), 50);

        let query = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let ann = index
            .search_with_force_brute(&query, 5, None, false)
            .unwrap();
        let brute = index
            .search_with_force_brute(&query, 5, None, true)
            .unwrap();
        assert_eq!(ann.len(), 5);
        assert_eq!(brute.len(), 5);
        // Top-1 should match for this simple axis-aligned case.
        assert_eq!(ann[0].chunk_id, brute[0].chunk_id);

        reset_hnsw_threshold_for_test();
    }
}
