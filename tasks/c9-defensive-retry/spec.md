# Spec: C9 — Defensive Retry + HTTP Error-Body Diagnostics (P2-13)

Date: 2026-09-08
Slice: **C9 only** (P2-13). Not C4/C6/C8/C10, not C1/C2/C3/C5/C7, not P2-3 `main.rs` bootstrap split.
Source card: `docs/实施方案-C阶段-技术债治理-2026-09-08.md` §2.9 / task card C9.
Baseline: `cargo test --no-default-features` 187 passed / 0 failed at `f34fb93`.

## Objective

Eliminate two failure modes on the critical outbound-HTTP path without changing any external contract:

1. **Panic on `max_attempts == 0`.** `RetryPolicy::run` ends with `Err(last_err.expect("max_attempts >= 1"))` (`src/retry.rs:131`). If a caller ever constructs a policy with `max_attempts: 0`, the loop body never runs, `last_err` stays `None`, and the `expect` panics the process. The slice must make `max_attempts == 0` panic-free.

2. **Silent empty HTTP error bodies.** Four call sites read a non-success response body with `resp.text().await.unwrap_or_default()` (`src/rerank/cohere.rs:144`, `src/agent/llm.rs:147`, `src/vector/embedder/gemini.rs:158`, `src/vector/embedder/mod.rs:282`). When the connection resets mid-body-read, the error becomes an empty string and the diagnostic (`"Cohere rerank HTTP 500: "`, `"LLM API returned HTTP 500: "`, etc.) loses its troubleshooting evidence. The slice must keep a body-read-failure diagnostic.

Success: `max_attempts == 0` never panics, and every HTTP error-body read failure surfaces a diagnostic string instead of a silent empty body — with no change to CLI flags, REST DTOs, MCP tool schemas, or retry/status policy.

## Tech Stack

- Rust **2024 edition**, single crate `mao_agent` (bin + lib), no workspace.
- `tokio` (async runtime, `tokio::time::sleep` in retry loop), `reqwest` (HTTP client), `tracing` (warn logging).
- **No new crates.** Reuse existing stdlib + current deps only.
- Tests: `#[cfg(test)]` modules, `tokio::test`, `--no-default-features` (no FastEmbed/Gemini/Cohere network).

## Commands

```bash
cargo fmt --check
cargo clippy --no-default-features --all-targets -- -D warnings
cargo test --no-default-features
```

Routine single-module run: `cargo test --no-default-features retry`

## Project Structure

```
src/retry.rs                    → RetryPolicy + new public helper diagnostic_http_error_body + #[cfg(test)]
src/rerank/cohere.rs:144        → call site 1 (uses helper)
src/agent/llm.rs:147            → call site 2 (uses helper)
src/vector/embedder/gemini.rs:158 → call site 3 (uses helper)
src/vector/embedder/mod.rs:282  → call site 4 (uses helper)
src/lib.rs:12                   → pub mod retry; (already public — no change)
tasks/c9-defensive-retry/spec.md → this spec
tasks/c9-defensive-retry/ASSUMPTIONS.md → assumptions (already present)
```

`tasks/plan.md` and `tasks/todo.md` are **not** touched. All C9 artifacts live under `tasks/c9-defensive-retry/`.

## Code Style

Match the existing `src/retry.rs` conventions: doc comments on public items, `tracing::warn!` for transient failures, `#[cfg(test)] mod tests` at the bottom of the module, `tokio::test` for async tests, `AtomicU32` counters for attempt counting.

Shipped seam — a small public helper on `crate::retry` so tests drive it without network:

```rust
/// Render an HTTP error response body, preserving a diagnostic when the body
/// read itself fails (e.g. connection reset mid-read) instead of silently
/// degrading to an empty string.
pub fn diagnostic_http_error_body(body: Result<String, reqwest::Error>) -> String {
    body.unwrap_or_else(|e| format!("<failed to read error response body: {e}>"))
}
```

Call sites replace `resp.text().await.unwrap_or_default()` with:

```rust
let body = diagnostic_http_error_body(resp.text().await);
```

Naming: the helper is `diagnostic_http_error_body` (public, on `crate::retry`). The defensive retry fallback keeps the existing `run` signature and returns `Err(last_err)` when present, else runs one defensive `op(0)`.

## Testing Strategy

