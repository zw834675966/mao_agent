# Tasks: Long-Horizon Retrieval Upgrade  ## Cycle 1 — P0 Cohere Rerank  - [x] TODO-01: Reranker trait + module scaffold   - Files: `src/rerank/mod.rs`, `src/rerank/cohere.rs` (stub), `src/lib.rs`, `src/error.rs`   - Verify: `cargo check --no-default-features` - [x] TODO-02: CohereReranker POST /v2/rerank + mock reorder test `[1,0,2]`   - Files: `src/rerank/cohere.rs`   - Verify: `cargo test --no-default-features rerank` - [x] TODO-03: `rerank_score` + `rerank_or_fallback`   - Files: `src/index/hybrid.rs`, `src/rerank/mod.rs`   - Verify: existing `test_rrf_fusion` + new fallback tests - [x] TODO-04: Wire CLI search/ask + DialecticalAgent + Axum `/api/v1/search`   - Flags: `--no-rerank`, `--rerank-model`; offline ⇒ no rerank   - Verify: `--help` shows flags; tests green without network - [x] TODO-05: README + gates   - Verify: fmt/clippy/test  ## Cycle 2 — P1 Retrieval Eval  - [x] TODO-06: eval metrics (`src/eval/mod.rs` Recall/MRR/NDCG@k)   - Commit: `feat(eval): add recall/mrr/ndcg@k metrics` - [x] TODO-07: `evals/retrieval/queries.jsonl` (~105) + README   - Commit: `feat(eval): add retrieval queries.jsonl (~100 auto-generated from corpus)` - [x] TODO-08: `eval-retrieval` CLI + `BASELINE.md`   - Commit: `feat(eval): add eval-retrieval CLI and BASELINE`  ## Cycle 3–6  - [x] TODO-09..10: HNSW + regression - [x] Cycle 4: SSE reranked + api_test   - `SseRerankedEvent { applied, chunk_ids, scores? }` always after `retrieved`   - `test_ask_stream_emits_event_sequence` expects retrieved → reranked → delta → citation → done   - MockReranker / `no_rerank` / None-reranker search tests + concurrency smoke - [x] Cycle 5: citation adversarial + dialectical four-stage structure   - Hardened `CitationVerifier`: equal-length JW window + Levenshtein edit budget (0 for short quotes <40 chars)   - `test_adversarial_citation_rejection_suite`: exact conf==1.0; synonym/reorder/fabricated/cross-doc/noise -> 100% reject   - `test_offline_dialectical_four_stage_structure`: four stage headings in order   - Gates: fmt / clippy -D warnings / cargo test --no-default-features - [x] Cycle 6: cleanup → 70+ tests (~77); docs sync; SSE error path no unwrap; plan/todo complete  ## Checkpoint  - [x] Cycle 1 gates green - [x] Cycle 2 gates green (fmt/clippy/test; eval-retrieval smoke offline) - [x] Cycle 5 gates green (adversarial citation 100% reject; four-stage structure; fmt/clippy/test) - [x] Cycle 6 gates green (fmt/clippy/test; docs synced; ~77 tests)  ## Cycle 7 — Hard-negative retrieval gold  - [x] Audit easy `queries.jsonl`: 100% long `「」` stems → offline Recall@5≈1.0 is weak - [x] Add `evals/retrieval/queries_hard.jsonl` (≥20 paraphrase / cross_doc / hard_negative) - [x] `GoldQuery` / `GoldQuerySet` lexical hardness helpers + title-stripped substring probe - [x] `tests/retrieval_hard_eval_test.rs` (easy saturates; hard resists substring matching) - [x] Update `evals/retrieval/BASELINE.md` + README; keep Cycles 1–6 checked - [c] Optional follow-up: live Cohere rerank numbers on hard set — cancelled/deferred (needs API secret)  ## Cycle 8 — B-grade P0 (runtime + docs)  Target: **B small-team intranet** — deliver what it takes to become B. Docs first + P0 code. Do not merge until review.  ### Done in this cycle - [x] `CONTEXT.md` — system boundaries, B target, data-flow sketch, non-goals - [x] ADRs: `docs/adr/0001-llm-fallback.md`, `0002-health-check-semantics.md`, `0003-bind-and-cors.md` - [x] `/health` → **503** when vector index empty (`index_loaded == false`) - [x] Graceful shutdown (`GracefulShutdown::wait` + `with_graceful_shutdown`) - [x] `LlmClient` + Online/Offline/`FallbackLlmClient` — API error with key → offline template - [x] Date filter: `chunk.date == "未知"` / empty must **not** pass interval filters - [x] Tests: health 503, LLM fallback on 500, unknown-date filter - [x] Gates: `fmt` / `clippy -D warnings` / `test --no-default-features`  ### Gap list (keep tracking)  **P0 (this PR)** - [x] Health fail-closed - [x] Graceful shutdown - [x] LLM online→offline fallback - [x] Unknown-date filter fail-closed - [x] CONTEXT + first ADRs  **P1 / P2 / P3** — see Cycle 9 gap list (CORS allowlist moved to Cycle 9 done).

