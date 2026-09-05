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

- [x] TODO-01: `src/rerank/` + `Reranker` trait
- [x] TODO-02: `CohereReranker` + mock tests
- [x] TODO-03: `HybridSearchResult.rerank_score` + `rerank_or_fallback`
- [x] TODO-04: CLI/Agent/Axum wiring (`--no-rerank`)
- [x] TODO-05: README/docs + CI gates green

### Cycle 2 — P1 Eval

- [x] TODO-06: `src/eval` Recall/MRR/NDCG
- [x] TODO-07: `evals/retrieval/queries.jsonl` ~100
- [x] TODO-08: `eval-retrieval` CLI + BASELINE.md

### Cycle 3 — P2 HNSW

- [x] TODO-09: `hnswlib-rs` threshold + fallback
- [x] TODO-10: regression test delta_recall < 1%

### Cycles 4–6

- [x] SSE `reranked` event + API tests
- [x] Citation adversarial suite
- [x] Simplify + README numbers → 70+ tests (~77)

## Checkpoints

- [x] After Cycle 1: `cargo fmt --check && cargo clippy --no-default-features --all-targets -- -D warnings && cargo test --no-default-features`
- [x] After Cycle 2: eval-retrieval smoke
- [x] After Cycle 3: hnsw_regression_test
- [x] Final: 70+ tests green (~77)

## Cycles 7–10 (B-grade + hard eval)

- [x] Cycle 7: hard-negative gold + offline hardness tests
- [x] Cycle 8: B-grade P0 (health fail-closed, shutdown, LLM fallback, CONTEXT/ADRs)
- [x] Cycle 9: B-grade P1 ops (request-id, retries, metrics, CORS allowlist)
- [x] Cycle 10: live/ready split, fallback metric, runbook, VectorFilter/HttpError seams, auth + ask concurrency, CI audit/release-build

### Cancelled (YAGNI / out of B)
- Credentials-mode CORS; public-internet TLS/rate-limit/audit; multi-node sync; live Cohere hard-set baseline (needs secret)
