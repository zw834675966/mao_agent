# Implementation Plan: Cycle 14 — Pi Agent Contract Tests & DSH Advisory Decoupling

**Status:** Implemented (Cycle 14). Human approved; tasks 14-1…14-9 landed.  
**Task list:** `tasks/todo.md`  
**Replan note:** Same Cycle 14 work as the previous `tasks/plan.md` / `tasks/todo.md` (all boxes unchecked). This file replaces that draft in place after a code-grounded granularity audit. It does not start a different project.

## Overview

Cycle 14 does **not** build a Pi Agent ecosystem from scratch. `docs/pi/` already ships README, `mcp.json`, persona, and the `dialectical-counselor` skill. The MCP server is already JSON-RPC 2.0 / MCP `2024-11-05`, transport-agnostic, and **client-agnostic**: `McpDispatcher::handle_request` ignores `clientInfo` and returns a fixed `initialize` payload (`src/mcp/dispatcher.rs`).

This cycle locks the **Pi adapter contract** (payload + session sequence), adds **stdlib ops probes** that match `docs/pi/mcp.json`, **retargets SRE docs** to Pi / `pi-mcp-adapter`, and **advises** deprecation of `docs/dsh/` (banner + redirect, no delete). No new MCP tools. No dispatcher runtime branch.

## Architecture Decisions

- **Client-agnostic server stays.** Do not add `if client == pi` in dispatcher. Pi tests are fixture/contract tests (`clientInfo.name = "pi-coding-agent"`), not a new code path.
- **Reuse the existing hermetic fixture.** New tests in `tests/mcp_test.rs` call `setup_test_context()` / `McpDispatcher::new`. No network. `--no-default-features`.
- **Keep 14-1 / 14-2 / 14-3 as three functions.** Failure localizes to one protocol frame. They are sequential only because they share one file (merge-conflict), not because the protocol requires it. Existing DSH-named fixtures stay; do not rename them in this cycle.
- **14-2/14-3 add a Pi session sequence**, not a duplicate of the existing tool tests: `initialize` (Pi clientInfo) → `notifications/initialized` → `tools/call`. Existing tests call `tools/call` without a prior handshake.
- **14-4 is `src/mcp/stdio.rs` only.** `dispatcher.rs` has no DSH string. Do not invent a dispatcher edit.
- **Probe scripts are ops tools, not CI gates.** Default stdio probe uses debug `cargo run --no-default-features -- mcp --offline`, not `--release`. HTTP probe `--mock` is the hermetic check (no live `serve`).
- **DSH deprecation is advisory.** Replacement already exists under `docs/pi/`. Do not `rm` `docs/dsh/`.
- **No new ADR.** This is docs + contract tests, not a runtime architecture change. Existing ADRs under `docs/adr/` stay untouched.

## Constraints Checklist (12/12)

| ID | Constraint | must |
| :--- | :--- | :---: |
| C1 | Keep task IDs 14-1 … 14-9 (split for failure localization). | yes |
| C2 | Do not add a Pi runtime branch; `initialize` ignores `clientInfo`. | yes |
| C3 | `query_dialectical_principles` JSON key is `principles` (not `triads`). Hits may include `graph_paths`. | yes |
| C4 | `verify_historical_citation` grounds on corpus title lookup only; caller `context_chunks` is not grounding. | yes |
| C5 | Do not recreate `docs/pi/` (README, `mcp.json`, persona, skill already exist). | yes |
| C6 | 14-1/2/3 sequential only because they share `tests/mcp_test.rs`. | yes |
| C7 | 14-4 touches `src/mcp/stdio.rs` only (XS). No forced `dispatcher.rs` edit. | yes |
| C8 | 14-5: Python 3 stdlib; default **not** `--release`; ops probe, not CI. Tracing stays on stderr of the child. | yes |
| C9 | 14-6: `--mock` is the hermetic verification (no live server). | yes |
| C10 | 14-9 verification is content/`git diff`, not `cargo fmt --check`. | yes |
| C11 | Advisory `[DEPRECATED]` on `docs/dsh/` files; do not delete. | yes |
| C12 | No new MCP tools; no dispatcher/stdio runtime behavior change; tests stay `--no-default-features` and offline. | yes |

