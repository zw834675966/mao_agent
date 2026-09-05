//! Adversarial / hard-negative retrieval gold: must not be solvable by substring matching alone.

use mao_agent::corpus::{ChineseSemanticChunker, ChunkerConfig, CorpusScanner};
use mao_agent::eval::{GoldQuery, GoldQuerySet};
use std::path::Path;

fn load_chunk_corpus() -> Vec<(String, String)> {
    let corpus_dir = Path::new("corpus");
    assert!(
        corpus_dir.is_dir(),
        "corpus/ missing — run from repo root (got {:?})",
        std::env::current_dir().ok()
    );
    let docs = CorpusScanner::load_documents_from_dir(corpus_dir).expect("load corpus");
    let chunker = ChineseSemanticChunker::new(ChunkerConfig {
        max_chars: 600,
        min_chars: 100,
        overlap_chars: 50,
        inject_context_header: true,
    });
    let mut out = Vec::new();
    for doc in &docs {
        for chunk in chunker.chunk_document(doc) {
            out.push((chunk.chunk_id, chunk.raw_text));
        }
    }
    assert!(!out.is_empty(), "expected non-empty chunk corpus");
    out
}

#[test]
fn easy_auto_gold_is_lexically_saturated() {
    let easy = GoldQuerySet::from_path(Path::new("evals/retrieval/queries.jsonl"))
        .expect("load easy gold");
    assert!(
        easy.len() >= 90,
        "expected ~105 easy queries, got {}",
        easy.len()
    );
    let rate = easy.easy_quoted_stem_rate(12);
    assert!(
        rate >= 0.95,
        "auto-generated gold should be mostly quoted-stem templates, rate={rate}"
    );

    let corpus = load_chunk_corpus();
    // Quoted stems make 8-gram containment trivial on the easy set.
    let ngram_recall = easy.mean_ngram_containment_recall_at_k(&corpus, 5, 8, true);
    assert!(
        ngram_recall >= 0.90,
        "8-gram containment should saturate on easy gold, got {ngram_recall}"
    );
}

#[test]
fn hard_gold_resists_easy_substring_matching() {
    let hard = GoldQuerySet::from_path(Path::new("evals/retrieval/queries_hard.jsonl"))
        .expect("load hard gold");
    assert!(
        hard.len() >= 20,
        "need >=20 hard queries, got {}",
        hard.len()
    );

    let easy_rate = hard.easy_quoted_stem_rate(12);
    assert!(
        easy_rate == 0.0,
        "hard gold must not use long 「」 stems, rate={easy_rate}"
    );
    for q in &hard.queries {
        assert!(
            !GoldQuery::has_easy_quoted_stem(&q.query, 8),
            "hard query still has quoted stem: {}",
            q.query
        );
        assert!(
            q.kind
                .as_deref()
                .is_some_and(|k| { matches!(k, "paraphrase" | "hard_negative" | "cross_doc") }),
            "hard query missing kind tag: {}",
            q.query
        );
    }

    let corpus = load_chunk_corpus();
    let id_set: std::collections::HashSet<&str> =
        corpus.iter().map(|(id, _)| id.as_str()).collect();
    for q in &hard.queries {
        for id in &q.expected_chunk_ids {
            assert!(
                id_set.contains(id.as_str()),
                "unknown gold chunk_id {id} for query {}",
                q.query
            );
        }
        for id in &q.expected_chunk_ids {
            let text = corpus
                .iter()
                .find(|(cid, _)| cid == id)
                .map(|(_, t)| GoldQuery::strip_book_titles(t))
                .unwrap();
            let probe = GoldQuery::strip_book_titles(&q.query);
            let overlap = GoldQuery::longest_char_substring(&probe, &text);
            assert!(
                overlap < 8,
                "hard query shares {overlap}-char span with gold (need <8): {}",
                q.query
            );
        }
    }

    // 8-gram containment must not solve the hard set (easy set saturates above).
    let ngram_recall = hard.mean_ngram_containment_recall_at_k(&corpus, 5, 8, true);
    assert!(
        ngram_recall < 0.35,
        "8-gram containment must not solve hard gold (recall@5={ngram_recall})"
    );
}
