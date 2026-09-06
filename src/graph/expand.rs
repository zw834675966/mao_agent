use crate::graph::SourceRef;
use crate::graph::store::GraphExpandHit;
use crate::index::HybridSearchResult;
use crate::model::DocumentChunk;
use std::collections::{HashMap, HashSet};

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
