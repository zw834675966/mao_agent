//! `eval-retrieval` CLI command implementation.
use super::EvalRetrievalArgs;
use super::runtime::{build_filter, get_embedder, load_store_interactive, make_reranker};
use crate::model::VectorFilter;
use crate::vector::store::VectorStore;
use std::path::Path;

#[derive(Debug, Clone, serde::Deserialize)]
struct EvalQueryFilter {
    period: Option<String>,
    volume: Option<String>,
    category: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct EvalGoldQuery {
    query: String,
    expected_chunk_ids: Vec<String>,
    #[serde(default)]
    filter: Option<EvalQueryFilter>,
}

fn load_eval_queries(path: &Path) -> Result<Vec<EvalGoldQuery>, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err(format!(
            "queries file not found: {} (expected evals/retrieval/queries.jsonl)",
            path.display()
        )
        .into());
    }
    let text = std::fs::read_to_string(path)?;
    let mut queries = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let q: EvalGoldQuery = serde_json::from_str(trimmed).map_err(|e| {
            format!(
                "invalid JSON on line {} of {}: {e}",
                line_no + 1,
                path.display()
            )
        })?;
        if q.expected_chunk_ids.is_empty() {
            return Err(
                format!("line {}: expected_chunk_ids must be non-empty", line_no + 1).into(),
            );
        }
        queries.push(q);
    }
    if queries.is_empty() {
        return Err("queries file contained no records".into());
    }
    Ok(queries)
}

fn eval_filter_from_query(f: &Option<EvalQueryFilter>) -> Option<VectorFilter> {
    let Some(f) = f else {
        return None;
    };
    build_filter(
        f.period.as_deref(),
        f.volume.as_deref(),
        f.category.as_deref(),
    )
}

/// Prefer the quoted stem inside 「…」 for BM25: full template questions tokenize into
/// too many AND terms and often return empty under Tantivy QueryParser.
/// Eval-only: gold questions often wrap a quoted stem in a long template.
/// Search/ask CLI keep the raw query (product UX). Do not silently change those paths.
fn bm25_query_text(query: &str) -> String {
    if let Some(start) = query.find('「')
        && let Some(end_rel) = query[start + '「'.len_utf8()..].find('」')
    {
        let s = start + '「'.len_utf8();
        let stem = &query[s..s + end_rel];
        if stem.chars().count() >= 4 {
            return stem.to_string();
        }
    }
    query.to_string()
}

