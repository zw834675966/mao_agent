use mao_agent::graph::{ResolvedGraphChunk, union_graph_bonus};
use mao_agent::index::HybridSearchResult;
use mao_agent::model::{DocumentChunk, HistoricalPeriod};

fn chunk(id: &str, title: &str) -> DocumentChunk {
    DocumentChunk {
        chunk_id: id.to_string(),
        doc_id: format!("doc_{id}"),
        doc_title: title.to_string(),
        author: "test".to_string(),
        period: HistoricalPeriod::Unknown,
        date: "1937-08".to_string(),
        volume: String::new(),
        category: String::new(),
        tags: vec![],
        chunk_index: 0,
        total_chunks: 1,
        char_count: 8,
        raw_text: title.to_string(),
        contextualized_text: title.to_string(),
        section_path: vec![],
    }
}

fn dual_hit(id: &str, title: &str, rrf: f32, rank: usize) -> HybridSearchResult {
    HybridSearchResult {
        chunk_id: id.to_string(),
        rrf_score: rrf,
        bm25_score: Some(1.0),
        vector_score: Some(0.9),
        rerank_score: None,
        graph_paths: None,
        rank,
        chunk: chunk(id, title),
    }
}

fn graph_chunk(id: &str, title: &str, path: &str) -> ResolvedGraphChunk {
    ResolvedGraphChunk {
        chunk: chunk(id, title),
        paths: vec![path.to_string()],
    }
}

#[test]
fn empty_graph_returns_dual_unchanged() {
    let dual = vec![
        dual_hit("c1", "矛盾论", 0.02, 1),
        dual_hit("c2", "实践论", 0.01, 2),
    ];
    let out = union_graph_bonus(dual.clone(), &[], Some(5));
    assert_eq!(out, dual);
}

#[test]
fn overlapping_chunk_keeps_rrf_and_gains_paths() {
    let dual = vec![dual_hit("c1", "矛盾论", 0.016, 1)];
    let graph = vec![graph_chunk(
        "c1",
        "矛盾论",
        "主要矛盾 -aligned_with-> 阿姆达尔定律 (Amdahl's Law)",
    )];
    let out = union_graph_bonus(dual, &graph, None);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].rrf_score, 0.016);
    assert_eq!(
        out[0].graph_paths.as_deref(),
        Some(["主要矛盾 -aligned_with-> 阿姆达尔定律 (Amdahl's Law)".to_string()].as_slice())
    );
}

#[test]
fn graph_unique_has_zero_rrf_and_reserved_tail_slots() {
    let dual: Vec<HybridSearchResult> = (0..5)
        .map(|i| dual_hit(&format!("d{i}"), "dual", 0.02 - i as f32 * 0.001, i + 1))
        .collect();
    let graph = vec![
        graph_chunk("g1", "阿姆达尔定律 (Amdahl's Law)", "A -aligned_with-> B"),
        graph_chunk("g2", "布鲁克斯法则", "A -aligned_with-> C"),
    ];
    let out = union_graph_bonus(dual, &graph, Some(5));
    assert_eq!(out.len(), 5);
    assert_eq!(out[3].chunk_id, "g1");
    assert_eq!(out[4].chunk_id, "g2");
    assert_eq!(out[3].rrf_score, 0.0);
    assert_eq!(out[4].rrf_score, 0.0);
    assert!(out[0].rrf_score > out[3].rrf_score);
    assert!(out[3].graph_paths.is_some());
}

#[test]
fn pre_rerank_pool_appends_bonus_without_dropping_dual() {
    let dual = vec![
        dual_hit("d0", "矛盾论", 0.02, 1),
        dual_hit("d1", "实践论", 0.01, 2),
    ];
    let graph = vec![graph_chunk(
        "g1",
        "阿姆达尔定律 (Amdahl's Law)",
        "A -aligned_with-> B",
    )];
    let out = union_graph_bonus(dual, &graph, None);
    assert_eq!(out.len(), 3);
    assert_eq!(out[2].chunk_id, "g1");
    assert_eq!(out[2].rrf_score, 0.0);
}