## Cycle 9 — B-grade P1 (ops)

Target: **B small-team intranet** ops hardening on top of P0. Do not merge until review.

### Done in this cycle
- [x] Request ID middleware (`X-Request-Id` generate/propagate) + structured tracing on Axum path
- [x] Retry + exponential backoff for Cohere LLM chat + Cohere rerank (bounded; offline fallback after retries)
- [x] Minimal metrics: `GET /metrics` (Prometheus text) + `GET /api/v1/metrics` (JSON) for `/search` and `/ask`
- [x] CORS allowlist (CLI/env/config; localhost defaults) — ADR 0004 (extends 0003)
- [x] Tests: request-id, metrics increments, retry then success/fallback, CORS reject
- [x] Gates: `fmt` / `clippy -D warnings` / `test --no-default-features`

### Gap list (updated)

**P1 (remaining)** → completed in Cycle 10
- [x] Structured readiness vs liveness (`/live` + `/health` readiness)
- [x] LLM fallback metric counter
- [x] Operator runbook

**P2** → completed / cancelled in Cycle 10
- [x] Request auth (shared bearer secret) — ADR 0005
- [x] SSE/ask max concurrent limits → 429
- [c] Credentials-mode CORS — cancelled (YAGNI; no cookie SPA)

**P3** → cancelled (out of B scope)
- [c] Public-internet posture (TLS, rate limit, audit logs)
- [c] Multi-node index sync / warm standby

## Cycle 10 — B-grade closeout (P1 remainder + P2 intranet + CI)

### A. P1 remaining
- [x] Split health: `GET /live` always 200; `/health` stays readiness (503 empty) — ADR 0002 updated
- [x] Metric counter for LLM fallback (`mao_llm_fallback_total` Prometheus + JSON)
- [x] `docs/ops/runbook.md`: ingest → health green → serve; CORS/auth env cited

### B. Structure
- [x] VectorFilter: prefer `periods` when both set; BM25 honors `periods`; contradiction tests
- [x] `VectorError::HttpError` opaque `String` (no public `reqwest::Error`)
- [x] Graceful-shutdown unit test (oneshot cancel / serve abort)

### C. P2 B-intranet
- [x] Optional bearer auth (`--api-token` / `MAO_API_TOKEN` / config) — ADR 0005
- [x] SSE/ask concurrency limit (default 32) → 429 — ADR 0005

### D. CI / deferred
- [x] CI: `cargo audit` + `cargo build --release --no-default-features`
- [x] Optional live Cohere hard-set baseline — **cancelled/deferred** (needs API secret; YAGNI for offline B bar)

### Cancelled (YAGNI / out of B scope)
- [c] Credentials-mode CORS (no cookie SPA) — YAGNI
- [c] P3 public-internet TLS / rate-limit / audit logs — out of B scope
- [c] Multi-node index sync / warm standby — out of B scope