async fn retrieve_chunk_ids_for_eval(
    args: &EvalRetrievalArgs,
    store: &VectorStore,
    ft_index: Option<&crate::index::FullTextIndex>,
    query: &str,
    filter: Option<&VectorFilter>,
    reranker: Option<&dyn crate::Reranker>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let k = args.k;
    let force_brute = args.force_brute;
    let bm25_q = bm25_query_text(query);

    match args.mode.as_str() {
        "bm25" => {
            let Some(ft) = ft_index else {
                return Err("BM25 mode requires tantivy_dir index".into());
            };
            let results = ft.search(&bm25_q, k, filter)?;
            Ok(results.into_iter().map(|r| r.chunk_id).collect())
        }
        "vector" => {
            let results = store
                .search_with_force_brute(query, k, filter, force_brute)
                .await?;
            Ok(results.into_iter().map(|r| r.chunk_id).collect())
        }
        _ => {
            // hybrid: fuse top_k*2 then optional rerank → top_k
            let vec_results = store
                .search_with_force_brute(query, k * 2, filter, force_brute)
                .await?;
            let bm25_results = if let Some(ft) = ft_index {
                match ft.search(&bm25_q, k * 2, filter) {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::warn!("BM25 search failed during eval: {e}");
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            };
            let coordinator = crate::index::HybridSearchCoordinator::default();
            let fused = coordinator.fuse(vec_results, bm25_results, k * 2);
            let hybrid = crate::rerank_or_fallback(fused, reranker, query, k).await;
            Ok(hybrid.into_iter().map(|r| r.chunk_id).collect())
        }
    }
}

pub async fn handle_eval_retrieval(
    args: &EvalRetrievalArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let queries = match load_eval_queries(&args.queries_file) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("❌ {e}");
            std::process::exit(1);
        }
    };

    if args.force_brute {
        tracing::info!("--force-brute: forcing exact brute-force vector scan (HNSW ANN disabled)");
    }

    // Fail fast on missing index before constructing an embedder (may otherwise
    // attempt remote/model init and obscure the real problem).
    if !args.index_file.exists() {
        eprintln!(
            "❌ 向量索引文件未找到: {}。请先运行 `cargo run --no-default-features -- init-samples` 与 `ingest --offline`。",
            args.index_file.display()
        );
        std::process::exit(1);
    }
    // Offline baseline: prefer --offline; if not set, still try load (identity mismatch handled)
    let embedder = get_embedder(&args.embedder, Some(&args.index_file))?;
    let store = match load_store_interactive(&args.index_file, embedder)? {
        Some(s) => s,
        None => std::process::exit(1),
    };

    let ft_index = if args.tantivy_dir.exists() {
        Some(crate::index::FullTextIndex::new_in_dir(&args.tantivy_dir)?)
    } else if args.mode == "bm25" || args.mode == "hybrid" {
        eprintln!(
            "❌ Tantivy 索引目录未找到: {}（mode={} 需要 BM25）",
            args.tantivy_dir.display(),
            args.mode
        );
        std::process::exit(1);
    } else {
        None
    };

    let reranker = make_reranker(
        args.embedder.offline,
        args.no_rerank,
        None,
        args.embedder.embed_api_key.as_deref(),
    );

    let mut sum_recall = 0.0_f32;
    let mut sum_mrr = 0.0_f32;
    let mut sum_ndcg = 0.0_f32;
    let n = queries.len() as f32;
    let k = args.k;

    for (idx, gq) in queries.iter().enumerate() {
        let filter = eval_filter_from_query(&gq.filter);
        let retrieved = retrieve_chunk_ids_for_eval(
            args,
            &store,
            ft_index.as_ref(),
            &gq.query,
            filter.as_ref(),
            reranker.as_deref(),
        )
        .await?;

        let recall = crate::eval::recall_at_k(&retrieved, &gq.expected_chunk_ids, k);
        let mrr = crate::eval::mrr_at_k(&retrieved, &gq.expected_chunk_ids, k);
        let ndcg = crate::eval::ndcg_at_k(&retrieved, &gq.expected_chunk_ids, k);
        sum_recall += recall;
        sum_mrr += mrr;
        sum_ndcg += ndcg;

        if args.json {
            let row = serde_json::json!({
                "type": "query",
                "index": idx,
                "query": gq.query,
                "expected_chunk_ids": gq.expected_chunk_ids,
                "retrieved_chunk_ids": retrieved,
                "recall_at_k": recall,
                "mrr_at_k": mrr,
                "ndcg_at_k": ndcg,
                "k": k,
                "mode": args.mode,
            });
            println!("{}", serde_json::to_string(&row)?);
        }
    }

    let mean_recall = sum_recall / n;
    let mean_mrr = sum_mrr / n;
    let mean_ndcg = sum_ndcg / n;

    if args.json {
        let summary = serde_json::json!({
            "type": "summary",
            "n_queries": queries.len(),
            "k": k,
            "mode": args.mode,
            "no_rerank": args.no_rerank || reranker.is_none(),
            "force_brute": args.force_brute,
            "offline": args.embedder.offline,
            "recall_at_k": mean_recall,
            "mrr_at_k": mean_mrr,
            "ndcg_at_k": mean_ndcg,
        });
        println!("{}", serde_json::to_string(&summary)?);
    } else {
        println!(
            "Eval retrieval | mode={} | k={} | queries={} | rerank={}",
            args.mode,
            k,
            queries.len(),
            if reranker.is_some() { "on" } else { "off" }
        );
        println!("| metric    | @{k}   | value   |");
        println!("|-----------|-------|---------|");
        println!("| Recall    | {k:<5} | {mean_recall:.3}   |");
        println!("| MRR       | {k:<5} | {mean_mrr:.3}   |");
        println!("| NDCG      | {k:<5} | {mean_ndcg:.3}   |");
    }

    Ok(())
}
