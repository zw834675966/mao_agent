use std::time::Instant;

use axum::{Json, extract::State, http::StatusCode};

use crate::model::VectorFilter;
use crate::rerank::rerank_or_fallback;
use crate::server::dto::{SearchHit, SearchRequest, SearchResponse};
use crate::server::error::{ApiError, ApiResult};
use crate::server::state::AppState;

fn build_filter(req: &SearchRequest) -> Option<VectorFilter> {
    let has_filter = req.period.is_some()
        || req.volume.is_some()
        || req.category.is_some()
        || req.tags.is_some()
        || req.start_date.is_some()
        || req.end_date.is_some()
        || req.doc_id.is_some()
        || req.keyword.is_some();
    if !has_filter {
        return None;
    }
    let mut f = VectorFilter::new();
    if let Some(ref p) = req.period {
        f.period = Some(crate::model::HistoricalPeriod::from_str_or_date(p));
    }
    if let Some(ref v) = req.volume {
        f.volume = Some(v.clone());
    }
    if let Some(ref c) = req.category {
        f.category = Some(c.clone());
    }
    if let Some(ref tags) = req.tags {
        f.tags = Some(tags.clone());
    }
    if let Some(ref s) = req.start_date {
        f.start_date = Some(s.clone());
    }
    if let Some(ref e) = req.end_date {
        f.end_date = Some(e.clone());
    }
    if let Some(ref d) = req.doc_id {
        f.doc_id = Some(d.clone());
    }
    if let Some(ref k) = req.keyword {
        f.keyword = Some(k.clone());
    }
    Some(f)
}

pub async fn handle_search(
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> ApiResult<(StatusCode, Json<SearchResponse>)> {
    let started = Instant::now();
    let result = handle_search_inner(state.clone(), req).await;
    state.metrics.record_search(started, result.is_err());
    result
}

async fn handle_search_inner(
    state: AppState,
    req: SearchRequest,
) -> ApiResult<(StatusCode, Json<SearchResponse>)> {
    if req.query.trim().is_empty() {
        return Err(ApiError::bad_request("query must not be empty"));
    }
    let top_k = req.top_k.clamp(1, 20);
    let mode = req.mode.to_lowercase();
    if !matches!(mode.as_str(), "hybrid" | "vector" | "bm25") {
        return Err(ApiError::bad_request(
            "mode must be one of: hybrid, vector, bm25",
        ));
    }
    let filter = build_filter(&req);
    let start = Instant::now();

    let (hits, mode_used) = match mode.as_str() {
        "vector" => {
            let results = state
                .store
                .search(&req.query, top_k, filter.as_ref())
                .await
                .map_err(ApiError::from)?;
            let hits: Vec<SearchHit> = results
                .into_iter()
                .filter(|r| req.min_score.is_none_or(|m| r.score >= m))
                .map(|r| SearchHit {
                    chunk_id: r.chunk_id,
                    rank: r.rank,
                    rrf_score: None,
                    vector_score: Some(r.score),
                    bm25_score: None,
                    rerank_score: None,
                    graph_paths: None,
                    chunk: r.chunk,
                })
                .collect();
            (hits, "vector")
        }
        "bm25" => {
            let tantivy = state.tantivy.as_ref().ok_or_else(|| {
                ApiError::service_unavailable(
                    "Tantivy index not loaded: run `mao_agent ingest` first",
                )
            })?;
            let results = tantivy
                .search(&req.query, top_k, filter.as_ref())
                .map_err(ApiError::from)?;
            let hits: Vec<SearchHit> = results
                .into_iter()
                .map(|r| SearchHit {
                    chunk_id: r.chunk_id,
                    rank: r.rank,
                    rrf_score: None,
                    vector_score: None,
                    bm25_score: Some(r.score),
                    rerank_score: None,
                    graph_paths: None,
                    chunk: r.chunk,
                })
                .collect();
            (hits, "bm25")
        }
        _ => {
            // hybrid: RRF fuse (top_k*2) then optional rerank → top_k
            let vec_results = state
                .store
                .search(&req.query, top_k * 2, filter.as_ref())
                .await
                .map_err(ApiError::from)?;
            let bm25_results = if let Some(ref ft) = state.tantivy {
                match ft.search(&req.query, top_k * 2, filter.as_ref()) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!("BM25 search failed: {e}, continuing vector-only");
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            };
            let fused = state.hybrid.fuse(vec_results, bm25_results, top_k * 2);
            let fused = if let Some(graph) = state.graph.as_ref() {
                let hits = graph.expand(&req.query, 2);
                let mut resolved = Vec::new();
                for hit in &hits {
                    for r in &hit.source_refs {
                        for chunk in state.store.chunks_matching_ref(r).await {
                            resolved.push(crate::graph::ResolvedGraphChunk {
                                chunk,
                                paths: hit.paths.clone(),
                            });
                        }
                    }
                }
                let skip = req.no_rerank.unwrap_or(false);
                crate::graph::union_graph_bonus(
                    fused,
                    &resolved,
                    if skip || state.reranker.is_none() {
                        Some(top_k)
                    } else {
                        None
                    },
                )
            } else {
                fused
            };
            let skip_rerank = req.no_rerank.unwrap_or(false);
            let reranker = if skip_rerank {
                None
            } else {
                state.reranker.as_deref()
            };
            let fused = rerank_or_fallback(fused, reranker, &req.query, top_k).await;
            let mut hits: Vec<SearchHit> = fused
                .into_iter()
                .map(|r| SearchHit {
                    chunk_id: r.chunk_id,
                    rank: r.rank,
                    rrf_score: Some(r.rrf_score),
                    vector_score: r.vector_score,
                    bm25_score: r.bm25_score,
                    rerank_score: r.rerank_score,
                    graph_paths: r.graph_paths,
                    chunk: r.chunk,
                })
                .collect();
            if let Some(min) = req.min_score {
                hits.retain(|h| h.vector_score.is_none_or(|s| s >= min));
            }
            (hits, "hybrid")
        }
    };

    let elapsed_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
    let total = hits.len();
    let resp = SearchResponse {
        query: req.query,
        mode: mode_used.to_string(),
        elapsed_ms,
        total,
        results: hits,
    };
    Ok((StatusCode::OK, Json(resp)))
}