## 最小颗粒度核查矩阵 (Granularity Audit Matrix)

| Task ID | 任务名称 | 影响文件 | 颗粒度 | 不可再分理由 |
| :--- | :--- | :--- | :---: | :--- |
| **14-1** | Pi Agent 握手契约测试 | 1 (`tests/mcp_test.rs`) | **XS** | 单函数：仅 `initialize` + `notifications/initialized`，不调用工具 |
| **14-2** | Pi Agent 辩证原则会话测试 | 1 (`tests/mcp_test.rs`) | **XS** | 单函数：握手后 `tools/call query_dialectical_principles`，断言 `principles` |
| **14-3** | Pi Agent 引用校验会话测试 | 1 (`tests/mcp_test.rs`) | **XS** | 单函数：握手后 `tools/call verify_historical_citation`，标题反查 |
| **14-4** | stdio 模块文档对齐 Pi | 1 (`src/mcp/stdio.rs`) | **XS** | 仅模块 docstring；不改运行代码。**审计后从 2 文件缩为 1 文件** |
| **14-5** | Pi Stdio 独立探测脚本 | 1 (`scripts/verify_pi_stdio.py`) | **XS** | 单脚本、stdlib、子进程帧流动 |
| **14-6** | Pi HTTP 独立探测脚本 | 1 (`scripts/verify_pi_http.py`) | **XS** | 单脚本、stdlib、`--mock` 校验 POST 形状 |
| **14-7** | 重定向 MCP SRE 指南 | 1 (`docs/ops/mcp_sre_guide.md`) | **XS** | 单文档：主运行时改为 Pi；不重写指标章节 |
| **14-8** | DSH 软废弃标记 | 2 (`docs/dsh/cordis.patch.example.yml`, `docs/dsh/dialectical_counselor_persona.md`) | **S** | 两文件头部 banner + 重定向，不删文件 |
| **14-9** | Runbook 与 AGENTS.md 索引 | 2 (`docs/ops/runbook.md`, `AGENTS.md`) | **S** | 同一决策的两处入口索引同步 |

## 颗粒度自审（4 轮）

### Round 1–3（既有草稿）

将“握手 + 工具调用”拆成 14-1/14-2/14-3；探测脚本用 Python 标准库；DSH 软废弃而非删除。

### Round 4（本轮，对照代码）

1. **复合任务 / 幽灵文件：** 草稿 14-4 把 `dispatcher.rs` 算进去，但该文件无 DSH 字符串（`src/mcp/dispatcher.rs` 模块文档是 transport-agnostic）。缩为 `stdio.rs` 单文件。
2. **过度设计：** 草稿把 Pi 写成新运行时分支。服务器不读 `clientInfo`。测试只锁定 Pi 适配器载荷与会话顺序。
3. **验收不确定性：**
   - 14-2 不得断言不存在的 `triads` 键（真实键：`principles`，见 `src/mcp/tools/principles.rs`）。
   - 14-3 不得写成“用 caller chunks 做原典反查”（与 `AGENTS.md` / `citation.rs` 相反）。
   - 14-5 默认 `--release` 过慢，且不是 CI  hermetic 门禁。
   - 14-9 用 `cargo fmt --check` 验收 Markdown 无效。
4. **假依赖：** 14-4 不依赖 14-3；14-5 不依赖 14-4；14-6 不依赖 14-5。Phase 2/3 文档与脚本可与测试链并行（除 14-9 应在 14-7/14-8 之后，以免索引与废弃标记不一致）。
5. **YAGNI 备选（未采纳）：** 只读架构席建议把 14-1/14-2/14-3 合并为一个 Pi `clientInfo` 测试。本计划 **保留三函数拆分**（用户要求失败定位到单帧）。若实现时发现三函数除 `clientInfo` 外与现有测试完全重复、且不包含握手会话，允许在 14-1 实现时把 14-2/14-3 收成同一文件内的薄会话包装，但仍保持三个 `#[tokio::test]` 名称以便单测过滤。

## 依赖图谱

