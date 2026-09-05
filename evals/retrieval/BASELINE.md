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
- `--force-brute` is accepted as a **P2 stub** (no behavioral change until HNSW).
- Rerank-on baseline is intentionally omitted here (requires live Cohere); compare later with the same CLI without `--no-rerank` when a key is present.

### Reproduce

```bash
cargo run --no-default-features -- eval-retrieval --k 5 --mode hybrid --no-rerank --offline
cargo run --no-default-features -- eval-retrieval --k 5 --mode vector --no-rerank --offline --json
cargo run --no-default-features -- eval-retrieval --k 5 --mode bm25 --no-rerank --offline --json
```
