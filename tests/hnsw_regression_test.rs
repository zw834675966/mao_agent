//! HNSW ANN vs brute-force recall regression (Cycle 3 / TODO-10).
//!
//! Builds a synthetic 64-d unit-vector corpus above the ANN threshold and
//! asserts that HNSW top-5 recall relative to exact brute force stays >= 0.99
//! (|delta_recall@5| < 0.01).

use mao_agent::model::{DocumentChunk, HistoricalPeriod, VectorEntry};
use mao_agent::vector::index::{
    VectorIndex, reset_hnsw_threshold_for_test, set_hnsw_threshold_for_test,
};

const DIM: usize = 64;
const N_VECTORS: usize = 3000;
const N_QUERIES: usize = 50;
const K: usize = 5;
const THRESHOLD: usize = 500;

fn make_chunk(id: &str) -> DocumentChunk {
    DocumentChunk {
        chunk_id: id.to_string(),
        doc_id: format!("doc_{id}"),
        doc_title: "synthetic".to_string(),
        author: "test".to_string(),
        period: HistoricalPeriod::WarOfResistance,
        date: "1938".to_string(),
        volume: "第二卷".to_string(),
        category: "军事".to_string(),
        tags: vec!["synthetic".to_string()],
        chunk_index: 0,
        total_chunks: 1,
        char_count: 8,
        raw_text: "synthetic".to_string(),
        contextualized_text: format!("synthetic {id}"),
        section_path: vec![],
    }
}

/// Deterministic unit vector from an integer seed (L2-normalized).
fn unit_vec(seed: u64) -> Vec<f32> {
    let mut v = Vec::with_capacity(DIM);
    let mut s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
    for _ in 0..DIM {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bits = ((s >> 33) as u32) & 0xFFFF;
        let x = (bits as f32 / 65535.0) * 2.0 - 1.0;
        v.push(x);
    }
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-12);
    for x in &mut v {
        *x /= norm;
    }
    v
}

fn recall_at_k(ann_ids: &[String], brute_ids: &[String], k: usize) -> f32 {
    if brute_ids.is_empty() || k == 0 {
        return 0.0;
    }
    let truth: std::collections::HashSet<&str> =
        brute_ids.iter().take(k).map(|s| s.as_str()).collect();
    let hits = ann_ids
        .iter()
        .take(k)
        .filter(|id| truth.contains(id.as_str()))
        .count();
    hits as f32 / truth.len().min(k) as f32
}

#[test]
fn hnsw_ann_recall_vs_brute_within_delta() {
    reset_hnsw_threshold_for_test();
    set_hnsw_threshold_for_test(THRESHOLD);

    let mut index = VectorIndex::new(DIM);
    let mut entries = Vec::with_capacity(N_VECTORS);
    for i in 0..N_VECTORS {
        let id = format!("v{i}");
        entries.push(VectorEntry {
            id: id.clone(),
            vector: unit_vec(i as u64 + 7),
            chunk: make_chunk(&id),
        });
    }
    index
        .insert_batch(entries)
        .expect("insert synthetic vectors");
    assert!(
        index.has_hnsw(),
        "HNSW must activate at threshold={THRESHOLD} with {N_VECTORS} vectors"
    );
    assert_eq!(index.len(), N_VECTORS);

    let mut recalls = Vec::with_capacity(N_QUERIES);
    for q in 0..N_QUERIES {
        let query = unit_vec(10_000 + q as u64);
        let ann = index
            .search_with_force_brute(&query, K, None, false)
            .expect("ANN search");
        let brute = index
            .search_with_force_brute(&query, K, None, true)
            .expect("brute search");
        assert_eq!(ann.len(), K);
        assert_eq!(brute.len(), K);
        let ann_ids: Vec<String> = ann.iter().map(|r| r.chunk_id.clone()).collect();
        let brute_ids: Vec<String> = brute.iter().map(|r| r.chunk_id.clone()).collect();
        recalls.push(recall_at_k(&ann_ids, &brute_ids, K));
    }

    let mean_recall = recalls.iter().sum::<f32>() / recalls.len() as f32;
    let delta = (1.0 - mean_recall).abs();
    eprintln!(
        "HNSW regression: mean_recall@5={mean_recall:.6} |delta|={delta:.6} (n={N_VECTORS}, q={N_QUERIES}, dim={DIM})"
    );
    assert!(
        mean_recall >= 0.99,
        "ANN recall@5 vs brute too low: {mean_recall:.6} (need >= 0.99)"
    );
    assert!(
        delta < 0.01,
        "|delta_recall@5|={delta:.6} exceeds 0.01 guard"
    );

    reset_hnsw_threshold_for_test();
}
