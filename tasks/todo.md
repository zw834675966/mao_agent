# Tasks: Cycle 15 — Loop Engineering Architecture Alignment

Plan document: `tasks/plan.md`.
Corpus & Architecture context: `README.md`, `AGENTS.md`.

---

## Phase 1: Hard Safety Gate & Machine Enforcement

### Task 15-1: Define Machine-Readable `gate.yaml` Policy

**Description:** Create `gate.yaml` at project root defining machine-enforced path denylists (`config.toml`, `.env*`, `data/*.bin`, `data/tantivy_index/**`, `corpus/**/raw/**`, `raw/**`, `*.embedcache`, `.fastembed_cache/**`), `maxFiles: 8`, and `autoMergeAllowlist` for documentation files.

**Acceptance criteria:**
- [x] `gate.yaml` created with `version: 1`.
- [x] `denylist` covers all 8 sensitive and derived asset path patterns.
- [x] `maxFiles` set to 8.
- [x] `autoMergeAllowlist` defined for markdown and doc files.

**Verification:**
- [x] `python -c "import yaml; d=yaml.safe_load(open('gate.yaml')); assert d['version']==1 and len(d['denylist'])>=8"`

**Dependencies:** None

**Files likely touched:**
- `gate.yaml`

**Estimated scope:** XS (1 file)

---

### Task 15-2: Implement Standalone `scripts/gate_check.py` CLI Tool

**Description:** Implement `scripts/gate_check.py` (Python 3 stdlib only) strictly conforming to the upstream `loop-gate` CLI contract: supports `check --action <commit|tool|auto-merge> --paths <p1,p2,...> [--gate-file gate.yaml] [--json]`. Exit codes: 0 = ALLOWED, 2 = ESCALATE, 1 = ERROR.

**Acceptance criteria:**
- [x] `scripts/gate_check.py` created with stdlib only (`pathlib`, `fnmatch`, `argparse`, `json`).
- [x] Denylist match immediately exits with code 2.
- [x] Paths count exceeding `maxFiles` exits with code 2.
- [x] Valid paths exit with code 0.

**Verification:**
- [x] `python scripts/gate_check.py check --action tool --paths "config.toml"` returns exit code 2
- [x] `python scripts/gate_check.py check --action tool --paths "src/lib.rs"` returns exit code 0
- [x] `python scripts/gate_check.py check --action tool --paths "corpus/papers_we_love/raw/test.pdf"` returns exit code 2

**Dependencies:** Task 15-1

**Files likely touched:**
- `scripts/gate_check.py`

**Estimated scope:** XS (1 file)

---

### Task 15-3: Connect Gate Check to `scripts/hook_guard.py` PreToolUse

**Description:** Refactor `scripts/hook_guard.py` so that when `handle_pre_tool` intercepts file modification tools (`write_to_file`, `replace_file_content`), it calls `gate_check.check_gate(...)` against `gate.yaml` to enforce mechanical protection, eliminating scattered hardcoded path strings.

**Acceptance criteria:**
- [x] `scripts/hook_guard.py` imports and integrates `gate_check`.
- [x] Denied paths in `gate.yaml` are blocked with explicit security rejection messages.

**Verification:**
- [x] Hook guard test command or manual invocation passes and blocks denied paths.

**Dependencies:** Task 15-2

**Files likely touched:**
- `scripts/hook_guard.py`

**Estimated scope:** XS (1 file)

---

### Checkpoint 15-1: Safety Gate Pipeline Verified
- [x] `gate.yaml` policy valid and loaded
- [x] `scripts/gate_check.py` passes all exit code tests (0/2)
- [x] `scripts/hook_guard.py` integrates with `gate.yaml`

---

## Phase 2: Durable State Machine & Cost Governance

### Task 15-4: Establish Operational `STATE.md` Schema and Baseline Ledger

**Description:** Create `STATE.md` at root as the active operational state spine. Records corpus document counts, active vector/tantivy/graph store snapshots, baseline retrieval metrics (Hybrid Recall@5=1.000, MRR@5=0.995), High Priority queue, Watch List, and Recent Noise.

**Acceptance criteria:**
- [x] `STATE.md` contains ISO-8601 `Last run` timestamp and mode.
- [x] Contains structured tables for `Corpus & Index State Snapshot` and `Retrieval Baseline Gate`.
- [x] Contains `## High Priority`, `## Watch List`, and `## Recent Noise` sections.

**Verification:**
- [ ] File inspection: confirms all required markdown headings and valid initial data.

**Dependencies:** Checkpoint 15-1

**Files likely touched:**
- `STATE.md`

**Estimated scope:** XS (1 file)

---

### Task 15-5: Establish Multi-Model Quotas in `loop-budget.md`

**Description:** Create `loop-budget.md` defining daily request/token ceilings for SiliconFlow, Cohere Chat, Cohere Rerank, and Gemini. Specifies the automatic fallback protocol to `--offline --no-rerank` at 80% quota, and defines the `MAO_LOOP_PAUSE` kill switch.

**Acceptance criteria:**
- [x] `loop-budget.md` contains daily quota table with fallback behaviors.
- [x] Defines protocol for handling quota exhaustion.
- [x] Defines kill switch mechanics.

