use crate::graph::SourceRef;
use crate::graph::store::{GraphExpandHit, GraphStore};
use crate::index::HybridSearchResult;
use crate::model::DocumentChunk;
use crate::vector::store::VectorStore;
use std::collections::{HashMap, HashSet};
use std::future::Future;

/// Turn expander hits into unique chunks via a `source_ref` lookup (unresolved refs dropped).
pub fn resolve_graph_chunks(
    hits: &[GraphExpandHit],
    mut lookup: impl FnMut(&SourceRef) -> Vec<DocumentChunk>,
) -> Vec<ResolvedGraphChunk> {
    let mut by_id: HashMap<String, ResolvedGraphChunk> = HashMap::new();
    for hit in hits {
        for r in &hit.source_refs {
            for chunk in lookup(r) {
                let entry =
                    by_id
                        .entry(chunk.chunk_id.clone())
                        .or_insert_with(|| ResolvedGraphChunk {
                            chunk,
                            paths: Vec::new(),
                        });
                for p in &hit.paths {
                    if !entry.paths.contains(p) {
                        entry.paths.push(p.clone());
                    }
                }
            }
        }
    }
    by_id.into_values().collect()
}

/// Async variant of [`resolve_graph_chunks`]: prefetch unique `source_refs` via `lookup`, then dedupe.
///
/// `lookup` takes an owned [`SourceRef`] so callers can `async move` without borrowing the key.
pub async fn resolve_graph_chunks_async<F, Fut>(
    hits: &[GraphExpandHit],
    mut lookup: F,
) -> Vec<ResolvedGraphChunk>
where
    F: FnMut(SourceRef) -> Fut,
    Fut: Future<Output = Vec<DocumentChunk>>,
{
    let mut cache: HashMap<SourceRef, Vec<DocumentChunk>> = HashMap::new();
    for hit in hits {
        for r in &hit.source_refs {
            if let std::collections::hash_map::Entry::Vacant(e) = cache.entry(r.clone()) {
                e.insert(lookup(r.clone()).await);
            }
        }
    }
    resolve_graph_chunks(hits, |r| cache.get(r).cloned().unwrap_or_default())
}

/// Expand `query` on `graph`, resolve refs via async `lookup`, then [`union_graph_bonus`].
/// Shared by CLI / DialecticalAgent / MCP / HTTP search — one place for hop count + union policy.
pub async fn expand_and_union_hybrid<F, Fut>(
    graph: &GraphStore,
    fused: Vec<HybridSearchResult>,
    query: &str,
    hops: u8,
    final_top_k: Option<usize>,
    lookup: F,
) -> Vec<HybridSearchResult>
where
    F: FnMut(SourceRef) -> Fut,
    Fut: Future<Output = Vec<DocumentChunk>>,
{
    let hits = graph.expand(query, hops);
    let resolved = resolve_graph_chunks_async(&hits, lookup).await;
    union_graph_bonus(fused, &resolved, final_top_k)
}

/// Max graph-unique chunks appended to the pre-rerank pool.
pub const GRAPH_BONUS_CAP: usize = 8;
/// Tail slots reserved for graph-unique chunks when truncating to final top_k without rerank.
pub const GRAPH_RESERVED: usize = 2;

/// A corpus chunk already resolved from graph `source_refs`, with expander paths.
#[derive(Debug, Clone)]
pub struct ResolvedGraphChunk {
    pub chunk: DocumentChunk,
    pub paths: Vec<String>,
}

/// Union graph chunks into dual RRF results without changing dual `rrf_score`s.
///
/// Dual hits that also appear in `graph_chunks` keep their score and gain `graph_paths`.
/// Graph-unique chunks are appended with `rrf_score = 0.0` (cap [`GRAPH_BONUS_CAP`]).
///
/// When `final_top_k` is `Some(k)`, apply reserved tail slots:
/// `dual[..k-r] + bonus[..r]` where `r = min(GRAPH_RESERVED, bonus.len(), k)`.
/// When `None`, return the full dual list plus bonus (pre-rerank pool).
pub fn union_graph_bonus(
    dual: Vec<HybridSearchResult>,
    graph_chunks: &[ResolvedGraphChunk],
    final_top_k: Option<usize>,
) -> Vec<HybridSearchResult> {
    if graph_chunks.is_empty() {
        return dual;
    }

    let mut dual = dual;
    let dual_ids: HashSet<String> = dual.iter().map(|h| h.chunk_id.clone()).collect();

    for g in graph_chunks {
        let id = &g.chunk.chunk_id;
        if let Some(hit) = dual.iter_mut().find(|h| h.chunk_id == *id) {
            match &mut hit.graph_paths {
                Some(existing) => {
                    for p in &g.paths {
                        if !existing.contains(p) {
                            existing.push(p.clone());
                        }
                    }
                }
                None => hit.graph_paths = Some(g.paths.clone()),
            }
        }
    }

    let mut bonus: Vec<HybridSearchResult> = Vec::new();
    let mut bonus_ids: HashSet<String> = HashSet::new();
    for g in graph_chunks {
        let id = g.chunk.chunk_id.clone();
        if dual_ids.contains(&id) || bonus_ids.contains(&id) {
            continue;
        }
        if bonus.len() >= GRAPH_BONUS_CAP {
            break;
        }
        bonus_ids.insert(id.clone());
        bonus.push(HybridSearchResult {
            chunk_id: id,
            rrf_score: 0.0,
            bm25_score: None,
            vector_score: None,
            rerank_score: None,
            graph_paths: if g.paths.is_empty() {
                None
            } else {
                Some(g.paths.clone())
            },
            rank: 0,
            chunk: g.chunk.clone(),
        });
    }

    let mut merged = match final_top_k {
        None => {
            dual.extend(bonus);
            dual
        }
        Some(0) => Vec::new(),
        Some(k) => {
            let r = GRAPH_RESERVED.min(bonus.len()).min(k);
            let dual_keep = k.saturating_sub(r).min(dual.len());
            let mut out = dual.into_iter().take(dual_keep).collect::<Vec<_>>();
            out.extend(bonus.into_iter().take(r));
            out
        }
    };

    for (i, hit) in merged.iter_mut().enumerate() {
        hit.rank = i + 1;
    }
    merged
}

