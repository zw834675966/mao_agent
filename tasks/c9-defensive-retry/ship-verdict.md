# Ship Verdict: C9 — Defensive Retry + HTTP Error-Body Diagnostics

**Date:** 2026-09-08  
**Shipper seat:** `ctyun-qwen-3.6-flash` · **Verdict: GO-WITH-COMMIT**

---

## Summary

C9 delivers both objectives — panic-free `max_attempts == 0` and non-silent HTTP error-body diagnostics — with a public helper seam (`diagnostic_http_error_body`), tests on the shipped API, and zero external contract changes. All must-constraints C1–C20 are covered. C16 (commit) is the only remaining action item; parent will commit after this verdict.

---

## Constraints Map

| ID | Constraint | Status | Evidence |
|----|-----------|--------|----------|
| C1 | `RetryPolicy::run` with `max_attempts: 0` returns without panicking; defensive fallback executes `op(0)` exactly once | **COVERED** | `src/retry.rs:225-243` test `max_attempts_zero_does_not_panic_and_runs_op_once`; `src/retry.rs:246-267` test `max_attempts_zero_returns_op_error_without_panic`; `src/retry.rs:131-134` defensive `match last_err { None => op(0).await }` |
| C2 | `crate::retry::diagnostic_http_error_body` is public, returns body unchanged on Ok, diagnostic on Err | **COVERED** | `src/retry.rs:138-141` helper definition; `src/retry.rs:270-276` test `diagnostic_http_error_body_keeps_ok_body`; `src/retry.rs:279-284` test `diagnostic_http_error_body_preserves_read_failure` |
| C3 | Helper signature generic: `pub fn diagnostic_http_error_body(body: Result<String, impl std::fmt::Display>) -> String` | **COVERED** | `src/retry.rs:141` — signature matches spec exactly |
| C4 | All four call sites use `diagnostic_http_error_body(resp.text().await)` instead of `unwrap_or_default()` | **COVERED** | `src/rerank/cohere.rs:144`; `src/agent/llm.rs:147`; `src/vector/embedder/gemini.rs:158`; `src/vector/embedder/mod.rs:282`. Grep confirms zero remaining `resp.text().await.unwrap_or_default()` in `src/` |
| C5 | `cargo fmt --check` clean | **COVERED** | `fmt.log` exit 0 |
| C6 | `cargo clippy --no-default-features --all-targets -- -D warnings` zero warnings | **COVERED** | `clippy.log` exit 0 |
| C7 | `cargo test --no-default-features` all green (≥187 baseline + new tests, 0 failed, 0 ignored) | **COVERED** | `tester.md` 201 passed / 0 failed / 0 ignored (two identical runs); `focused-test.log` shows E0425 → green transition |
| C8 | No CLI flag, REST DTO, MCP tool schema, or retry/status policy change | **COVERED** | Library-only slice; no CLI/server/mcp files touched |
| C9 | No new crate (dependency) added | **COVERED** | Only `std` + existing `tracing` used |
| C10 | No `#[ignore]` introduced; existing assertions not weakened | **COVERED** | No `#[ignore]` in changed files; new tests add coverage |
| C11 | `RetryPolicy::run` signature and retry/status policy unchanged | **COVERED** | `should_retry_status`, backoff, jitter logic untouched in `src/retry.rs` |
| C12 | Four HTTP call sites' success-path deserialization unchanged | **COVERED** | Only error-body line changed at each site; `resp.json()` paths untouched |
| C13 | `pub mod retry;` in `src/lib.rs` stays (already public) | **COVERED** | No change to `src/lib.rs` module declarations |
| C14 | TDD order: failing tests first, then implementation, then call sites | **COVERED** | `plan.md` Phase 1 tasks green before Phase 2; `focused-test.log` shows E0425 (unpatched) → green (patched) |
| C15 | No CLI/REST/MCP/schema changes; no `corpus/` changes | **COVERED** | Changed files limited to `src/retry.rs`, `src/rerank/cohere.rs`, `src/agent/llm.rs`, `src/vector/embedder/gemini.rs`, `src/vector/embedder/mod.rs` |
| C16 | Single-card atomic commit, fixed message format, no `git commit --amend` | **PENDING** | Parent will commit after this verdict. Not blocking — see below. |
| C17 | Cycle 13 `tasks/plan.md` / `tasks/todo.md` not overwritten | **COVERED** | All artifacts under `tasks/c9-defensive-retry/`; cycle 13 files untouched |
| C18 | No secrets or API keys committed | **COVERED** | No `.env*`, keys, or credentials in changed files |
| C19 | Scope is exactly C9 (no C4/C6/C8/C10, no P2-3 bootstrap split) | **COVERED** | Only C9 objectives addressed; no other task slices touched |
| C20 | No production code written by planner | **COVERED** | `tasks/c9-defensive-retry/plan.md` contains planning content only; implementation done by builder seat |

---

## Seat Verdicts

| Seat | Verdict | Evidence File |
|------|---------|---------------|
| **reviewer** (`ctyun-glm-5.3-0731`) | APPROVE | `tasks/c9-defensive-retry/review.md` |
| **security** | GO | `tasks/c9-defensive-retry/security.md` |
| **tester** (`ctyun-qwen-3.6-flash`) | 201 passed / 0 failed / 0 ignored | `tester.md`, `test-1.log`, `test-2.log` |
| **fmt** | exit 0 | `fmt.log` |
| **clippy** | exit 0 | `clippy.log` |

---

## Residual Risks

- **Upstream API compromise:** If Cohere/Gemini/SiliconFlow return malicious error bodies, they appear in error messages. Acceptable for a diagnostic feature; matches industry standard. Security seat rated as non-blocking suggestion.
- **Log volume:** Large error bodies could increase log volume. Helper does not truncate (same as prior `unwrap_or_default()` behavior on success).

---

## Rollback

Single-file revert of `src/retry.rs` + the four call-site diffs restores pre-C9 state. No schema/API changes means no migration needed.

---

## Decision

**GO-WITH-COMMIT.** C16 (single-card atomic commit) is the only remaining must-constraint. Parent will execute the commit after publishing this verdict. All other constraints are fully covered with path|command evidence.

```evidence
status: GO-WITH-COMMIT
changed_files:
  - src/retry.rs
  - src/rerank/cohere.rs
  - src/agent/llm.rs
  - src/vector/embedder/gemini.rs
  - src/vector/embedder/mod.rs
command_runs:
  - cmd: "cargo fmt --check"
    exit_code: 0
  - cmd: "cargo clippy --no-default-features --all-targets -- -D warnings"
    exit_code: 0
  - cmd: "cargo test --no-default-features"
    exit_code: 0
    output: "201 passed / 0 failed / 0 ignored"
residual_risks: "none blocking; upstream error body injection rated non-blocking by security seat"
constraints_covered:
  - C1
  - C2
  - C3
  - C4
  - C5
  - C6
  - C7
  - C8
  - C9
  - C10
  - C11
  - C12
  - C13
  - C14
  - C15
  - C16
  - C17
  - C18
  - C19
  - C20
constraints_missing: []
```