```
Phase 1 (same file: tests/mcp_test.rs)
  14-1 ──► 14-2 ──► 14-3 ──► Checkpoint 14-1
                               │
Phase 2 (independent of each other; independent of 14-4↔14-6)
  14-4 stdio docstring          ─┐
  14-5 verify_pi_stdio.py       ─┼──► Checkpoint 14-2
  14-6 verify_pi_http.py --mock ─┘

Phase 3
  14-7 SRE retarget ──► 14-8 DSH banners ──► 14-9 index sync ──► Checkpoint 14-3
  (14-7 may start in parallel with Phase 2)
```

## 任务拆解详情

### Phase 1: Hermetic Pi contract tests

#### Task 14-1: Pi Agent handshake contract test

**Description:** Add `test_mcp_pi_agent_handshake` in `tests/mcp_test.rs`. Send MCP `initialize` with Pi adapter `clientInfo` (`name: "pi-coding-agent"`, `version: "0.1.0"`) and `protocolVersion: "2024-11-05"`. Assert success payload. Then send `notifications/initialized` and assert the dispatcher returns `None` (notification). Do not call tools. Do not rename existing `dsh-agent` fixtures.

**Acceptance criteria:**

- `tests/mcp_test.rs` contains `test_mcp_pi_agent_handshake`.
- Initialize with Pi `clientInfo` yields `protocolVersion == "2024-11-05"` and `serverInfo.name == "mao_agent"`.
- `notifications/initialized` produces no JSON-RPC response.

**Verification:** `cargo test --no-default-features --test mcp_test test_mcp_pi_agent_handshake`

**Dependencies:** None  
**Files likely touched:** `tests/mcp_test.rs`  
**Estimated scope:** XS (1 file)

#### Task 14-2: Pi Agent query_dialectical_principles session test

**Description:** Add `test_mcp_pi_agent_query_dialectical_principles`. After the same Pi initialize (+ initialized), `tools/call` `query_dialectical_principles` with `query` set, `synthesize: false`. Parse `content[0].text` JSON. Assert `principles` is a non-empty array and `principles[0].doc_title` is present (fixture expects `矛盾论` for query `矛盾的法则与转化`). Do not assert a `triads` key.

**Acceptance criteria:**

- Test function exists and does not mix citation verification.
- Returned text JSON has `principles` (array) and no requirement on `triads`.
- `synthesize` is false / omitted so the test stays offline.

**Verification:** `cargo test --no-default-features --test mcp_test test_mcp_pi_agent_query_dialectical_principles`

**Dependencies:** Task 14-1 (same file)  
**Files likely touched:** `tests/mcp_test.rs`  
**Estimated scope:** XS (1 file)

#### Task 14-3: Pi Agent verify_historical_citation session test

**Description:** Add `test_mcp_pi_agent_verify_citation`. After Pi initialize, call `verify_historical_citation` with a grounded quote and `claimed_title: "矛盾论"`. Omit `context_chunks` (title lookup / auto-retrieve). Assert `confidence` is a number and `verdict` is `ExactMatch` (or `is_valid == true`). Do not treat caller chunks as grounding.

**Acceptance criteria:**

- Test function exists.
- Arguments include `quote` + `claimed_title`; `context_chunks` omitted.
- Response JSON includes `confidence` and `verdict`.

**Verification:** `cargo test --no-default-features --test mcp_test test_mcp_pi_agent_verify_citation`

**Dependencies:** Task 14-2 (same file)  
**Files likely touched:** `tests/mcp_test.rs`  
**Estimated scope:** XS (1 file)

### Checkpoint 14-1

- `cargo test --no-default-features --test mcp_test` (full file, including pre-existing tests)
- No dispatcher/stdio runtime edits in this phase

### Phase 2: Transport docs and ops probes

#### Task 14-4: Align stdio module docs with Pi Agent

**Description:** Update the module docstring in `src/mcp/stdio.rs` so the documented stdio client is `@earendil-works/pi-coding-agent` via `pi-mcp-adapter`, while remaining MCP 2024-11-05 / line-delimited JSON-RPC. Do not change `run_stdio_loop` or tests. Do not edit `dispatcher.rs`.

**Acceptance criteria:**

