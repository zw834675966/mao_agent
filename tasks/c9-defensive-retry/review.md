# Review: C9 — Defensive Retry + HTTP Error-Body Diagnostics

**Seat:** reviewer (`ctyun-glm-5.3-0731`) · **Date:** 2026-09-08
**Verdict:** APPROVE

## Review Summary

C9 delivers both objectives — panic-free `max_attempts == 0` and non-silent HTTP error-body diagnostics — with a public helper seam, tests on the shipped API, and no CLI/REST/MCP schema change.

### Critical / Important / Suggestions

None blocking.

### Axes

1. **Correctness** — `RetryPolicy::run` no longer `expect`s; `max_attempts == 0` takes `None => op(0)`. Helper returns Ok body unchanged and Err diagnostic. Four call sites use the helper; grep shows no remaining `resp.text().await.unwrap_or_default()`.
2. **Readability** — helper is a few lines with a factual doc comment.
3. **Architecture** — helper lives in `retry` (HTTP retry utilities). Generic `impl Display` lets tests avoid constructing `reqwest::Error`. No new crate.
4. **Security** — error bodies are untrusted; they stay in `VectorError` strings. Separate security seat: GO.
5. **Performance** — `unwrap_or_else` allocates only on Err. Not a hot path.

### Verification Story

| Check | Evidence |
|-------|----------|
| panic-free zero attempts | `src/retry.rs` `match last_err`; tests `max_attempts_zero_*` |
| helper contract | `diagnostic_http_error_body_*` tests |
| four call sites | `cohere.rs`, `llm.rs`, `gemini.rs`, `embedder/mod.rs` |
| tester | `cargo test --no-default-features` 201 passed / 0 failed / 0 ignored |

```evidence
status: APPROVE
changed_files:
  - src/retry.rs
  - src/rerank/cohere.rs
  - src/agent/llm.rs
  - src/vector/embedder/gemini.rs
  - src/vector/embedder/mod.rs
command_runs: []
residual_risks: "none"
```
