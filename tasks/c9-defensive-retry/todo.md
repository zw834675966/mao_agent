# C9 — Defensive Retry + HTTP Error-Body Diagnostics (P2-13)

Baseline: `cargo test --no-default-features` 187 passed / 0 failed at `f34fb93`.
All artifacts under `tasks/c9-defensive-retry/`. Cycle 13 `tasks/plan.md` / `tasks/todo.md` untouched.

## Task 1: Add failing tests for `max_attempts == 0` and `diagnostic_http_error_body`

**Description:** In `src/retry.rs` `#[cfg(test)] mod tests`, add (a) a `tokio::test` that runs `RetryPolicy { max_attempts: 0, .. }` with a closure and asserts it does not panic, calls `op(0)` exactly once, and returns that result; (b) a test that `diagnostic_http_error_body(Ok("body"))` returns `"body"` unchanged; (c) a test that `diagnostic_http_error_body(Err("connection reset"))` returns a string containing `"<failed to read error response body:"` and the error text. These fail against current code (helper does not exist; `expect` path is the target). No production code yet.

**Acceptance criteria:**
- [x] `max_attempts == 0` test exists and fails (or does not compile) against current `run`/no-helper
- [x] `diagnostic_http_error_body` Ok/Err tests exist and fail against current code
- [x] Tests use `Err("connection reset")` (no `reqwest::Error` construction), `AtomicU32` for attempt counting, `tokio::test` for async

**Verification:**
- [ ] `cargo test --no-default-features retry` shows the new tests failing (red)
- [ ] `cargo fmt --check` clean

**Dependencies:** None

**Files likely touched:**
- `src/retry.rs`

**Estimated scope:** Small (1 file)

## Task 2: Implement `diagnostic_http_error_body` helper + defensive `run` fallback

**Description:** Add the public helper `pub fn diagnostic_http_error_body(body: Result<String, impl std::fmt::Display>) -> String` returning `body.unwrap_or_else(|e| format!("<failed to read error response body: {e}>"))`. Replace the terminal `Err(last_err.expect("max_attempts >= 1"))` in `RetryPolicy::run` with `match last_err { Some(e) => Err(e), None => op(0).await }`. Keep `run` signature and retry/status policy unchanged. Make Task 1 tests green.

**Acceptance criteria:**
- [x] Helper is `pub` on `crate::retry`, generic over `impl std::fmt::Display`, message contains `<failed to read error response body:`
- [x] `run` returns `Err(last_err)` when present, else runs one defensive `op(0)`; no `expect` remains
- [x] Task 1 tests pass (green)

**Verification:**
- [ ] `cargo test --no-default-features retry` green (new + existing retry tests: `retries_then_succeeds`, `stops_when_not_retryable`, `embeddings_http_preserves_legacy_embed_timing`, `backoff_grows_then_caps`)
- [ ] `cargo fmt --check` clean

**Dependencies:** Task 1

**Files likely touched:**
- `src/retry.rs`

**Estimated scope:** Small (1 file)

## Checkpoint: After Tasks 1-2
- [ ] `cargo test --no-default-features retry` green
- [ ] `cargo fmt --check` clean
- [ ] Helper green before touching call sites

## Task 3: Replace `unwrap_or_default()` at `cohere.rs:144` and `llm.rs:147`

**Description:** In `src/rerank/cohere.rs` (line 144) and `src/agent/llm.rs` (line 147), replace `let body = resp.text().await.unwrap_or_default();` with `let body = diagnostic_http_error_body(resp.text().await);`. Import the helper (e.g. `use crate::retry::diagnostic_http_error_body;`). Do not touch the success-path `resp.json()` deserialization or the error-format strings.

**Acceptance criteria:**
- [x] Both call sites call `diagnostic_http_error_body(resp.text().await)`
- [x] No `resp.text().await.unwrap_or_default()` remains in these two files
- [x] Success-path deserialization unchanged

**Verification:**
- [ ] `cargo test --no-default-features` green (call-site modules compile)
- [ ] `cargo fmt --check` clean

**Dependencies:** Task 2

**Files likely touched:**
- `src/rerank/cohere.rs`
- `src/agent/llm.rs`

**Estimated scope:** Small (2 files)

## Task 4: Replace `unwrap_or_default()` at `gemini.rs:158` and `mod.rs:282`

**Description:** In `src/vector/embedder/gemini.rs` (line 158) and `src/vector/embedder/mod.rs` (line 282), replace `let body = resp.text().await.unwrap_or_default();` with `let body = diagnostic_http_error_body(resp.text().await);`. Import the helper. Do not touch success-path deserialization or error-format strings.

**Acceptance criteria:**
- [x] Both call sites call `diagnostic_http_error_body(resp.text().await)`
- [x] No `resp.text().await.unwrap_or_default()` remains in these two files
- [x] Success-path deserialization unchanged

**Verification:**
- [ ] `cargo test --no-default-features` green
- [ ] `cargo fmt --check` clean

**Dependencies:** Task 2

**Files likely touched:**
- `src/vector/embedder/gemini.rs`
- `src/vector/embedder/mod.rs`

**Estimated scope:** Small (2 files)

## Checkpoint: After Tasks 3-4
- [ ] grep shows no remaining `resp.text().await.unwrap_or_default()` anywhere
- [ ] `cargo test --no-default-features` all green

## Task 5: Full CI gate — fmt, clippy, full test suite

**Description:** Run the full official gate. Confirm no CLI/REST/MCP/schema/policy change, no new crate, no `#[ignore]`. Prepare a single-card atomic commit (fixed message format, no `git commit --amend`).

**Acceptance criteria:**
- [ ] `cargo fmt --check` clean
- [ ] `cargo clippy --no-default-features --all-targets -- -D warnings` zero warnings
- [ ] `cargo test --no-default-features` all green (≥187 baseline + new tests, 0 failed, 0 ignored)
- [ ] No CLI flag / REST DTO / MCP tool schema / retry-status policy change; no new crate; no `#[ignore]`

**Verification:**
- [ ] All three gate commands pass
- [ ] grep confirms no `resp.text().await.unwrap_or_default()` remains

**Dependencies:** Tasks 3, 4

**Files likely touched:**
- None (verification only) — commit of Tasks 1-4 changes

**Estimated scope:** Small (verification)

## Checkpoint: Complete
- [ ] All acceptance criteria met
- [ ] Ready for review (later `security` seat in scope per ASSUMPTIONS #6)