- `src/mcp/stdio.rs` header no longer presents `@deepseek-ai/dsh-mcp-client` as the primary client.
- Header names Pi / `pi-mcp-adapter` (and may still say the framing is generic MCP stdio).
- Zero runtime/test logic changes in this task.

**Verification:** `cargo clippy --no-default-features --lib -- -D warnings`

**Dependencies:** None (may run parallel with Phase 1)  
**Files likely touched:** `src/mcp/stdio.rs`  
**Estimated scope:** XS (1 file)

#### Task 14-5: Pi stdio probe script

**Description:** Add `scripts/verify_pi_stdio.py` (Python 3, stdlib only: `subprocess`, `json`, `sys`, `pathlib`, `argparse`). Spawn `cargo run --no-default-features -- mcp --offline` (debug, not `--release`). Drive line-delimited JSON-RPC: `initialize` (Pi clientInfo) → `notifications/initialized` → `tools/list` → `query_dialectical_principles` with `synthesize: false`. Child diagnostics must remain on stderr; script diagnostics on stderr. Exit 0 on valid frames. Optional `--bin` to reuse an already-built binary.

**Acceptance criteria:**

- File exists; no third-party imports.
- Default command is not `--release`.
- Exit code 0 when initialize + tools/list frames parse as JSON-RPC success.
- Not added to `.github/workflows/ci.yml`.

**Verification:** `python scripts/verify_pi_stdio.py` (first run may compile; not a CI gate)

**Dependencies:** None  
**Files likely touched:** `scripts/verify_pi_stdio.py`  
**Estimated scope:** XS (1 file)

#### Task 14-6: Pi HTTP probe script

**Description:** Add `scripts/verify_pi_http.py` (Python 3 stdlib). `--mock` (default for verification) checks that a Pi-shaped `POST` JSON-RPC body for `initialize` / `tools/list` matches the documented `/api/v1/mcp` contract (jsonrpc 2.0, method, clientInfo). `--url` optionally POSTs to a live server; not required for the task gate.

**Acceptance criteria:**

- Supports `--mock` and `--url`.
- `--mock` exits 0 with no network.
- Documents `POST /api/v1/mcp` (and may mention `/mcp` alias).

**Verification:** `python scripts/verify_pi_http.py --mock`

**Dependencies:** None  
**Files likely touched:** `scripts/verify_pi_http.py`  
**Estimated scope:** XS (1 file)

### Checkpoint 14-2

- `python scripts/verify_pi_http.py --mock`
- `cargo clippy --no-default-features --all-targets -- -D warnings`
- Stdio probe: run when a debug binary can be spawned; do not block CI

### Phase 3: SRE retarget, DSH advisory deprecation, index sync

#### Task 14-7: Retarget MCP SRE guide to Pi Agent

**Description:** Update `docs/ops/mcp_sre_guide.md` so the primary target runtime is Pi Coding Agent (`@earendil-works/pi-coding-agent`) and `pi-mcp-adapter`. Keep schema Draft-7 / OpenCode notes (still true). Do not rewrite metrics/alerting sections. Point mount docs at `docs/pi/README.md` and `docs/pi/mcp.json`.

**Acceptance criteria:**

- Title/target-runtime blurb lists Pi first.
- Troubleshooting includes `pi-mcp-adapter` / stdio framing (stdout vs stderr).
- Unrelated Prometheus examples left intact unless they name DSH as the only client.

**Verification:** Manual: links to `docs/pi/README.md` and `docs/pi/mcp.json` resolve; no broken relative paths.

**Dependencies:** None (before 14-9)  
**Files likely touched:** `docs/ops/mcp_sre_guide.md`  
**Estimated scope:** XS (1 file)

#### Task 14-8: Advisory deprecation banners on DSH artifacts

**Description:** Prepend `[DEPRECATED]` banners to `docs/dsh/cordis.patch.example.yml` and `docs/dsh/dialectical_counselor_persona.md`. Redirect to `docs/pi/mcp.json` and `docs/pi/dialectical_counselor_persona.md` respectively. Keep file bodies so old DSH readers can still copy-paste.

**Acceptance criteria:**

- YAML file: comment banner at top, pointer to `docs/pi/mcp.json`.
- Persona file: markdown banner at top, pointer to `docs/pi/dialectical_counselor_persona.md`.
- Files not deleted.

