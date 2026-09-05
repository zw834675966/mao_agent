# Docs Hygiene (定向卫生包 B) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align docs and ignore rules with post-API push state at `8acb512` without changing runtime behavior.

**Architecture:** Docs-only / ignore-only surgical edits on the existing single-crate Rust repo. No new modules, no API changes. Verification is grep + `cargo test --no-default-features` (expect 56 passed). Work from local checkout `D:\rust\mao_agent` (Cloud Agents unavailable on current plan).

**Tech Stack:** Git, Markdown docs, `.gitignore`, Rust test suite (`cargo test --no-default-features`).

## Global Constraints

- Spec: `docs/superpowers/specs/2026-09-05-docs-hygiene-design.md` (approved).
- No source (`src/**`), CI, or Cargo.toml changes.
- Do not commit secrets, `config.toml`, `data/`, `.pai/`, or `nul`.
- Keep `cargo test --no-default-features` at **56 passed**.
- Suggested final commit message: `docs: align hygiene with post-API push (plan, agent instructions, ignore logs)`.
- Open/update PR to `main`; user reviews on GitHub.

---

## File map

| File | Responsibility |
|------|----------------|
| `tasks/plan.md` | Mark completed checkpoints; fix stale "44" → "56" in Task 3 label |
| `AGENT_INSTRUCTIONS.md` | Offline dim 512; `--no-default-features` ingest/self-heal include `--offline` |
| `.gitignore` | Ignore `logs/` |
| `README.md` | Read-only check; edit only if drift found |
| `tasks/todo.md` | Optional micro-align; no new tasks |
| local `nul` | Delete if present (never stage) |

---

### Task 1: Check off `tasks/plan.md` and align test-count label

**Files:**
- Modify: `tasks/plan.md`
- Test: N/A (docs); verify with Select-String / grep

**Interfaces:**
- Consumes: Spec §In scope item 1; current `tasks/todo.md` all `[x]`
- Produces: plan checkpoints all checked; Task 3 label says 56 not 44

- [ ] **Step 1: Open `tasks/plan.md` and apply these exact checkbox / label edits**

Change:

```markdown
- [ ] Focused tests pass
```

to:

```markdown
- [x] Focused tests pass
```

Change:

```markdown
- [x] Task 3: README ingest/search/`--offline` + test count 44
```

to:

```markdown
- [x] Task 3: README ingest/search/`--offline` + test count 56
```

Change the Complete checkpoint block from:

```markdown
- [ ] `cargo test --no-default-features`
- [ ] `cargo fmt --check`
- [ ] `cargo clippy --no-default-features --all-targets -- -D warnings`
```

to:

```markdown
- [x] `cargo test --no-default-features`
- [x] `cargo fmt --check`
- [x] `cargo clippy --no-default-features --all-targets -- -D warnings`
```

- [ ] **Step 2: Verify no unchecked boxes remain in `tasks/plan.md`**

Run (PowerShell):

```powershell
Select-String -Path tasks\plan.md -Pattern '\[ \]'
```

Expected: no matches (empty output).

- [ ] **Step 3: Commit**

```bash
git add tasks/plan.md
git commit -m "docs: check off plan.md checkpoints (embed defaults complete)"
```

---

### Task 2: Fix `AGENT_INSTRUCTIONS.md` offline dimension and ingest commands

**Files:**
- Modify: `AGENT_INSTRUCTIONS.md` (approx. lines 122, 145, 200, 204)
- Test: N/A (docs); verify with Select-String

**Interfaces:**
- Consumes: Spec §In scope item 2; `LOCAL_EMBEDDING_DIM` = 512 for offline deterministic path
- Produces: offline example uses `dimension: 512`; `--no-default-features` ingest snippets include `--offline`

- [ ] **Step 1: Fix ingest pipeline command (~line 122)**

Change:

```bash
cargo run --no-default-features -- ingest --corpus-dir corpus --batch-size 32
```

to:

```bash
cargo run --no-default-features -- ingest --offline --corpus-dir corpus --batch-size 32
```

- [ ] **Step 2: Fix library example `EmbedderSelection` (~line 145)**

In the block with `offline: true`, change:

```rust
        dimension: 1536,
```

to:

```rust
        dimension: 512,
```

Leave `offline: true`, `model: "deterministic"`, and surrounding code unchanged.

- [ ] **Step 3: Fix self-heal table missing-index row (~line 200)**

Change the Automated Self-Healing Action cell from:

```text
Execute: `cargo run --no-default-features -- init-samples && cargo run --no-default-features -- ingest --corpus-dir corpus`
```

to:

```text
Execute: `cargo run --no-default-features -- init-samples && cargo run --no-default-features -- ingest --offline --corpus-dir corpus`
```

- [ ] **Step 4: Fix self-heal table access-denied regenerate hint (~line 204)**

