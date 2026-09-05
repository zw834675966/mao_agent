//! Pure retrieval metrics: Recall@k, MRR@k, NDCG@k (no I/O, no network).

mod gold;

pub use gold::{GoldQuery, GoldQueryFilter, GoldQuerySet};

use std::collections::HashSet;

/// Fraction of expected chunk IDs that appear in the top-`k` retrieved results.
///
/// Edge cases: `k == 0` or empty `expected` → `0.0`. If `k > retrieved.len()`, uses all retrieved.
pub fn recall_at_k(retrieved: &[String], expected: &[String], k: usize) -> f32 {
    if k == 0 || expected.is_empty() {
        return 0.0;
    }
    let expected_set: HashSet<&str> = expected.iter().map(String::as_str).collect();
    if expected_set.is_empty() {
        return 0.0;
    }
    let top = retrieved.iter().take(k);
    let hits = top.filter(|id| expected_set.contains(id.as_str())).count();
    hits as f32 / expected_set.len() as f32
}

/// Mean Reciprocal Rank of the first relevant hit within top-`k`.
///
/// Returns `1/rank` (1-indexed) of the first retrieved ID in `expected`, or `0.0` if none.
pub fn mrr_at_k(retrieved: &[String], expected: &[String], k: usize) -> f32 {
    if k == 0 || expected.is_empty() {
        return 0.0;
    }
    let expected_set: HashSet<&str> = expected.iter().map(String::as_str).collect();
    for (i, id) in retrieved.iter().take(k).enumerate() {
        if expected_set.contains(id.as_str()) {
            return 1.0 / (i + 1) as f32;
        }
    }
    0.0
}

/// Discounted Cumulative Gain at `k` with binary relevance (1 if in `expected`, else 0).
///
/// Uses `rel_i / log2(i + 1)` for 1-indexed rank `i`.
pub fn dcg_at_k(retrieved: &[String], expected: &HashSet<String>, k: usize) -> f32 {
    if k == 0 || expected.is_empty() {
        return 0.0;
    }
    let mut dcg = 0.0_f32;
    for (i, id) in retrieved.iter().take(k).enumerate() {
        if expected.contains(id) {
            let rank = (i + 1) as f32;
            dcg += 1.0 / (rank + 1.0).log2();
        }
    }
    dcg
}

/// Normalized DCG@k: `dcg_at_k / idcg_at_k` with binary relevance.
///
/// Ideal DCG assumes all relevant items ranked first (up to `min(k, |expected|)`).
pub fn ndcg_at_k(retrieved: &[String], expected: &[String], k: usize) -> f32 {
    if k == 0 || expected.is_empty() {
        return 0.0;
    }
    let expected_set: HashSet<String> = expected.iter().cloned().collect();
    let dcg = dcg_at_k(retrieved, &expected_set, k);

    // Ideal ranking: all unique expected IDs first
    let ideal: Vec<String> = expected_set.into_iter().collect();
    let ideal_set: HashSet<String> = ideal.iter().cloned().collect();
    let idcg = dcg_at_k(&ideal, &ideal_set, k);
    if idcg == 0.0 {
        return 0.0;
    }
    dcg / idcg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(xs: &[&str]) -> Vec<String> {
        xs.iter().map(|s| (*s).to_string()).collect()
    }

    /// Hand-computed: retrieved=[a,b,c], expected={b,c}
    /// recall@2=0.5, recall@3=1.0, mrr@3=0.5
    /// DCG@3 = 0/log2(2) + 1/log2(3) + 1/log2(4) = 1/log2(3) + 0.5
    /// IDCG@3 = 1/log2(2) + 1/log2(3) = 1 + 1/log2(3)
    /// NDCG@3 = DCG/IDCG
    #[test]
    fn test_hand_computed_abc_bc() {
        let retrieved = ids(&["a", "b", "c"]);
        let expected = ids(&["b", "c"]);

        assert!((recall_at_k(&retrieved, &expected, 2) - 0.5).abs() < 1e-5);
        assert!((recall_at_k(&retrieved, &expected, 3) - 1.0).abs() < 1e-5);
        assert!((mrr_at_k(&retrieved, &expected, 3) - 0.5).abs() < 1e-5);

        let exp_set: HashSet<String> = expected.iter().cloned().collect();
        let dcg = dcg_at_k(&retrieved, &exp_set, 3);
        let expected_dcg = 1.0 / 3.0_f32.log2() + 0.5;
        assert!(
            (dcg - expected_dcg).abs() < 1e-5,
            "dcg={dcg} expected={expected_dcg}"
        );

        let ndcg = ndcg_at_k(&retrieved, &expected, 3);
        let idcg = 1.0 + 1.0 / 3.0_f32.log2();
        let expected_ndcg = expected_dcg / idcg;
        assert!(
            (ndcg - expected_ndcg).abs() < 1e-5,
            "ndcg={ndcg} expected={expected_ndcg}"
        );
    }

    #[test]
    fn test_edge_k_zero_and_empty_expected() {
        let retrieved = ids(&["a", "b"]);
        let expected = ids(&["a"]);
        assert_eq!(recall_at_k(&retrieved, &expected, 0), 0.0);
        assert_eq!(mrr_at_k(&retrieved, &expected, 0), 0.0);
        assert_eq!(ndcg_at_k(&retrieved, &expected, 0), 0.0);

        let empty: Vec<String> = vec![];
        assert_eq!(recall_at_k(&retrieved, &empty, 5), 0.0);
        assert_eq!(mrr_at_k(&retrieved, &empty, 5), 0.0);
        assert_eq!(ndcg_at_k(&retrieved, &empty, 5), 0.0);
    }

    #[test]
    fn test_k_greater_than_len_truncates() {
        let retrieved = ids(&["b"]);
        let expected = ids(&["b", "c"]);
        // Only one retrieved; hit 1 of 2 → recall 0.5; MRR = 1.0
        assert!((recall_at_k(&retrieved, &expected, 10) - 0.5).abs() < 1e-5);
        assert!((mrr_at_k(&retrieved, &expected, 10) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_no_hit_returns_zero() {
        let retrieved = ids(&["x", "y", "z"]);
        let expected = ids(&["a", "b"]);
        assert_eq!(recall_at_k(&retrieved, &expected, 3), 0.0);
        assert_eq!(mrr_at_k(&retrieved, &expected, 3), 0.0);
        assert_eq!(ndcg_at_k(&retrieved, &expected, 3), 0.0);
    }

    #[test]
    fn test_perfect_ranking_ndcg_one() {
        let retrieved = ids(&["b", "c", "a"]);
        let expected = ids(&["b", "c"]);
        assert!((ndcg_at_k(&retrieved, &expected, 2) - 1.0).abs() < 1e-5);
        assert!((recall_at_k(&retrieved, &expected, 2) - 1.0).abs() < 1e-5);
        assert!((mrr_at_k(&retrieved, &expected, 2) - 1.0).abs() < 1e-5);
    }
}
