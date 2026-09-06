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
- `eval-retrieval --force-brute` forces exact brute-force vector scan (disables HNSW ANN) for recall comparison; the flag is not on `search`.
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

## Cycle 12 addendum (2026-09-06): BM25 colloquial-query fix

- **Mechanism correction:** the old note above said long queries fail from "too many AND terms". Precise cause is tantivy `QueryParser` coercing a whitespace-free multi-term leaf into a **slop-0 `Phrase`** query (`generate_literals_for_str`, tantivy 0.22.1), forcing the whole sentence to appear consecutively. Default operator was never AND (`conjunction_by_default: false` → OR); the phrase coercion happens before operators apply.
- **Fix (query-side only):** `FullTextIndex::should_query_for_colloquial` (`src/index/fulltext.rs`) pre-tokenizes with the index-time Jieba analyzer and unions terms (`Should`); zero-token input falls back to `QueryParser`. `fuse` weights, rerank, and the eval `「」`-stem path are untouched.
- **Numbers (same env/flags as above, `--k 5 --no-rerank --offline`, 105 queries):** Hybrid Recall@5 **1.000** / MRR@5 **0.995** / NDCG@5 **0.996** (was 1.000 / 0.984 / 0.988); BM25 leg **1.000 / 0.995 / 0.996** (was 0.781 × 3 — the old leg silently returned empty on long stems whose token positions didn't align). Vector leg unchanged. Gain comes from recovering the ~22% of queries where the BM25 leg was empty, not from weakening the metric.
- CLI spot check: `search --offline --no-rerank "主要矛盾与阿姆达尔定律"` now shows BM25 scores (9.50/4.56/5.49), Top1《矛盾论》 dual RRF 0.01639; `search --mode bm25` on the same sentence returns candidates instead of 0.