**Verification:**
- [ ] File inspection: confirms table consistency with actual CLI flags.

**Dependencies:** Task 15-4

**Files likely touched:**
- `loop-budget.md`

**Estimated scope:** XS (1 file)

---

### Task 15-6: Establish Agent Constraints in `loop-constraints.md`

**Description:** Create `loop-constraints.md` declaring immutable engineering invariants for autonomous agents: test deletion prohibitions, binary artifacts immutability (`data/*.bin`), and max 3-attempt automated fix budget before human escalation.

**Acceptance criteria:**
- [x] Declares Testing Redlines (never suppress or delete tests to pass CI).
- [x] Declares Artifact Invariants (never hand-edit `data/*.bin`).
- [x] Declares Attempt Budget Cap (max 3 tries).

**Verification:**
- [ ] File inspection: confirms all 3 invariant domains are documented.

**Dependencies:** Task 15-5

**Files likely touched:**
- `loop-constraints.md`

**Estimated scope:** XS (1 file)

---

### Checkpoint 15-2: State & Budget Framework Verified
- [x] `STATE.md` created with verified baseline numbers
- [x] `loop-budget.md` and `loop-constraints.md` documented and verified

---

## Phase 3: First Autonomous L1 Loop & Project Runbook Sync

### Task 15-7: Implement L1 Corpus & Index Integrity Inspector in `scripts/loop_l1_inspect.py`

**Description:** Create `scripts/loop_l1_inspect.py` (Python 3 stdlib only) implementing Gate 1 (Cargo compile & clippy check), Gate 2 (Corpus YAML frontmatter scan & raw/ directory zero-pollution assertion), and Gate 3 (Index artifacts existence and size checks). Supports `--check-integrity`.

**Acceptance criteria:**
- [x] `scripts/loop_l1_inspect.py` created with stdlib only.
- [x] Scans all clean corpus markdown documents and validates frontmatter fields.
- [x] Asserts `raw/` files are not treated as active corpus documents.
- [x] Returns exit code 0 on health, 1 on anomaly.

**Verification:**
- [x] `python scripts/loop_l1_inspect.py --check-integrity` succeeds with exit code 0

**Dependencies:** Checkpoint 15-2

**Files likely touched:**
- `scripts/loop_l1_inspect.py`

**Estimated scope:** XS (1 file)

---

### Task 15-8: Implement L1 Baseline Retrieval Gate & `STATE.md` Sync

**Description:** Extend `scripts/loop_l1_inspect.py` to implement Gate 4 (runs `cargo run --no-default-features -- eval-retrieval --k 5 --mode hybrid --no-rerank --offline --json` and asserts `recall_at_k >= 0.99`), Gate 5 (hard-negative defense test), and Gate 6 (atomically writes updated metrics and timestamp to `STATE.md`). Supports `--full`.

**Acceptance criteria:**
- [x] `--full` runs all 6 gates end-to-end.
- [x] Summary JSON is parsed and `recall_at_k >= 0.99` is enforced.
- [x] Any failing query is extracted into `STATE.md` High Priority list.
- [x] `STATE.md` is updated atomically via tempfile replacement.

**Verification:**
- [x] `python scripts/loop_l1_inspect.py --full` succeeds and updates `STATE.md`

**Dependencies:** Task 15-7

**Files likely touched:**
- `scripts/loop_l1_inspect.py`

**Estimated scope:** XS (1 file)

---

### Task 15-9: Update Runbook and AGENTS.md with Loop Engineering Protocol

**Description:** Synchronize `docs/ops/runbook.md` and `AGENTS.md` to introduce the Loop Engineering operational procedures: running `gate_check.py`, reading/updating `STATE.md`, consulting `loop-budget.md`, and triggering `loop_l1_inspect.py`.

**Acceptance criteria:**
- [x] `docs/ops/runbook.md` includes Section 7 on Loop Engineering.
- [x] `AGENTS.md` references `gate.yaml`, `STATE.md`, and `loop-budget.md`.

**Verification:**
- [x] `cargo fmt --check`
- [x] Git diff check: `git diff docs/ops/runbook.md AGENTS.md`

**Dependencies:** Task 15-8

**Files likely touched:**
- `docs/ops/runbook.md`
- `AGENTS.md`

**Estimated scope:** S (2 files)

---

## Checkpoint 15-3: Full Gate & L1 Loop Delivery
- [x] `python scripts/loop_l1_inspect.py --full` passes all 6 gates
- [x] `python scripts/gate_check.py check --action commit --paths "src/lib.rs"` passes
- [x] `cargo fmt --check`
- [x] `cargo clippy --no-default-features --all-targets -- -D warnings`
- [x] `cargo test --no-default-features` (all 212 tests pass)

---

## Historical Notes (Cycle 14 Archived)
- Cycle 14 delivered Pi Agent contract integration (`tests/mcp_test.rs`), verification scripts (`scripts/verify_pi_*.py`), and archived DSH with `[DEPRECATED]` banners. All 9 tasks (14-1 to 14-9) completed and forensically approved.
