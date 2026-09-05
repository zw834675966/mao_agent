# Spec: Docs & hygiene alignment after Axum API push (定向卫生包 B)

Confirmed intent (brainstorming): route **B** — align docs/ignore rules with repo state at `8acb512` without changing runtime behavior. User reviews this spec before the implementation plan.

## Objective

- **Who:** Maintainers and agents working from `D:\rust\mao_agent` / GitHub clone.
- **Why:** After the Axum API push, `tasks/plan.md` checkpoints are stale vs `todo.md`, `AGENT_INSTRUCTIONS.md` still shows offline `dimension: 1536`, probe `logs/` dirty the tree, and a local `nul` stub may remain.
- **Success:** Docs and ignore rules match current code; `cargo test --no-default-features` still reports **56** passed; no feature or API changes.

## In scope

1. **`tasks/plan.md`** — Check off completed Foundation / Complete checkpoints so they match `tasks/todo.md`. Do not rewrite task bodies.
2. **`AGENT_INSTRUCTIONS.md`** — Surgical fixes only:
   - Offline `EmbedderSelection` example: when `offline: true`, set `dimension: 512` (not `1536`).
   - Ingest / self-heal commands that use `--no-default-features` must include `--offline` (deterministic embedder path).
3. **`.gitignore`** — Add `logs/` so serve/probe logs stop dirtying the working tree.
4. **`README.md`** — Consistency check only. Expected **no change** (already documents 56 tests and offline 512-dim). Fix only if leftover drift is found.
5. **Local `nul`** — Delete the empty file if present (already gitignored; do not commit it).
6. **`tasks/todo.md`** — Minor wording alignment with plan if needed; no new tasks.

## Out of scope

Source logic, CI, API/server behavior, robustness gaps (unknown `--mode`, BM25 fail-open, citation thresholds), new features, full `AGENT_INSTRUCTIONS.md` rewrite.

## Delivery

- Single implementation commit (after this spec is approved and an implementation plan is written).
- Suggested commit message: `docs: align hygiene with post-API push (plan, agent instructions, ignore logs)`
- Open a PR to `main`; user reviews on GitHub.

## Acceptance (implementation)

- [ ] `cargo test --no-default-features` → 56 passed
- [ ] `git status` has no unexpected dirty files after cleanup
- [ ] `AGENT_INSTRUCTIONS.md` offline example uses `dimension: 512`
- [ ] `--no-default-features` ingest/self-heal snippets include `--offline`
- [ ] `.gitignore` contains `logs/`
- [ ] `tasks/plan.md` checkpoints match completed `todo.md`

## Boundaries

- **Always:** Keep tests green; docs-only / ignore-only diffs.
- **Ask first:** Any source or CI change; expanding into robustness fixes.
- **Never:** Commit secrets, edit `data/` artifacts, silent embedder fallbacks.

## Open questions

None — design §§1–2 approved 2026-09-05.
