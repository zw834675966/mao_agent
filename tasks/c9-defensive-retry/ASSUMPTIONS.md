# I0 + ASSUMPTIONS — C9 defensive retry + HTTP error-body

Date: 2026-09-08
Baseline remaining C-phase cards at `f34fb93`: C4 / C6 / C8 / C9 / C10
Already landed (do not re-do): C1 constant-time auth, C2 VectorError Deserialization, C3 GraphStore `MAOGS01\0`, C5 VectorStore lock-outside-IO, C7 CitationVerifier char-slice windows.

## Chosen slice

**C9 (P2-13)** — 关键路径防御性错误处理与 HTTP 响应体读取增强.

Why this one: multi-file (`src/retry.rs` + four HTTP call sites), independently testable without network, builder worktree justified. C4 is single-file CLI; C10 is one-module Tantivy; C6 SSE borrow; C8 OnceLock — deferred.

Dao (locate → read): `corpus/mao_dun_lun.md` §主要矛盾 — 「捉住了这个主要矛盾，一切问题就迎刃而解了。」 Main contradiction for this slice: silent empty HTTP error bodies / unreachable `expect` vs diagnostic, panic-free failure.

## ASSUMPTIONS I'M MAKING

1. Scope is **exactly C9**. No C4/C6/C8/C10, no P2-3 `main.rs` bootstrap split, no CLI/REST/MCP schema or flag changes.
2. The testable shipped seam for HTTP diagnostics is a small public helper on `retry` (e.g. `diagnostic_http_error_body`) used by all four `resp.text().await.unwrap_or_default()` sites. Tests drive that helper (and `RetryPolicy::run`) with no network.
3. `max_attempts == 0` must not panic. One defensive `op(0)` (or equivalent of today's `.max(1)`) is acceptable; retry/status policy otherwise unchanged.
4. Tests: existing `#[cfg(test)]` style in `src/retry.rs` (and call-site compile via those modules). `tempfile` / `DeterministicEmbedder` only if a neighboring test already uses them. `--no-default-features`, no `#[ignore]`, no new crates.
5. Human spec-review click is waived (OBJECTIVE). Assumptions live in this file and in the spec. Cycle 13 `tasks/plan.md` / `tasks/todo.md` are not overwritten; artifacts go under `tasks/c9-defensive-retry/`.
6. C9 touches outbound HTTP error-body handling → later `security` seat is in scope; no auth/storage change.

→ Proceeding with these. Correct them in the spec if a seat finds a conflict with the C9 card in `docs/实施方案-C阶段-技术债治理-2026-09-08.md`.
