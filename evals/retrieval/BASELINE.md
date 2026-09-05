# Retrieval Baseline (Cycle 2 / P1)

Offline deterministic metrics on the sample corpus index. **No live Cohere / no rerank.**

## Environment

| Item | Value |
|------|-------|
| Timestamp (Asia/Shanghai) | 2026-09-05 16:09:34 +08:00 |
| Embedder | DeterministicEmbedder (`--offline`, 512-dim) |
| Index | `data/vector_store.bin` + `data/tantivy_index` (59 chunks / 15 docs after `ingest --offline`) |
| Queries | `evals/retrieval/queries.jsonl` (105 gold queries) |
| Flags | `--k 5 --no-rerank --offline` |
| Rerank | off (baseline; Cohere not called) |

Commit: branch `feat/retrieval-upgrade`, message `feat(eval): add eval-retrieval CLI and BASELINE` (see `git log -1`).

## Headline numbers (`k=5`)

| Mode | Recall@5 | MRR@5 | NDCG@5 |
|------|----------|-------|--------|
| **Hybrid** (vector + BM25 RRF) | **1.000** | **0.984** | **0.988** |
| Vector | 1.000 | 0.965 | 0.974 |
| BM25 | 0.781 | 0.781 | 0.781 |

### Notes

- Gold queries are auto-generated from chunk text with deterministic templates; lexical overlap is high, so offline DeterministicEmbedder Recall@5 saturates for vector/hybrid.
- BM25 leg in `eval-retrieval` uses the stem inside Chinese quotation marks when present — full template questions tokenize into too many AND terms and otherwise return empty under Tantivy QueryParser.
- --force-brute is wired: forces exact brute-force vector scan (disables HNSW ANN) for recall comparison.
- **HNSW ANN** activates at HNSW_THRESHOLD=5000 vectors; graph is skipped in serde snapshots and rebuilt on load.

## Synthetic HNSW recall guard (Cycle 3)

| Item | Value |
|------|-------|
| Corpus | 3000 synthetic 64-d L2-normalized unit vectors |
| Queries | 50 deterministic unit queries |
| Threshold override | 500 (via set_hnsw_threshold_for_test) |
| Metric | mean Recall@5 of ANN ids vs brute-force ground truth |
| Result | mean_recall@5 >= 0.99 (absolute delta < 0.01); see 	ests/hnsw_regression_test.rs |

Crate note: crates.io hnswlib-rs 0.10 pulls corenn-kernels (nightly eature(f16)). Dependency is hnsw-stable 0.10.1 aliased as hnswlib-rs so imports remain hnswlib_rs::*.
- Rerank-on baseline is intentionally omitted here (requires live Cohere); compare later with the same CLI without `--no-rerank` when a key is present.

### Reproduce

```bash
cargo run --no-default-features -- eval-retrieval --k 5 --mode hybrid --no-rerank --offline
cargo run --no-default-features -- eval-retrieval --k 5 --mode vector --no-rerank --offline --json
cargo run --no-default-features -- eval-retrieval --k 5 --mode bm25 --no-rerank --offline --json
```

## Hard-negative / paraphrase subset (Cycle 7)

Dedicated file: `evals/retrieval/queries_hard.jsonl` (**26** queries: paraphrase / cross_doc / hard_negative).

| Check | Result |
|-------|--------|
| Easy auto-gold `「」` stem rate (≥12 chars) | ~1.0 (weak quality signal; see above) |
| Hard gold `「」` stem rate (≥8 chars) | **0.0** (enforced by `tests/retrieval_hard_eval_test.rs`) |
| 8-gram containment Recall@5 on easy gold (titles stripped) | ≳ 0.90 (saturates) |
| 8-gram containment Recall@5 on hard gold (titles stripped) | **&lt; 0.35** gate + per-query LCS&lt;8 |

Offline DeterministicEmbedder Hybrid Recall@5 on the **easy** auto-gold remains ~1.0 and is **not** a meaningful ranking quality signal. Use the hard file for regression:

```bash
cargo run --no-default-features -- eval-retrieval --queries-file evals/retrieval/queries_hard.jsonl --k 5 --mode hybrid --no-rerank --offline
cargo test --no-default-features --test retrieval_hard_eval_test
```

Live Cohere rerank comparison is still out of scope for this offline baseline (default model remains `rerank-v3.5`).

## Live Cohere hard-set (Cycle 10)

**Cancelled/deferred** for the B-grade offline bar: live Cohere rerank numbers on `queries_hard.jsonl` need an API secret. Revisit when a CI secret is available.
