# Tasks: Cycle 14 — Pi Agent Contract Tests & DSH Advisory Decoupling

Plan document: `tasks/plan.md`.  
Human approved; Cycle 14 implemented.

---

## Phase 1: Hermetic Pi contract tests

### Task 14-1: Add Pi Agent handshake contract test

**Description:** Add `test_mcp_pi_agent_handshake` in `tests/mcp_test.rs`. Verify MCP `initialize` with Pi `clientInfo` (`name: "pi-coding-agent"`, `version: "0.1.0"`) and that `notifications/initialized` returns no response. Do not call tools. Do not rename existing `dsh-agent` fixtures. Server remains client-agnostic.

**Acceptance criteria:**
- [x] `tests/mcp_test.rs` contains `test_mcp_pi_agent_handshake`.
- [x] Initialize with Pi `clientInfo` asserts `protocolVersion` `2024-11-05` and `serverInfo.name` `mao_agent`.
- [x] `notifications/initialized` produces no JSON-RPC response.

**Verification:**
- [x] `cargo test --no-default-features --test mcp_test test_mcp_pi_agent_handshake`

**Dependencies:** None

**Files likely touched:**
- `tests/mcp_test.rs`

**Estimated scope:** XS (1 file)

---

### Task 14-2: Add Pi Agent query_dialectical_principles session test

**Description:** Add `test_mcp_pi_agent_query_dialectical_principles`. After Pi initialize + initialized, `tools/call` `query_dialectical_principles` with `synthesize: false`. Assert JSON key `principles` (array) and `doc_title`. Do not assert `triads`.

**Acceptance criteria:**
- [x] `tests/mcp_test.rs` contains `test_mcp_pi_agent_query_dialectical_principles`.
- [x] Call uses `synthesize: false` (or omitted) and stays offline.
- [x] Parsed text JSON has non-empty `principles`; no `triads` requirement.

**Verification:**
- [x] `cargo test --no-default-features --test mcp_test test_mcp_pi_agent_query_dialectical_principles`

**Dependencies:** Task 14-1 (same file)

**Files likely touched:**
- `tests/mcp_test.rs`

**Estimated scope:** XS (1 file)

---

### Task 14-3: Add Pi Agent verify_historical_citation session test

**Description:** Add `test_mcp_pi_agent_verify_citation`. After Pi initialize, call `verify_historical_citation` with `quote` + `claimed_title`, omit `context_chunks`. Assert `confidence` and `verdict`. Title lookup only; caller chunks are not grounding.

**Acceptance criteria:**
- [x] `tests/mcp_test.rs` contains `test_mcp_pi_agent_verify_citation`.
- [x] `context_chunks` omitted; `claimed_title` present.
- [x] Response JSON includes `confidence` and `verdict`.

**Verification:**
- [x] `cargo test --no-default-features --test mcp_test test_mcp_pi_agent_verify_citation`

**Dependencies:** Task 14-2 (same file)

**Files likely touched:**
- `tests/mcp_test.rs`

**Estimated scope:** XS (1 file)

---

## Checkpoint 14-1: Protocol tests green
- [x] `cargo test --no-default-features --test mcp_test`
- [x] No runtime edits in `src/mcp/dispatcher.rs` or `src/mcp/stdio.rs` from Phase 1

---

## Phase 2: Transport docs and ops probes

### Task 14-4: Align stdio module docs with Pi Agent

**Description:** Update `src/mcp/stdio.rs` module docstring: primary documented client is `@earendil-works/pi-coding-agent` via `pi-mcp-adapter`. Do not change runtime. Do not edit `dispatcher.rs` (no DSH string there).

**Acceptance criteria:**
- [x] Header no longer presents `@deepseek-ai/dsh-mcp-client` as the primary client.
- [x] Header names Pi / `pi-mcp-adapter`.
- [x] No runtime or unit-test logic changes in this task.

**Verification:**
- [x] `cargo clippy --no-default-features --lib -- -D warnings`

**Dependencies:** None (parallel with Phase 1)

**Files likely touched:**
- `src/mcp/stdio.rs`

**Estimated scope:** XS (1 file)

---

### Task 14-5: Implement Pi stdio probe script

**Description:** Create `scripts/verify_pi_stdio.py` (Python 3 stdlib). Spawn `cargo run --no-default-features -- mcp --offline` (debug, not `--release`). Drive initialize (Pi clientInfo) → initialized → tools/list → query_dialectical_principles (`synthesize: false`). Child traces on stderr. Exit 0 on valid frames. Optional `--bin`. Not a CI job.