**Verification:** `git diff docs/dsh/cordis.patch.example.yml docs/dsh/dialectical_counselor_persona.md` shows header-only (or header-plus-banner) change.

**Dependencies:** Task 14-7 (narrative: replacement docs already exist; banner after SRE retarget)  
**Files likely touched:** `docs/dsh/cordis.patch.example.yml`, `docs/dsh/dialectical_counselor_persona.md`  
**Estimated scope:** S (2 files)

#### Task 14-9: Sync runbook and AGENTS.md client index

**Description:** Point `docs/ops/runbook.md` §6 MCP at `docs/pi/README.md` as the primary client mount. Update `AGENTS.md` serve/mcp bullet: Pi (`docs/pi/README.md`) is the documented first-party client; DSH (`docs/dsh/…`) is archived/advisory. Do not change CLI flag semantics.

**Acceptance criteria:**

- Runbook MCP section links `docs/pi/README.md`.
- `AGENTS.md` lists Pi as primary mount and DSH as archived/deprecated path.
- No runtime/code change.

**Verification:** `git diff docs/ops/runbook.md AGENTS.md` (not `cargo fmt --check`)

**Dependencies:** Task 14-8  
**Files likely touched:** `docs/ops/runbook.md`, `AGENTS.md`  
**Estimated scope:** S (2 files)

### Checkpoint 14-3 (full gate)

- `cargo fmt --check`
- `cargo clippy --no-default-features --all-targets -- -D warnings`
- `cargo test --no-default-features`
- Human skim: DSH files still present with banners; Pi docs unchanged except as referenced

## Parallelization

| Lane | Tasks | Model / seat when implementing |
| :--- | :--- | :--- |
| A (same file, sequential) | 14-1 → 14-2 → 14-3 | Main or one `builder` |
| B (docs, XS) | 14-4, 14-7 | Lightweight third-party `general-purpose` / main |
| C (scripts) | 14-5, 14-6 | Lightweight third-party; no new deps |
| D (after 14-7) | 14-8 → 14-9 | Main (AGENTS.md is project SSOT) |

Do not parallelize 14-1/14-2/14-3 across two writers.

## Risks and Mitigations

| Risk | Impact | Mitigation |
| :--- | :--- | :--- |
| Pi tests become clones of existing MCP tests | Low | Require session sequence (initialize → initialized → tools/call); keep three filterable test names |
| Stdio probe first-run compile time / Windows exe lock | Med | Default debug build; `--bin` override; document `os error 5` using existing SRE note |
| `--mock` HTTP script tests nothing real | Low | Hermetic shape check only; live `--url` is optional and out of CI |
| Readers of DSH docs get lost | Low | Advisory banner + redirect; files kept |
| Index (`AGENTS.md`) drifts from code | Med | 14-9 last; claim only mount paths, not new tools |

## Non-goals

- New MCP tools or protocol versions
- FastEmbed / network tests / changing embedder chain
- Deleting `docs/dsh/`
- Recreating `docs/pi/`
- Wiring probe scripts into GitHub Actions
- Renaming existing `dsh-agent` / `dsh-http-test` fixtures
- Editing `src/mcp/dispatcher.rs` runtime or adding DSH strings there
- New ADR under `docs/adr/`

## Open Questions

None blocking. Optional override: merge 14-1/14-2/14-3 into one test function (YAGNI seat). Default is keep three functions.

## Planning provenance

- Skill: `planning-and-task-breakdown` → `tasks/plan.md` + `tasks/todo.md`
- Seats (third-party): `explore` (`ctyun-deepseek-v4-flash`) inventory; builtin `plan` (`ctyun-deepseek-v4-flash`) YAGNI verdicts; `planner` first attempt truncated (`max_tokens`); `planner` retry (`opencode-go-deepseek-v4-flash`) produced a scrambled ID remap — **discarded**. Main session (`grok-4.6`) synthesized against `src/mcp/*` and `tests/mcp_test.rs`.
- Explore residual claiming `docs/pi/dialectical_counselor_persona.md` / skill missing is **false**; both files exist (verified by `list_dir` + `read_file`).
