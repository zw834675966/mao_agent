# Tasks: Long-Horizon Retrieval Upgrade

## Cycle 1 — P0 Cohere Rerank

- [x] TODO-01: Reranker trait + module scaffold
  - Files: `src/rerank/mod.rs`, `src/rerank/cohere.rs` (stub), `src/lib.rs`, `src/error.rs`
  - Verify: `cargo check --no-default-features`
- [x] TODO-02: CohereReranker POST /v2/rerank + mock reorder test `[1,0,2]`
  - Files: `src/rerank/cohere.rs`
  - Verify: `cargo test --no-default-features rerank`
- [x] TODO-03: `rerank_score` + `rerank_or_fallback`
  - Files: `src/index/hybrid.rs`, `src/rerank/mod.rs`
  - Verify: existing `test_rrf_fusion` + new fallback tests
- [x] TODO-04: Wire CLI search/ask + DialecticalAgent + Axum `/api/v1/search`
  - Flags: `--no-rerank`, `--rerank-model`; offline ⇒ no rerank
  - Verify: `--help` shows flags; tests green without network
- [x] TODO-05: README + gates
  - Verify: fmt/clippy/test

## Cycle 2 — P1 Retrieval Eval

- [x] TODO-06: eval metrics (`src/eval/mod.rs` Recall/MRR/NDCG@k)
  - Commit: `feat(eval): add recall/mrr/ndcg@k metrics`
- [x] TODO-07: `evals/retrieval/queries.jsonl` (~105) + README
  - Commit: `feat(eval): add retrieval queries.jsonl (~100 auto-generated from corpus)`
- [x] TODO-08: `eval-retrieval` CLI + `BASELINE.md`
  - Commit: `feat(eval): add eval-retrieval CLI and BASELINE`

## Cycle 3–6

- [x] TODO-09..10: HNSW + regression
- [ ] Cycle 4: SSE reranked + api_test
- [ ] Cycle 5: citation adversarial
- [ ] Cycle 6: cleanup → 70+ tests

## Checkpoint

- [x] Cycle 1 gates green
- [x] Cycle 2 gates green (fmt/clippy/test; eval-retrieval smoke offline)
