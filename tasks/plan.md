# Implementation Plan: Long-Horizon Retrieval Upgrade (6 Cycles)

## Overview

Execute P0 Cohere Rerank → P1 eval harness → P2 HNSW → server SSE → citation adversarial → debt cleanup, per `.omo/plans/retrieval-upgrade.md` and the long-horizon artifact. Each cycle: inspect → TDD → adversarial checks → fmt/clippy/test → simplify.

## Architecture Decisions (locked)

- Rerank: Cohere only, `POST https://api.cohere.com/v2/rerank`, default model `rerank-v3.5` (override via `--rerank-model` / `COHERE_RERANK_MODEL`). No network in unit tests (mock only).
- Pipeline: `fuse(top_k*2) → rerank → top_k`; no key / HTTP fail → warn + original order.
- Eval before HNSW regression gate; HNSW via `hnswlib-rs`, threshold 5000, rebuild-on-load, brute fallback.
- Snapshot format unchanged (`#[serde(skip)]` for HNSW).

## Task List

### Cycle 1 — P0 Rerank (Batch 1)

- [ ] TODO-01: `src/rerank/` + `Reranker` trait
- [ ] TODO-02: `CohereReranker` + mock tests
- [ ] TODO-03: `HybridSearchResult.rerank_score` + `rerank_or_fallback`
- [ ] TODO-04: CLI/Agent/Axum wiring (`--no-rerank`)
- [ ] TODO-05: README/docs + CI gates green

### Cycle 2 — P1 Eval

- [ ] TODO-06: `src/eval` Recall/MRR/NDCG
- [ ] TODO-07: `evals/retrieval/queries.jsonl` ~100
- [ ] TODO-08: `eval-retrieval` CLI + BASELINE.md

### Cycle 3 — P2 HNSW

- [ ] TODO-09: `hnswlib-rs` threshold + fallback
- [ ] TODO-10: regression test delta_recall < 1%

### Cycles 4–6

- [ ] SSE `reranked` event + API tests
- [ ] Citation adversarial suite
- [ ] Simplify + README numbers → 70+ tests

## Checkpoints

- [ ] After Cycle 1: `cargo fmt --check && cargo clippy --no-default-features --all-targets -- -D warnings && cargo test --no-default-features`
- [ ] After Cycle 2: eval-retrieval smoke
- [ ] After Cycle 3: hnsw_regression_test
- [ ] Final: 70+ tests green