/// Unified asynchronous graph expansion for hybrid search results.
///
/// If `graph` is `None`, returns `fused` untouched.
/// Otherwise, expands query up to 2 hops, resolves matching chunks from `store`,
/// and merges them using `union_graph_bonus` with the specified `final_top_k`.
pub async fn expand_with_graph(
    graph: Option<&GraphStore>,
    store: &VectorStore,
    fused: Vec<HybridSearchResult>,
    query: &str,
    final_top_k: Option<usize>,
) -> Vec<HybridSearchResult> {
    let Some(graph) = graph else {
        return fused;
    };
    let hits = graph.expand(query, 2);
    let mut resolved = Vec::new();
    for hit in &hits {
        for r in &hit.source_refs {
            for chunk in store.chunks_matching_ref(r).await {
                resolved.push(ResolvedGraphChunk {
                    chunk,
                    paths: hit.paths.clone(),
                });
            }
        }
    }
    union_graph_bonus(fused, &resolved, final_top_k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::Domain;
    use crate::model::HistoricalPeriod;

    fn dual_hit(id: &str, rrf: f32, rank: usize) -> HybridSearchResult {
        HybridSearchResult {
            chunk_id: id.to_string(),
            rrf_score: rrf,
            bm25_score: None,
            vector_score: None,
            rerank_score: None,
            graph_paths: None,
            rank,
            chunk: DocumentChunk {
                chunk_id: id.to_string(),
                doc_id: format!("doc_{id}"),
                doc_title: id.to_string(),
                author: "test".into(),
                period: HistoricalPeriod::Unknown,
                date: "1937-08".into(),
                volume: String::new(),
                category: String::new(),
                domain: Domain::Any,
                tags: vec![],
                chunk_index: 0,
                total_chunks: 1,
                char_count: 0,
                raw_text: id.to_string(),
                contextualized_text: id.to_string(),
                section_path: vec![],
            },
        }
    }

    #[tokio::test]
    async fn expand_with_graph_none_returns_fused_unchanged() {
        let fused = vec![dual_hit("c1", 0.02, 1), dual_hit("c2", 0.01, 2)];
        let store = VectorStore::new_deterministic(2);
        let out = expand_with_graph(None, &store, fused.clone(), "矛盾论", Some(5)).await;
        assert_eq!(out, fused);
    }

    #[tokio::test]
    async fn expand_with_graph_some_injects_graph_chunks() {
        let graph_json = r#"{
            "entities": [
                {
                    "id": "e1",
                    "name": "主要矛盾",
                    "aliases": [],
                    "source_refs": [{"doc_title": "矛盾论"}]
                },
                {
                    "id": "e2",
                    "name": "阿姆达尔定律 (Amdahl's Law)",
                    "aliases": [],
                    "source_refs": [{"doc_title": "阿姆达尔定律 (Amdahl's Law)"}]
                }
            ],
            "relationships": [
                {
                    "id": "rel1",
                    "source": "e1",
                    "target": "e2",
                    "rel_type": "aligned_with"
                }
            ]
        }"#;
        let graph = GraphStore::from_json_str(graph_json).unwrap();
        let store = VectorStore::new_deterministic(2);
        let chunk1 = DocumentChunk {
            chunk_id: "c1".into(),
            doc_id: "doc1".into(),
            doc_title: "矛盾论".into(),
            author: "test".into(),
            period: HistoricalPeriod::Unknown,
            date: "1937".into(),
            volume: String::new(),
            category: String::new(),
            domain: Domain::Any,
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 0,
            raw_text: "矛盾论全文".into(),
            contextualized_text: "矛盾论全文".into(),
            section_path: vec![],
        };
        let chunk2 = DocumentChunk {
            chunk_id: "c2".into(),
            doc_id: "doc2".into(),
            doc_title: "阿姆达尔定律 (Amdahl's Law)".into(),
            author: "test".into(),
            period: HistoricalPeriod::Unknown,
            date: "1937".into(),
            volume: String::new(),
            category: String::new(),
            domain: Domain::Any,
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 0,
            raw_text: "阿姆达尔定律".into(),
            contextualized_text: "阿姆达尔定律".into(),
            section_path: vec![],
        };
        store.index_chunks(vec![chunk1, chunk2]).await.unwrap();
        let fused = vec![];
        let result = expand_with_graph(Some(&graph), &store, fused, "主要矛盾", Some(5)).await;
        assert!(!result.is_empty(), "graph expansion should inject chunks");
        // The seed "主要矛盾" resolves chunk1; the 1-hop target "次要矛盾" resolves chunk2 with paths.
        let c2 = result.iter().find(|h| h.chunk_id == "c2");
        assert!(
            c2.is_some(),
            "should inject second-hop chunk with graph_paths"
        );
        assert!(c2.unwrap().graph_paths.is_some());
    }
}
