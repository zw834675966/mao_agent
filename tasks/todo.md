# Tasks: Cycle 13 — Dialectical MCP Engine & DSH Integration (Google SRE & Eng Practices Aligned)

Plan document: `tasks/plan.md`.
Corpus & Architecture context: `README.md`, `AGENTS.md`.

---

## Task 13-1: Define MCP JSON-RPC 2.0 Protocol Types and Strict Schema

**Description:** Define standard Model Context Protocol (MCP 2024-11-05) data structures and Google API-aligned error models in `src/mcp/types.rs`. Implement standard JSON-RPC 2.0 requests, responses, notifications, `initialize`, `tools/list`, and `tools/call`. Tool schemas must strictly conform to OpenCode Go / Draft 7 specifications: root `"type": "object"`, explicit `properties`, and root-level `required: [...]` string arrays (never property-level `required: true`).

**Acceptance criteria:**
- [x] `src/mcp/mod.rs` and `src/mcp/types.rs` created and registered in `src/lib.rs`.
- [x] Structures defined with serde derives: `JsonRpcRequest`, `JsonRpcResponse`, `JsonRpcError`, `McpInitializeResult`, `McpToolDefinition`, `McpCallToolResult`, `McpTextContent`.
- [x] Standard JSON-RPC / Google API error codes defined: `INVALID_PARAMS` (-32602), `METHOD_NOT_FOUND` (-32601), `RESOURCE_EXHAUSTED` (-32053), `INTERNAL_ERROR` (-32603).
- [x] Tool definitions for `query_dialectical_principles` and `verify_historical_citation` produce valid JSON Schema with root `required` array and zero property-level `required: true`.

**Verification:**
- [x] Tests pass: `cargo test --no-default-features --lib mcp::types`
- [x] Build succeeds: `cargo check --no-default-features`

**Dependencies:** None

**Files likely touched:**
- `src/lib.rs`
- `src/mcp/mod.rs`
- `src/mcp/types.rs`

**Estimated scope:** Small (3 files)

---

## Task 13-2: Implement McpDispatcher Core & query_dialectical_principles

**Description:** Implement `McpDispatcher` in `src/mcp/dispatcher.rs` handling `initialize`, `notifications/initialized`, `ping`, `tools/list`, and `query_dialectical_principles`. Implements hybrid search + graph triad expansion and optional `synthesize: bool` execution with input validation (clamped `top_k: 1..=20`, non-empty query check).

**Acceptance criteria:**
- [x] `McpDispatcher::new(...)` constructed with `Arc<VectorStore>`, `Option<TantivyIndex>`, `Option<Arc<GraphStore>>`, and `Option<Arc<dyn Reranker>>`.
- [x] `query_dialectical_principles` executes hybrid RRF retrieval, expands graph contradiction triads, and formats output as structured JSON.
- [x] When `synthesize: true` is requested, invokes `DialecticalAgent` to append structured philosophical analysis report.
- [x] Unknown method returns standard -32601 `MethodNotFound` error; empty query returns -32602 `InvalidParams`.

**Verification:**
- [x] Tests pass: `cargo test --no-default-features --lib mcp::dispatcher`
- [x] Clippy clean: `cargo clippy --no-default-features --lib -- -D warnings`

**Dependencies:** Task 13-1

**Files likely touched:**
- `src/mcp/mod.rs`
- `src/mcp/dispatcher.rs`

**Estimated scope:** Medium (2 files)

---

## Task 13-3: Implement verify_historical_citation with Self-Grounding Auto-Retrieval

**Description:** Implement the `verify_historical_citation` tool inside `McpDispatcher` in `src/mcp/dispatcher.rs`. When `context_chunks` is missing or empty, automatically looks up the authentic document chunks from the local corpus using `claimed_title` and `quote`, feeding them into `CitationVerifier`.

**Acceptance criteria:**
- [x] Input validation: `quote` and `claimed_title` must not be empty; `min_confidence` clamped to `0.0..=1.0` (default 0.85).
- [x] If `context_chunks` is omitted, auto-retrieves matching document chunks by title from `VectorStore` / `TantivyIndex`.
- [x] If claimed document title does not exist in local corpus, returns structured verdict `DocNotFound` with `confidence: 0.0` and `is_valid: false`.
- [x] Authentic quotes against real corpus titles yield `ExactMatch` or `FuzzyMatch` with `is_valid: true`.