Change:

```text
Regenerate via `cargo run --no-default-features -- ingest`.
```

to:

```text
Regenerate via `cargo run --no-default-features -- ingest --offline`.
```

- [ ] **Step 5: Verify**

Run:

```powershell
Select-String -Path AGENT_INSTRUCTIONS.md -Pattern 'dimension: 1536'
Select-String -Path AGENT_INSTRUCTIONS.md -Pattern 'dimension: 512'
Select-String -Path AGENT_INSTRUCTIONS.md -Pattern '--no-default-features -- ingest(?! --offline)'
```

Expected:
- first command: no matches
- second: at least one match (`dimension: 512`)
- third: no matches (every `--no-default-features -- ingest` is followed by `--offline`)

- [ ] **Step 6: Commit**

```bash
git add AGENT_INSTRUCTIONS.md
git commit -m "docs: fix offline embed dim and --offline ingest in AGENT_INSTRUCTIONS"
```

---

### Task 3: Ignore `logs/`, delete local `nul`, README/todo sanity check

**Files:**
- Modify: `.gitignore`
- Delete (local only): `nul` if present
- Possibly modify: `README.md`, `tasks/todo.md` (only if drift)
- Test: `cargo test --no-default-features`

**Interfaces:**
- Consumes: Spec §In scope items 3–6
- Produces: `logs/` ignored; working tree clean of expected dirt; tests still 56 green

- [ ] **Step 1: Append `logs/` to `.gitignore`**

After the existing `nul` line, ensure the file ends with:

```gitignore
.pai/
nul
logs/
```

Do not remove existing entries (`/target`, `config.toml`, `.pai/`, `nul`, `/data`, etc.).

- [ ] **Step 2: Delete local `nul` if it exists**

```powershell
if (Test-Path -LiteralPath 'nul') { Remove-Item -LiteralPath '.\nul' -Force }
# On Windows, prefer deleting via cmd if PowerShell blocks the reserved name:
cmd /c "if exist nul del /f /q nul"
```

Do **not** `git add nul`.

- [ ] **Step 3: README consistency check (edit only if needed)**

```powershell
Select-String -Path README.md -Pattern '34 |44 |56 '
Select-String -Path README.md -Pattern '默认 512'
```

Expected: README mentions **56** tests and offline default **512**. If already correct → **no README edit**. If wrong → fix the number/wording only.

- [ ] **Step 4: `tasks/todo.md` micro-check**

If any line still says test count **44**, change to **56**. If already aligned with completed checkboxes → no edit.

- [ ] **Step 5: Run full test suite**

```bash
cargo test --no-default-features
```

Expected: `56 passed; 0 failed` (30 lib + 8 api + 1 chunker + 5 config + 1 e2e + 4 hybrid + 7 vector_store).

- [ ] **Step 6: Confirm `logs/` no longer appears as untracked**

```bash
git status -sb
```

Expected: `logs/` not listed as `??` (ignored). No unexpected modified source files.

- [ ] **Step 7: Commit hygiene files**

```bash
git add .gitignore
# add README.md / tasks/todo.md only if modified
git add -u README.md tasks/todo.md 2>nul
git commit -m "docs: align hygiene with post-API push (plan, agent instructions, ignore logs)"
```

If Task 1–2 already committed separately, this commit message can be `chore: ignore logs/ and finish hygiene cleanup` instead; ensure the PR description still cites the suggested overall message.

- [ ] **Step 8: Push branch and open/update PR to `main`**

```bash
git push -u origin HEAD
gh pr create --title "docs: hygiene cleanup (定向卫生包 B)" --body "## Summary
- Check off tasks/plan.md; align test count label to 56
- Fix AGENT_INSTRUCTIONS offline dimension 512 + --offline ingest/self-heal
- Ignore logs/; delete local nul if present
- README/todo only if drift

Spec: docs/superpowers/specs/2026-09-05-docs-hygiene-design.md

## Test plan
- [ ] cargo test --no-default-features → 56 passed
- [ ] git status clean of logs/
- [ ] grep confirms no dimension: 1536 offline example
"
```

If a design-only PR (#1) already exists on the same branch, push and update that PR body instead of opening a duplicate.

---

## Spec coverage self-review

| Spec item | Task |
|-----------|------|
| plan.md checkpoints | Task 1 |
| AGENT_INSTRUCTIONS dim + offline ingest | Task 2 |
| `.gitignore` logs/ | Task 3 |
| README check | Task 3 Step 3 |
| delete local nul | Task 3 Step 2 |
| todo.md micro-align | Task 3 Step 4 |
| 56 tests green | Task 3 Step 5 |
| PR delivery | Task 3 Step 8 |
| Out of scope (src/CI/API) | Global Constraints — no tasks touch these |

No TBD/TODO placeholders in steps. No source edits.
