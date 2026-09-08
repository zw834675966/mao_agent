# Implementation Plan: C9 — Defensive Retry + HTTP Error-Body Diagnostics (P2-13)

Date: 2026-09-08 · Slice: **C9 only** · Baseline: `cargo test --no-default-features` 187 passed / 0 failed at `f34fb93`.

## Overview

Eliminate two failure modes on the critical outbound-HTTP path without changing any external contract:

1. **Panic on `max_attempts == 0`.** `RetryPolicy::run` ends with `Err(last_err.expect("max_attempts >= 1"))` (`src/retry.rs:131`). Make `max_attempts == 0` panic-free.
2. **Silent empty HTTP error bodies.** Four call sites read a non-success body with `resp.text().await.unwrap_or_default()` (`src/rerank/cohere.rs:144`, `src/agent/llm.rs:147`, `src/vector/embedder/gemini.rs:158`, `src/vector/embedder/mod.rs:282`). Keep a body-read-failure diagnostic.

No CLI flags, REST DTOs, MCP tool schemas, or retry/status policy changes. No new crates. No `#[ignore]`.

## Architecture Decisions

- **Shipped seam:** a small public helper `crate::retry::diagnostic_http_error_body` used by all four call sites. Signature is generic over the error type so tests drive it without constructing a `reqwest::Error`:
  ```rust
  pub fn diagnostic_http_error_body(body: Result<String, impl std::fmt::Display>) -> String {
      body.unwrap_or_else(|e| format!("<failed to read error response body: {e}>"))
  }
  ```
  Message must contain `<failed to read error response body:`.
- **Defensive retry fallback:** keep `RetryPolicy::run` signature and retry/status policy unchanged. Replace the terminal `expect` with `match last_err { Some(e) => Err(e), None => op(0).await }`. Note: `run()` already computes `self.max_attempts.max(1)` (line 105), so `last_err` is currently always `Some`; the `None` arm is defensive-only but makes `max_attempts == 0` provably panic-free.
- **TDD order:** first write failing `#[cfg(test)]` tests in `src/retry.rs`, then implement the helper + fallback, then update the four call sites once the helper is green.
- **No new crates / no network:** tests drive the helper with `Err("connection reset")` and drive `RetryPolicy::run` with closures. `tempfile` / `DeterministicEmbedder` not needed (not used by `retry.rs` today).
- **Artifacts:** all C9 files live under `tasks/c9-defensive-retry/`. Cycle 13 `tasks/plan.md` / `tasks/todo.md` are **not** touched.

## Task List

### Phase 1: TDD — failing tests first (src/retry.rs)

- [x] Task 1: Add failing tests for `max_attempts == 0` and `diagnostic_http_error_body`
- [x] Task 2: Implement `diagnostic_http_error_body` helper + defensive `run` fallback (make tests green)

### Checkpoint: Phase 1
- [ ] `cargo test --no-default-features retry` green (new + existing retry tests)
- [ ] `cargo fmt --check` clean

### Phase 2: Wire the helper into the four call sites

- [x] Task 3: Replace `unwrap_or_default()` at `src/rerank/cohere.rs:144` and `src/agent/llm.rs:147`
- [x] Task 4: Replace `unwrap_or_default()` at `src/vector/embedder/gemini.rs:158` and `src/vector/embedder/mod.rs:282`

### Checkpoint: Phase 2
- [ ] grep shows no remaining `resp.text().await.unwrap_or_default()`
- [ ] `cargo test --no-default-features` all green (≥187 baseline + new tests, 0 failed, 0 ignored)

### Phase 3: Full gate

- [ ] Task 5: Full CI gate — fmt, clippy, full test suite

### Checkpoint: Complete
- [ ] `cargo fmt --check` clean
- [ ] `cargo clippy --no-default-features --all-targets -- -D warnings` zero warnings
- [ ] `cargo test --no-default-features` all green
- [ ] No CLI/REST/MCP/schema/policy change, no new crate, no `#[ignore]`
- [ ] Single-card atomic commit, fixed message format, no `git commit --amend`

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| `max_attempts == 0` defensive `op(0)` changes observable behavior | Med | `run()` already does `.max(1)`, so `None` arm is unreachable today; `op(0)` matches the existing single-attempt semantics. Covered by test. |
| Generic helper signature diverges from spec's `reqwest::Error` version | Low | Parent clarified generic `impl std::fmt::Display`; message contract (`<failed to read error response body:`) unchanged. |
| Call-site edit breaks success-path deserialization | Med | Only the error-body line changes; success-path `resp.json()` untouched. Verified by full suite. |
| Overwriting Cycle 13 `tasks/plan.md` / `tasks/todo.md` | High | All artifacts written under `tasks/c9-defensive-retry/` only. |

## Open Questions

None blocking. Parent decided slice scope, seam name, `max_attempts == 0` behavior, test location, commands, and helper signature. Human spec review waived (OBJECTIVE).

## Constraints Checklist

| ID | Constraint | must |
|----|-----------|------|
| C1 | `RetryPolicy::run` with `max_attempts: 0` returns without panicking; defensive fallback executes `op(0)` exactly once and returns its result | true |
| C2 | `crate::retry::diagnostic_http_error_body` is public, returns body unchanged on `Ok`, and a `"<failed to read error response body: {e}>"` diagnostic on `Err` | true |
| C3 | Helper signature generic: `pub fn diagnostic_http_error_body(body: Result<String, impl std::fmt::Display>) -> String` (tests pass `Err("connection reset")` without `reqwest::Error`) | true |
| C4 | All four call sites (`cohere.rs`, `llm.rs`, `gemini.rs`, `mod.rs`) call `diagnostic_http_error_body(resp.text().await)` instead of `resp.text().await.unwrap_or_default()` | true |
| C5 | `cargo fmt --check` clean | true |
| C6 | `cargo clippy --no-default-features --all-targets -- -D warnings` zero warnings | true |
| C7 | `cargo test --no-default-features` all green (≥187 baseline + new tests, 0 failed, 0 ignored) | true |
| C8 | No CLI flag, REST DTO, MCP tool schema, or retry/status policy change | true |
| C9 | No new crate (dependency) added | true |
| C10 | No `#[ignore]` introduced; existing assertions not weakened | true |
| C11 | `RetryPolicy::run` signature and retry/status policy (`should_retry_status`, backoff, jitter) unchanged | true |
| C12 | Four HTTP call sites' success-path deserialization unchanged | true |
| C13 | `pub mod retry;` in `src/lib.rs` stays (already public, no change) | true |
| C14 | TDD order: failing tests in `src/retry.rs` first, then implementation, then call sites | true |
| C15 | No CLI/REST/MCP/schema changes; no `corpus/` or `corpus_sample15/` changes | true |
| C16 | Single-card atomic commit, fixed message format, no `git commit --amend` | true |
| C17 | Cycle 13 `tasks/plan.md` / `tasks/todo.md` not overwritten; artifacts under `tasks/c9-defensive-retry/` | true |
| C18 | No secrets or API keys committed | true |
| C19 | Scope is exactly C9 (no C4/C6/C8/C10, no P2-3 `main.rs` bootstrap split) | true |
| C20 | No production code written by planner (this plan only) | true |