**Acceptance criteria:**
- [x] Script exists; stdlib only.
- [x] Default spawn is not `--release`.
- [x] Exit 0 on valid initialize + tools/list frames.
- [x] Not added to `.github/workflows/ci.yml`.

**Verification:**
- [x] `python scripts/verify_pi_stdio.py` (ops probe; first run may compile)

**Dependencies:** None

**Files likely touched:**
- `scripts/verify_pi_stdio.py`

**Estimated scope:** XS (1 file)

---

### Task 14-6: Implement Pi HTTP probe script

**Description:** Create `scripts/verify_pi_http.py` (Python 3 stdlib) with `--mock` and `--url`. `--mock` validates Pi-shaped JSON-RPC POST bodies for `/api/v1/mcp` with no network.

**Acceptance criteria:**
- [x] Supports `--mock` and `--url`.
- [x] `--mock` exits 0 with no network.
- [x] Documents `POST /api/v1/mcp`.

**Verification:**
- [x] `python scripts/verify_pi_http.py --mock`

**Dependencies:** None

**Files likely touched:**
- `scripts/verify_pi_http.py`

**Estimated scope:** XS (1 file)

---

## Checkpoint 14-2: Docs + hermetic HTTP probe
- [x] `python scripts/verify_pi_http.py --mock`
- [x] `cargo clippy --no-default-features --all-targets -- -D warnings`
- [x] Stdio probe run when a debug binary can be spawned (not a CI blocker)

---

## Phase 3: SRE retarget, DSH advisory deprecation, index sync

### Task 14-7: Retarget MCP SRE guide to Pi Agent

**Description:** Update `docs/ops/mcp_sre_guide.md` primary runtime to Pi Agent + `pi-mcp-adapter`. Keep Draft-7 / metrics sections. Link `docs/pi/README.md` and `docs/pi/mcp.json`. Retarget, do not rewrite.

**Acceptance criteria:**
- [x] Target-runtime blurb lists Pi first.
- [x] Troubleshooting covers `pi-mcp-adapter` / stdout vs stderr framing.
- [x] Links to `docs/pi/` resolve.

**Verification:**
- [x] Manual markdown link check to `docs/pi/README.md` and `docs/pi/mcp.json`

**Dependencies:** None (complete before 14-9)

**Files likely touched:**
- `docs/ops/mcp_sre_guide.md`

**Estimated scope:** XS (1 file)

---

### Task 14-8: Add deprecation notices to DSH artifacts

**Description:** Prepend `[DEPRECATED]` banners to the two `docs/dsh/` files; redirect to `docs/pi/`. Do not delete.

**Acceptance criteria:**
- [x] `docs/dsh/cordis.patch.example.yml` header redirects to `docs/pi/mcp.json`.
- [x] `docs/dsh/dialectical_counselor_persona.md` header redirects to `docs/pi/dialectical_counselor_persona.md`.
- [x] Both files still present.

**Verification:**
- [x] `git diff docs/dsh/cordis.patch.example.yml docs/dsh/dialectical_counselor_persona.md`

**Dependencies:** Task 14-7

**Files likely touched:**
- `docs/dsh/cordis.patch.example.yml`
- `docs/dsh/dialectical_counselor_persona.md`

**Estimated scope:** S (2 files)

---

### Task 14-9: Update runbook and AGENTS.md client index

**Description:** Sync `docs/ops/runbook.md` §6 and `AGENTS.md` so Pi (`docs/pi/README.md`) is the primary documented client mount and DSH is archived/advisory. No CLI/runtime change.

**Acceptance criteria:**
- [x] Runbook MCP section links `docs/pi/README.md`.
- [x] `AGENTS.md` lists Pi as primary mount; DSH as archived/deprecated path.

**Verification:**
- [x] `git diff docs/ops/runbook.md AGENTS.md` (not `cargo fmt --check`)

**Dependencies:** Task 14-8

**Files likely touched:**
- `docs/ops/runbook.md`
- `AGENTS.md`

**Estimated scope:** S (2 files)

---

## Checkpoint 14-3: Full gate
- [x] `cargo fmt --check`
- [x] `cargo clippy --no-default-features --all-targets -- -D warnings`
- [x] `cargo test --no-default-features`
- [x] DSH files remain with banners; Pi docs not recreated

---

## Constraints (must)

- [x] C1–C12 in `tasks/plan.md` still hold at Checkpoint 14-3
- [x] No new MCP tools; no `clientInfo` branch in dispatcher
- [x] Probe scripts not added to GitHub Actions
