# Spec: Align AGENTS.md embed dim + VerifyRequest comment

Confirmed intent (2026-09-05): fix two docs/comment drifts found in the real offline E2E run report. No runtime behavior changes.

## Objective

- **Who:** Maintainers and agents reading `AGENTS.md` / API DTO docs.
- **Why:** (1) `AGENTS.md` still says the CLI deterministic embed dimension constant is **384**; offline / `LOCAL_EMBEDDING_DIM` is **512**. (2) `VerifyRequest.context_chunks` rustdoc says empty chunks fall back to full-text lookup by title; `handlers/verify.rs` hard-rejects empty with **400**.
- **Success:** Docs match code. `cargo test --no-default-features` still **56** passed.

## In scope

1. **`AGENTS.md`** — Change the testing-notes bullet that references "CLI's 384 constant" to **512**. Do not change the note that unit-test dims may be 64/128/256.
2. **`src/server/dto.rs`** — Update `VerifyRequest.context_chunks` doc comment to state chunks must be non-empty (empty → 400); optionally mention retrieving via `/api/v1/search`. Do not change fields, serde attributes, or types.

## Out of scope

Handler logic, new tests, socket-level API tests, CI, embedder code, robustness gaps, Cohere/FastEmbed verification, non-Windows.

## Acceptance

- [ ] `AGENTS.md` has no stale CLI **384** claim for deterministic dim; states **512** where that constant is referenced
- [ ] `VerifyRequest.context_chunks` comment matches handler (non-empty required / 400)
- [ ] Diff touches only the two files above (plus this spec/plan if committed together)
- [ ] `cargo test --no-default-features` → 56 passed

## Boundaries

- **Always:** Docs/comments only; keep tests green.
- **Ask first:** Changing handler to implement title fallback; expanding into socket tests.
- **Never:** Commit secrets; silent embedder fallbacks.

## Open questions

None — scope approved; user asked for short spec + plan before implement.