- **Location:** `#[cfg(test)] mod tests` in `src/retry.rs`, matching the existing tokio tests there. Call-site modules compile via their own `#[cfg(test)]`; no new integration test files required.
- **No network:** tests drive `diagnostic_http_error_body` with a synthetic `Result<String, reqwest::Error>` and drive `RetryPolicy::run` with closures. No `tempfile` / `DeterministicEmbedder` needed (neither is used by `retry.rs` today).
- **No `#[ignore]`, no new crates.**
- **Coverage expectations:**
  - `max_attempts == 0` does not panic and returns a result (defensive `op(0)` runs once).
  - `diagnostic_http_error_body(Ok(body))` returns the body unchanged.
  - `diagnostic_http_error_body(Err(e))` returns a string containing `"<failed to read error response body:"` and the error text.
  - Existing retry tests (`retries_then_succeeds`, `stops_when_not_retryable`, `embeddings_http_preserves_legacy_embed_timing`, `backoff_grows_then_caps`) stay green.
- **Gate:** `cargo test --no-default-features` all green (≥187 baseline + new tests, 0 failed, 0 ignored).

## Boundaries

- **Always:**
  - Run `cargo fmt --check`, `cargo clippy --no-default-features --all-targets -- -D warnings`, `cargo test --no-default-features` before done.
  - Keep `RetryPolicy::run` signature and retry/status policy (`should_retry_status`, backoff, jitter) unchanged.
  - Keep the four HTTP call sites' success-path deserialization unchanged.
  - Use the shared `diagnostic_http_error_body` helper at all four call sites.
  - Make `max_attempts == 0` panic-free (one defensive `op(0)` or equivalent of today's `.max(1)` is acceptable).
  - Keep `pub mod retry;` in `src/lib.rs` (already public).
  - Single-card atomic commit, fixed message format, no `git commit --amend`.

- **Ask first:**
  - Any change to CLI flags, REST DTOs, MCP tool schemas, or retry/status policy.
  - Adding any dependency (new crate).
  - Touching `tasks/plan.md` or `tasks/todo.md`.
  - Expanding scope beyond C9 (e.g. C4/C6/C8/C10 or P2-3).

- **Never:**
  - Introduce `#[ignore]` to skip tests or weaken existing assertions.
  - Change the normal HTTP success-path response deserialization.
  - Change `corpus/` or `corpus_sample15/` files.
  - Commit secrets or API keys.
  - Use `git commit --amend`.

## Success Criteria

Testable, naming shipped functions:

1. `RetryPolicy::run` with `max_attempts: 0` returns without panicking; the defensive fallback executes `op(0)` exactly once and returns its result. Verified by a `#[cfg(test)]` test in `src/retry.rs`.
2. `crate::retry::diagnostic_http_error_body` is public and returns the body unchanged on `Ok`, and a `"<failed to read error response body: {e}>"` diagnostic on `Err`. Verified by `#[cfg(test)]` tests in `src/retry.rs`.
3. All four call sites (`src/rerank/cohere.rs`, `src/agent/llm.rs`, `src/vector/embedder/gemini.rs`, `src/vector/embedder/mod.rs`) call `diagnostic_http_error_body(resp.text().await)` instead of `resp.text().await.unwrap_or_default()`. Verified by grep (no remaining `unwrap_or_default()` on `resp.text()`).
4. `cargo fmt --check` clean; `cargo clippy --no-default-features --all-targets -- -D warnings` zero warnings; `cargo test --no-default-features` all green (≥187 baseline + new tests, 0 failed, 0 ignored).
5. No CLI flag, REST DTO, MCP tool schema, or retry/status policy change. No new crate. No `#[ignore]`.

## ASSUMPTIONS

Copied/adapted from `tasks/c9-defensive-retry/ASSUMPTIONS.md`:

1. Scope is **exactly C9**. No C4/C6/C8/C10, no P2-3 `main.rs` bootstrap split, no CLI/REST/MCP schema or flag changes.
2. The testable shipped seam for HTTP diagnostics is a small public helper on `retry` (`diagnostic_http_error_body`) used by all four `resp.text().await.unwrap_or_default()` sites. Tests drive that helper (and `RetryPolicy::run`) with no network.
3. `max_attempts == 0` must not panic. One defensive `op(0)` (or equivalent of today's `.max(1)`) is acceptable; retry/status policy otherwise unchanged.
4. Tests: existing `#[cfg(test)]` style in `src/retry.rs` (and call-site compile via those modules). `tempfile` / `DeterministicEmbedder` only if a neighboring test already uses them. `--no-default-features`, no `#[ignore]`, no new crates.
5. Human spec-review click is waived (OBJECTIVE). Assumptions live in this file and in `ASSUMPTIONS.md`. Cycle 13 `tasks/plan.md` / `tasks/todo.md` are not overwritten; artifacts go under `tasks/c9-defensive-retry/`.
6. C9 touches outbound HTTP error-body handling → later `security` seat is in scope; no auth/storage change.

## Open Questions

None blocking. The parent already decided the slice scope, the shipped seam name, the `max_attempts == 0` behavior, the test location, and the commands. Human spec review is waived; assumptions are surfaced above and in `ASSUMPTIONS.md`.
