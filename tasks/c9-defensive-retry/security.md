# Security Review: C9 Defensive Retry + HTTP Error-Body Diagnostics

**Scope:** `src/retry.rs` helper `diagnostic_http_error_body` and four call sites
**Date:** 2026-09-08
**Reviewer:** security auditor (read-only)
**Dao seat:** purpose | one-contradiction | one-easy-step | locate-then-read | claim=path|command

---

## Summary

The C9 slice introduces `diagnostic_http_error_body` to replace silent `unwrap_or_default()` on HTTP error response body reads. The change is defensive: it preserves a diagnostic when the body read itself fails (e.g., connection reset mid-read), without altering retry policy, auth, storage, or success-path deserialization.

**Security verdict: GO** — no blocking issues. One suggestion for future hardening.

---

## Threat Model

| Boundary | Trust Level | Data Flow |
|----------|-------------|-----------|
| Outbound HTTP error response body | **Untrusted** | Upstream API (Cohere, Gemini, SiliconFlow, OpenAI-compatible) → `resp.text().await` → `diagnostic_http_error_body` → `VectorError` → `ApiError` → JSON response / logs |

**Assets at risk:**
- API keys (in `bearer_auth` headers, not in bodies)
- User query text (in request bodies, not in error response bodies)
- System integrity (DoS via log flooding)

---

## Findings

### Critical: None

No critical security issues identified.

### Important: None

No important security issues identified.

### Suggestion: Log Injection Hardening (Future)

**Observation:** The error body (when successfully read) is passed through unchanged into error strings like:
- `"Cohere rerank HTTP {status}: {body}"`
- `"LLM API returned HTTP {status}: {body}"`
- `"Embedding API returned HTTP {status}: {body}"`

These strings flow through:
1. `tracing::warn!` (retry loop logging)
2. `VectorError` → `ApiError` → Axum JSON response

**Risk:** If an upstream API returns malicious content in an error body (e.g., JSON with newlines, control characters, or crafted strings resembling log injection), it could:
- Pollute structured logs (though `tracing` handles this)
- Confuse operators reading logs
- In extreme cases, if logs are parsed by brittle tools, cause misinterpretation

**Mitigation in place:**
- Axum's `Json` response uses proper JSON serialization (escapes control characters)
- `tracing` structured logging handles string fields safely
- No direct shell execution or HTML rendering of error bodies

**Recommendation (future hardening, not blocking):** Consider truncating and sanitizing error bodies before inclusion in error messages. This is not required for C9 but would harden against log injection.

**Status:** Not a blocker. Current handling matches industry standard for upstream API error diagnostics.

---

## Verification Checklist

| Check | Status | Evidence |
|-------|--------|----------|
| No secrets in code | PASS | `src/retry.rs` contains no API keys |
| No new dependencies | PASS | Only `std` + existing `tracing` |
| All `resp.text()` call sites use helper | PASS | 4 matches in `src/`, all wrapped in `diagnostic_http_error_body` |
| `max_attempts == 0` is panic-free | PASS | `src/retry.rs:131-134` — defensive `op(0).await` when `last_err` is `None` |
| No authN/authZ changes | PASS | No auth code modified |
| No storage changes | PASS | No persistence code modified |
| Error bodies don't reach HTML/innerHTML | PASS | Axum JSON serialization only |
| No SSRF introduced | PASS | No new URL construction |

---

## Changed Files

- `src/retry.rs` — adds `diagnostic_http_error_body` helper, defensive `max_attempts == 0` handling
- `src/rerank/cohere.rs` — uses helper at line 144
- `src/agent/llm.rs` — uses helper at line 147
- `src/vector/embedder/gemini.rs` — uses helper at line 158
- `src/vector/embedder/mod.rs` — uses helper at line 282

---

## Residual Risks

- **Upstream API compromise:** If Cohere/Gemini/SiliconFlow are compromised and return malicious error bodies, the application will include them in error messages. This is acceptable for a diagnostic feature and matches standard practice.
- **Log volume:** Large error bodies could increase log volume. The helper does not truncate, but this is existing behavior (previously `unwrap_or_default()` would also pass through full bodies on success).

---

## Go/No-Go

**GO** from a security lens. The change is defensive, adds no new attack surface, and improves observability without compromising security boundaries.

---

```evidence
status: GO
changed_files:
  - src/retry.rs
  - src/rerank/cohere.rs
  - src/agent/llm.rs
  - src/vector/embedder/gemini.rs
  - src/vector/embedder/mod.rs
command_runs:
  - grep -n "resp.text()" src/**/*.rs (4 matches, all use helper)
residual_risks: "none blocking; suggestion for future log injection hardening documented"
```