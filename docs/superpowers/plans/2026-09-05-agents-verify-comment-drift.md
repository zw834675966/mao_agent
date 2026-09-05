# AGENTS.md + VerifyRequest Comment Drift Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align `AGENTS.md` CLI deterministic embed dim (384→512) and `VerifyRequest.context_chunks` rustdoc with the live handler (empty → 400), without changing runtime behavior.

**Architecture:** Two surgical text edits. No new modules. Verification is grep + `cargo test --no-default-features` (expect 56 passed). Work on local checkout `D:\rust\mao_agent` (Cloud Agents unavailable).

**Tech Stack:** Markdown, Rust doc comments, Git, `cargo test --no-default-features`.

## Global Constraints

- Spec: `docs/superpowers/specs/2026-09-05-agents-verify-comment-drift-design.md`
- Docs/comments only — do not change handler logic, serde, types, tests, or CI
- Do not commit secrets, `config.toml`, `data/`, `.pai/`, `nul`, or `logs/`
- Keep `cargo test --no-default-features` at **56 passed**
- Suggested commit message: `docs: align AGENTS.md embed dim and VerifyRequest comment`
- Open PR to `main`; user reviews on GitHub (or later chooses local merge)

---

## File map

| File | Change |
|------|--------|
| `AGENTS.md` | Testing notes: "CLI's 384 constant" → "CLI's 512 constant" |
| `src/server/dto.rs` | `VerifyRequest.context_chunks` rustdoc → non-empty / 400 |

---

### Task 1: Fix `AGENTS.md` CLI dim reference

**Files:**
- Modify: `AGENTS.md` (testing notes bullet, currently references 384)
- Test: N/A (docs); verify with Select-String

**Interfaces:**
- Consumes: Spec §In scope item 1; `LOCAL_EMBEDDING_DIM` = 512
- Produces: no "CLI's 384" claim; CLI constant described as 512

- [ ] **Step 1: Apply exact edit**

In `AGENTS.md`, change:

```markdown
- Vector dim in tests varies (64/128/256) — construct stores via `VectorStore::new_deterministic(dim)` rather than copying CLI's 384 constant.
```

to:

```markdown
- Vector dim in tests varies (64/128/256) — construct stores via `VectorStore::new_deterministic(dim)` rather than copying CLI's 512 constant.
```

Keep the 64/128/256 sentence intent unchanged.

- [ ] **Step 2: Verify**

```powershell
Select-String -Path AGENTS.md -Pattern 'CLI''s 384|CLI's 384'
Select-String -Path AGENTS.md -Pattern 'CLI''s 512|CLI's 512'
```

Expected: first empty; second matches the testing-notes bullet.

- [ ] **Step 3: Commit** (or fold into Task 2 single commit if implementing both in one pass)

```bash
git add AGENTS.md
git commit -m "docs: fix AGENTS.md CLI deterministic embed dim 384→512"
```

---

### Task 2: Fix `VerifyRequest.context_chunks` rustdoc + verify suite

**Files:**
- Modify: `src/server/dto.rs` (`VerifyRequest` field comment only)
- Test: `cargo test --no-default-features`

**Interfaces:**
- Consumes: Spec §In scope item 2; handler behavior in `src/server/handlers/verify.rs` (empty → 400)
- Produces: rustdoc matches hard reject; no field/serde changes

- [ ] **Step 1: Apply exact comment edit**

Replace:

```rust
    /// 用于比对的上下文块；若为空则从全文索引中按 title 检索兜底
    #[serde(default)]
    pub context_chunks: Vec<DocumentChunk>,
```

with:

```rust
    /// 用于比对的上下文块；不得为空（空则 400）。可先用 `/api/v1/search` 取回 chunk 再传入。
    #[serde(default)]
    pub context_chunks: Vec<DocumentChunk>,
```

Do not change `#[serde(default)]`, field type, or surrounding fields.

- [ ] **Step 2: Confirm handler still rejects empty (read-only check)**

```powershell
Select-String -Path src\server\handlers\verify.rs -Pattern 'context_chunks must not be empty'
```

Expected: one match (behavior unchanged; docs now agree).

- [ ] **Step 3: Run full suite**

```bash
cargo test --no-default-features
```

Expected: 56 passed; 0 failed.

- [ ] **Step 4: Commit + push PR**

```bash
git add AGENTS.md src/server/dto.rs
git commit -m "docs: align AGENTS.md embed dim and VerifyRequest comment"
git push -u origin HEAD
gh pr create --title "docs: align AGENTS.md embed dim and VerifyRequest comment" --body "## Summary
- AGENTS.md: CLI deterministic embed dim 384→512
- VerifyRequest.context_chunks rustdoc: empty → 400 (no title fallback)

Spec: docs/superpowers/specs/2026-09-05-agents-verify-comment-drift-design.md

## Test plan
- [ ] cargo test --no-default-features → 56 passed
- [ ] no CLI 384 claim left in AGENTS.md
- [ ] dto comment matches verify handler
"
```

If Tasks 1–2 use separate commits, push both; PR description still lists both files.

---

## Spec coverage self-review

| Spec item | Task |
|-----------|------|
| AGENTS.md 384→512 | Task 1 |
| VerifyRequest comment | Task 2 |
| 56 tests green | Task 2 Step 3 |
| Out of scope (handler/tests/API) | Global Constraints |

No TBD placeholders. No source behavior edits.