**Verification:**
- [x] Tests pass: `cargo test --no-default-features --lib mcp::dispatcher::tests`
- [x] Clippy clean: `cargo clippy --no-default-features --lib -- -D warnings`

**Dependencies:** Task 13-2

**Files likely touched:**
- `src/mcp/dispatcher.rs`
- `src/vector/index.rs`
- `src/vector/store.rs`

**Estimated scope:** Small-Medium (2 files)

---

## Checkpoint 13-1: Foundation & Tool Logic
- [x] `cargo test --no-default-features --lib mcp` all green (9/9 passed)
- [x] Tool inputSchema assertion passes: root `required: [...]` only, valid Draft 7 JSON Schema
- [x] Auto-retrieval verification passes for authentic and fabricated quotes
- [x] Clippy `-D warnings` clean

---

## Task 13-4: Implement Stdio Transport & CLI Subcommand (SRE Graceful EOF)

**Description:** Implement `src/mcp/stdio.rs` for newline-delimited JSON-RPC 2.0 streaming over `stdin`/`stdout`. Add `Commands::Mcp(McpArgs)` in `src/cli/mod.rs` and handle it in `src/main.rs`. Ensure all logging (`tracing`) is physically redirected to `stderr` to prevent JSON-RPC stdout pollution, and stdin EOF triggers clean shutdown without leaks.

**Acceptance criteria:**
- [x] `src/cli/mod.rs` has `Commands::Mcp(McpArgs)` with paths to index, tantivy, and graph files.
- [x] Running `mao_agent mcp` reads stdin line by line and outputs JSON-RPC lines to stdout.
- [x] Tracing logs in `main.rs` are directed to `std::io::stderr` during stdio MCP execution.
- [x] Reaching EOF on stdin cleanly exits the loop and process with code 0 (no hang, no panic).

**Verification:**
- [x] Tests pass: `cargo test --no-default-features --lib mcp::stdio`
- [x] Build succeeds: `cargo check --no-default-features`

**Dependencies:** Task 13-3

**Files likely touched:**
- `src/mcp/stdio.rs`
- `src/cli/mod.rs`
- `src/main.rs`

**Estimated scope:** Medium (3 files)

---

## Task 13-5: Mount Streamable HTTP MCP Route with Overload Protection & Metrics

**Description:** Expose Streamable HTTP MCP endpoint (`POST /api/v1/mcp`) in `src/server/handlers/mcp.rs` and register it in `src/server/mod.rs`. Protect heavy `synthesize: true` calls with `AppState.ask_semaphore` concurrency control, and record MCP request metrics in `src/server/metrics.rs`.

**Acceptance criteria:**
- [x] `POST /api/v1/mcp` accepts JSON-RPC request and returns JSON-RPC response with `content-type: application/json`.
- [x] Calls requesting `synthesize: true` acquire permit from `state.ask_semaphore`; if capacity exhausted, returns standard -32053 `RESOURCE_EXHAUSTED` error.
- [x] Metrics updated: `record_mcp_request(tool, duration, is_err)`.
- [x] Route protected by existing bearer token middleware if configured, or open if loopback without token.

**Verification:**
- [x] Tests pass: `cargo test --no-default-features --lib server::handlers::mcp`
- [x] Build succeeds: `cargo check --no-default-features`

**Dependencies:** Task 13-3

**Files likely touched:**
- `src/server/handlers/mod.rs`
- `src/server/handlers/mcp.rs`
- `src/server/metrics.rs`
- `src/server/mod.rs`

**Estimated scope:** Medium (4 files)

---

## Task 13-6: Hermetic End-to-End Integration Test Suite

**Description:** Create `tests/mcp_test.rs` with hermetic test fixtures. Validates end-to-end MCP lifecycle: `initialize` handshake, `tools/list` schema validation, `query_dialectical_principles` execution, auto-retrieval `verify_historical_citation`, and error handling for malformed JSON and unknown methods.

**Acceptance criteria:**
- [x] Test `test_mcp_initialize_and_tools_list_schema` confirms protocol version and valid OpenAPI schema.
- [x] Test `test_mcp_query_principles_returns_triads` validates retrieval + graph relationships.
- [x] Test `test_mcp_citation_verification_auto_lookup` verifies real title match vs fabricated title rejection.
- [x] Test `test_mcp_stdio_roundtrip` simulates stdin/stdout pipeline without network dependencies.

