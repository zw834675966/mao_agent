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

## Cycle 2–6

- [ ] TODO-06..08: eval metrics + queries.jsonl + eval-retrieval
- [ ] TODO-09..10: HNSW + regression
- [ ] Cycle 4: SSE reranked + api_test
- [ ] Cycle 5: citation adversarial
- [ ] Cycle 6: cleanup → 70+ tests

## Checkpoint

- [x] Cycle 1 gates green
