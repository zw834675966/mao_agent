//! C4: Hybrid retrieval domain filtering integration test.
//!
//! Builds an in-memory corpus whose chunks carry explicit `Domain::History` /
//! `Domain::Engineering` tags (via frontmatter `category`), then verifies that
//! a `--domain`-style filter propagates through both the vector and BM25 legs
//! and the fused hybrid results:
//! - `history` → only History chunks
//! - `engineering` → only Engineering chunks
//! - `any` / no filter → all chunks

use std::sync::Arc;

use mao_agent::corpus::{ChineseSemanticChunker, Domain};
use mao_agent::index::{FullTextIndex, HybridSearchCoordinator};
use mao_agent::model::{Document, DocumentMetadata, HistoricalPeriod, VectorFilter};
use mao_agent::vector::VectorStore;

fn history_doc() -> Document {
    Document {
        id: "doc_history".to_string(),
        metadata: DocumentMetadata {
            title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            date: "1938-05-26".to_string(),
            period: "抗日战争时期".to_string(),
            volume: "毛泽东选集第二卷".to_string(),
            category: "history".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::WarOfResistance,
        headnote: None,
        content: "中日战争是持久战，战略防御、战略相持、战略反攻三个阶段。兵民是胜利之本。"
            .to_string(),
        footnotes: vec![],
        file_path: None,
    }
}

fn engineering_doc() -> Document {
    Document {
        id: "doc_engineering".to_string(),
        metadata: DocumentMetadata {
            title: "阿姆达尔定律".to_string(),
            author: "未知".to_string(),
            date: "2024-01-01".to_string(),
            period: String::new(),
            volume: String::new(),
            category: "engineering".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::Unknown,
        headnote: None,
        content:
            "串行比例决定加速比上限，优化应抓住主要瓶颈。阿姆达尔定律揭示了可扩展性的本质约束。"
                .to_string(),
        footnotes: vec![],
        file_path: None,
    }
}

async fn build_corpus() -> (Arc<VectorStore>, Arc<FullTextIndex>) {
    let docs = vec![history_doc(), engineering_doc()];
    let store = Arc::new(VectorStore::new_deterministic(64));
    for d in &docs {
        store
            .index_document(d)
            .await
            .expect("index history/engineering doc");
    }

    let chunker = ChineseSemanticChunker::new(Default::default());
    let ft = FullTextIndex::new_in_ram().expect("in-ram fulltext");
    for d in &docs {
        ft.insert_batch(&chunker.chunk_document(d))
            .expect("insert chunks");
    }

    (store, Arc::new(ft))
}

async fn hybrid_ids(
    store: &VectorStore,
    ft: &FullTextIndex,
    query: &str,
    filter: Option<&VectorFilter>,
) -> Vec<String> {
    let vec_results = store
        .search(query, 10, filter)
        .await
        .expect("vector search");
    let bm25_results = ft.search(query, 10, filter).expect("bm25 search");
    let coordinator = HybridSearchCoordinator::default();
    coordinator
        .fuse(vec_results, bm25_results, 10)
        .into_iter()
        .map(|r| r.chunk_id)
        .collect()
}

#[tokio::test]
async fn test_history_domain_filter_returns_only_history_chunks() {
    let (store, ft) = build_corpus().await;
    let filter = VectorFilter::new().with_domain(Domain::History);

    let ids = hybrid_ids(&store, &ft, "战争 战略 持久", Some(&filter)).await;
    assert!(
        !ids.is_empty(),
        "history filter must recall the history doc"
    );
    for id in &ids {
        assert!(
            id.starts_with("doc_history"),
            "history filter returned non-history chunk: {id}"
        );
    }
}

#[tokio::test]
async fn test_engineering_domain_filter_returns_only_engineering_chunks() {
    let (store, ft) = build_corpus().await;
    let filter = VectorFilter::new().with_domain(Domain::Engineering);

    let ids = hybrid_ids(&store, &ft, "阿姆达尔 瓶颈 加速", Some(&filter)).await;
    assert!(
        !ids.is_empty(),
        "engineering filter must recall the engineering doc"
    );
    for id in &ids {
        assert!(
            id.starts_with("doc_engineering"),
            "engineering filter returned non-engineering chunk: {id}"
        );
    }
}

#[tokio::test]
async fn test_any_domain_filter_returns_all_chunks() {
    let (store, ft) = build_corpus().await;
    let filter = VectorFilter::new().with_domain(Domain::Any);

    let ids = hybrid_ids(&store, &ft, "战争 战略 阿姆达尔 瓶颈", Some(&filter)).await;
    let has_history = ids.iter().any(|id| id.starts_with("doc_history"));
    let has_engineering = ids.iter().any(|id| id.starts_with("doc_engineering"));
    assert!(
        has_history && has_engineering,
        "any filter must return both domains, got: {ids:?}"
    );
}

#[tokio::test]
async fn test_no_filter_returns_all_chunks() {
    let (store, ft) = build_corpus().await;

    let ids = hybrid_ids(&store, &ft, "战争 战略 阿姆达尔 瓶颈", None).await;
    let has_history = ids.iter().any(|id| id.starts_with("doc_history"));
    let has_engineering = ids.iter().any(|id| id.starts_with("doc_engineering"));
    assert!(
        has_history && has_engineering,
        "no filter must return both domains, got: {ids:?}"
    );
}

#[tokio::test]
async fn test_mismatched_domain_filter_returns_nothing() {
    let (store, ft) = build_corpus().await;
    // Engineering filter over a history-only query must not leak the history chunk.
    let filter = VectorFilter::new().with_domain(Domain::Engineering);
    let ids = hybrid_ids(&store, &ft, "持久战 战略相持", Some(&filter)).await;
    assert!(
        ids.iter().all(|id| id.starts_with("doc_engineering")),
        "engineering filter leaked a non-engineering chunk: {ids:?}"
    );
}