**Verification:**
- [x] Tests pass: `cargo test --no-default-features --test mcp_test`
- [x] Full suite green: `cargo test --no-default-features` (160 tests passing)

**Dependencies:** Task 13-4, Task 13-5

**Files likely touched:**
- `tests/mcp_test.rs`

**Estimated scope:** Medium (1 test file)

---

## Checkpoint 13-2: Dual Transport & SRE Verification
- [x] Full test suite passes: `cargo test --no-default-features` (all 160 unit & integration tests green)
- [x] Stdio and Streamable HTTP endpoints both verified
- [x] Concurrency limit and error code mappings verified

---

## Task 13-7: DSH cordis.patch.yml Configuration & Operational Runbook

**Description:** Create ready-to-use DSH mounting configuration artifacts in `docs/dsh/` including both `stdio` and `streamable-http` configurations for `cordis.patch.yml`, with exact syntax verified against DSH `@deepseek-ai/dsh-mcp-client`. Add SRE operational guide in `docs/ops/mcp_sre_guide.md`.

**Acceptance criteria:**
- [x] `docs/dsh/cordis.patch.example.yml` created with valid `- insert:` syntax for `mcp-mao`.
- [x] `docs/ops/mcp_sre_guide.md` created covering monitoring metrics, error codes, and troubleshooting runbooks.

**Verification:**
- [x] Manual check: syntax passes YAML lint and matches local DSH profile structure.

**Dependencies:** Task 13-6

**Files likely touched:**
- `docs/dsh/cordis.patch.example.yml`
- `docs/ops/mcp_sre_guide.md`

**Estimated scope:** XS (2 doc files)

---

## Task 13-8: Dialectical Counselor System Persona Charter

**Description:** Write `docs/dsh/dialectical_counselor_persona.md` documenting the 4-step Dialectical Thinking Chain (实事求是/调查研究 -> 矛盾剖析/问题定性 -> 战略切片/作战计划 -> 实践检验/闭环标准). Establishes operational guidelines for DSH agents to invoke `mcp__mao__*` tools before touching destructive tools.

**Acceptance criteria:**
- [x] Document contains complete, copy-pasteable system prompt persona for DSH.
- [x] Seamlessly aligns with DSH's existing `dsh-pai-lite` action gate (`主矛`, `最小切片`, `验收`).
- [x] Includes clear tool-calling guidelines and examples for when to consult `query_dialectical_principles` and `verify_historical_citation`.

**Verification:**
- [x] Manual check: comprehensive review against user behavioral guidelines.

**Dependencies:** Task 13-7

**Files likely touched:**
- `docs/dsh/dialectical_counselor_persona.md`

**Estimated scope:** XS (1 doc file)

---

## Checkpoint 13-3: Full Production Gate & Delivery
- [x] `cargo fmt --check`
- [x] `cargo clippy --no-default-features --all-targets -- -D warnings`
- [x] `cargo test --no-default-features` all green (160 tests)
- [x] Ready for user review and deployment into DSH

---

## Cycle 12 closure note (2026-09-06, appended post-rewrite)

Cycle 12 tasks (Google 3 Nits + ANALYSIS C-1~C-7) completed earlier this session; this file was rewritten for Cycle 13 in a parallel session, so the historical checklist is preserved in git history + ANALYSIS.md. Final unrun gate now executed:

- [x] serve live probe on current default index (port 3210, serve --offline --graph-file data/graph_store.bin): /live **200**, /health **200** (index_loaded/tantivy_loaded true, 59 vectors / 15 docs / 512-dim), /metrics **200**.
- [x] POST /api/v1/search {"mode":"graph"} → **400** (mode allowlist intact).
- [x] Hybrid 主要矛盾与阿姆达尔定律 over HTTP: 3 hits, Top1《矛盾论》 graph-annotated (seed ref resolves; empty Amdahl bonus expected on sample index — scope note in vals/retrieval/GRAPH.md).
- Nit logged during probe: seed-annotated dual hits serialize graph_paths: [] (empty array) instead of absent — cosmetic, defer to a later cycle.
