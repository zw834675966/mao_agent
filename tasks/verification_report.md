# Cycle 13: Dialectical MCP Engine & DSH Integration 深度核实报告

> **核实时间**：2026-09-07  
> **核实原则**：Anti-Sycophancy Protocol（不轻信勾选状态，真理高于礼貌，凭据说话）  
> **核查范围**：`tasks/plan.md` 与 `tasks/todo.md` 中的 Task 13-1 至 Task 13-8 及 Checkpoint 13-1 至 13-3  
> **执行环境**：Windows 10 / PowerShell 7.6.5 / Rustc & Cargo 1.98.0  

---

## Executive Summary (执行摘要)

经逐项对照代码实现、文件物理存在性、协议契约规范及实际运行 Rust 官方门禁测试：

1. **核心功能真实完成度**：**100% 真实可用**。Model Context Protocol (MCP 2024-11-05) 双通道传输（Stdio 管道与 Streamable HTTP `/api/v1/mcp`）、Google SRE 过载保护（信号量并发门禁与 -32053 映射）、防客户端 400 的 OpenAPI/Draft 7 Schema 安全契约、历史引文自闭环反查防伪逻辑均已完全落地且全绿通过。
2. **官方门禁实测结果**：
   - `cargo fmt --check`：**PASS**（格式完全合规，零 Diff）
   - `cargo clippy --no-default-features --all-targets -- -D warnings`：**PASS**（Dev Profile 编译通过，零警告）
   - `cargo test --no-default-features`：**PASS**（**165 个测试全绿通过，0 失败**）
3. **发现的虚报、偏差与过期项**：
   - **虚报 / 放置位置偏差 (1 项)**：Task 13-6 验收标准声称 `tests/mcp_test.rs` 包含 `test_mcp_stdio_roundtrip`。实测 `tests/mcp_test.rs` **并不存在**该测试；该 stdio 管道测试实际被放在了 `src/mcp/stdio.rs` 单元测试中（名为 `test_stdio_loop_handshake_and_eof`）。
   - **实现与契约偏差 (2 项)**：
     - Task 13-1 声称定义结构体 `McpTextContent`，实际代码实现为枚举 `McpContent::Text { text: String }`。
     - Task 13-5 及 plan.md 规划的指标埋点 `record_mcp_request(tool, duration, is_err)` 及带 `{tool}` 维度的指标，实际代码实现为聚合方法 `record_mcp(&self, started: Instant, is_error: bool)`，未实现按 tool 细分的 Prometheus label 维度。
   - **测试用例命名偏差 (3 项)**：Task 13-6 列出的 `test_mcp_initialize_and_tools_list_schema`、`test_mcp_query_principles_returns_triads`、`test_mcp_citation_verification_auto_lookup` 在 `tests/mcp_test.rs` 中的实际命名为 `test_mcp_handshake_and_capabilities`、`test_mcp_query_with_graph_expansion`、`test_mcp_verify_citation_auto_retrieval` 等。
   - **统计数值滞后/过期 (2 项)**：
     - Checkpoint 13-1 声称 `cargo test --no-default-features --lib mcp` 为 `(9/9 passed)`，现实际为 **16 passed**（包含 stdio 与 server 处理器相关单元测试）。
     - Checkpoint 13-2 与 13-3 声称全量测试为 `160 tests`，现实际全量测试通过数为 **165 tests**。

---

## 官方门禁与抽样测试运行实证 (Gate Evidence)

### 门禁 1: `cargo fmt --check`
```text
$ cargo fmt --check
Exit code: 0
Status: 真实通过 (代码格式规范，无任何待格式化文件)
```

### 门禁 2: `cargo clippy --no-default-features --all-targets -- -D warnings`
```text
$ cargo clippy --no-default-features --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.38s
Exit code: 0
Status: 真实通过 (所有 targets 零 Warning，构建完全清洁)
```

### 门禁 3: `cargo test --no-default-features` (全量测试套件)
```text
$ cargo test --no-default-features
     Running unittests src\lib.rs: 101 passed; 0 failed
     Running unittests src\main.rs: 0 passed; 0 failed
     Running tests\api_test.rs: 22 passed; 0 failed
     Running tests\chunker_test.rs: 1 passed; 0 failed
     Running tests\config_test.rs: 6 passed; 0 failed
     Running tests\e2e_ingest_test.rs: 1 passed; 0 failed
     Running tests\graph_expand_test.rs: 4 passed; 0 failed
     Running tests\graph_store_test.rs: 6 passed; 0 failed
     Running tests\hnsw_regression_test.rs: 1 passed; 0 failed
     Running tests\hybrid_and_agent_test.rs: 4 passed; 0 failed
     Running tests\mcp_test.rs: 10 passed; 0 failed
     Running tests\retrieval_hard_eval_test.rs: 2 passed; 0 failed
     Running tests\vector_store_test.rs: 7 passed; 0 failed
     Doc-tests mao_agent: 0 passed; 0 failed

Total: 165 passed; 0 failed; 0 ignored; finished cleanly.
Exit code: 0
Status: 真实通过 (实测通过 165 个测试，超出 todo.md 记录的 160 个)
```

### 抽样单项测试运行验证
| 测试命令 | 预期状态 | 实测输出 | 结论 |
| :--- | :--- | :--- | :--- |
| `cargo test --no-default-features --lib mcp::types` | 3 tests pass | `3 passed; 0 failed; 98 filtered out` | **真实通过** |
| `cargo test --no-default-features --lib mcp::dispatcher` | 9 tests pass | `9 passed; 0 failed; 92 filtered out` | **真实通过** |
| `cargo test --no-default-features --lib mcp::stdio` | 2 tests pass | `2 passed; 0 failed; 99 filtered out` | **真实通过** |
| `cargo test --no-default-features --lib server::handlers::mcp` | 2 tests pass | `2 passed; 0 failed; 99 filtered out` | **真实通过** |
| `cargo test --no-default-features --test mcp_test` | 10 tests pass | `10 passed; 0 failed; finished in 2.83s` | **真实通过** |
| `cargo run --no-default-features -- mcp --help` | CLI Subcommand Help | 成功打印 `mao_agent.exe mcp` 参数帮助 | **真实通过** |

---

## 逐项任务深度核实矩阵 (Task 13-1 ~ 13-8)

### Task 13-1: MCP JSON-RPC 2.0 Protocol Types and Strict Schema
- **文件检查**：
  - `src/lib.rs`: 存在 `pub mod mcp;` 导出。
  - `src/mcp/mod.rs`: 存在，正确导出 `dispatcher`、`stdio`、`types`。
  - `src/mcp/types.rs`: 存在（400 行）。
- **验收标准逐条对照**：
  1. `src/mcp/mod.rs` 与 `src/mcp/types.rs` 创建并注册：**【真实通过】**。
  2. 结构体定义与 serde derive：**【有偏差】**。
     - `JsonRpcRequest`、`JsonRpcResponse`、`JsonRpcError`、`McpInitializeResult`、`McpToolDefinition`、`McpCallToolResult` 均存在且 derive 正确。
     - **偏差证据**：标准声明为结构体 `McpTextContent`，实际实现为 Tagged Enum `pub enum McpContent { Text { text: String } }`（位于 `src/mcp/types.rs:206-209`）。虽然功能完全等价且更易扩展，但类型命名与文档不一致。
  3. 标准 JSON-RPC / Google API 错误码定义：**【真实通过】**。
     - `PARSE_ERROR (-32700)`、`INVALID_REQUEST (-32600)`、`METHOD_NOT_FOUND (-32601)`、`INVALID_PARAMS (-32602)`、`INTERNAL_ERROR (-32603)`、`RESOURCE_EXHAUSTED (-32053)` 均在 `src/mcp/types.rs:74-80` 严格定义。
  4. Tool Schema 根级 `required` 且无属性级 `required: true`：**【真实通过】**。
     - 单元测试 `test_strict_json_schema_compliance_no_nested_required_true` 递归遍历并通过断言。

---

### Task 13-2: McpDispatcher Core & query_dialectical_principles
- **文件检查**：
  - `src/mcp/dispatcher.rs`: 存在（638 行）。
- **验收标准逐条对照**：
  1. `McpDispatcher::new` 签名与状态注入：**【真实通过】**。
     - 接收 `Arc<VectorStore>`、`Option<Arc<FullTextIndex>>`、`Option<Arc<GraphStore>>`、`Option<Arc<dyn Reranker>>`。
  2. `query_dialectical_principles` 混合检索与图谱三元组扩展：**【真实通过】**。
     - 包含 Vector 检索、Tantivy BM25、RRF 融合、Graph 矛盾三元组展开与可选重排，输出结构化 JSON。
  3. `synthesize: true` 触发哲学报告推演：**【真实通过】**。
     - 内嵌调用 `DialecticalAgent::ask`，失败优雅降级而不导致进程 Panic。
  4. 未知方法与空入参错误处理：**【真实通过】**。
     - 未知方法返回 `-32601`，空入参/超长入参返回 `-32602`。

---

### Task 13-3: verify_historical_citation with Self-Grounding Auto-Retrieval
- **文件检查**：
  - `src/mcp/dispatcher.rs:320-386`: 完整实现。
- **验收标准逐条对照**：
  1. 输入校验与边界钳位：**【真实通过】**。
     - `quote` 与 `claimed_title` 空值检查；`min_confidence` 默认 0.85 且钳位至 `0.0..=1.0`。
  2. 原典自闭环反查防伪设计：**【真实通过 (设计加强)】**。
     - 代码明确规避了外部调用方传入恶意 `context_chunks` 自证假引文的安全风险；无论调用方是否传入 `context_chunks`，均直接通过 `store.chunks_matching_title(&args.claimed_title)` 从本地原典库检索真假。
  3. 篇名不存在判定：**【真实通过】**。
     - 若篇名在本地库未命中，立即返回 `verdict: "DocNotFound"`, `confidence: 0.0`, `is_valid: false`。
  4. 真实引文判定：**【真实通过】**。
     - 精确命中返回 `ExactMatch`，高置信度模糊匹配返回 `FuzzyMatch`。

---

### Checkpoint 13-1: Foundation & Tool Logic
- **验收项比对**：
  - `cargo test --no-default-features --lib mcp`: **【过期/数值滞后】**。
    - 记录写着 `(9/9 passed)`。实测执行 `cargo test --no-default-features --lib mcp` 结果为 **16 passed; 0 failed**。
  - Tool inputSchema 契约断言：**【真实通过】**。
  - 原典自查与伪造拒识验证：**【真实通过】**。
  - Clippy `-D warnings` 清洁：**【真实通过】**。

---

### Task 13-4: Stdio Transport & CLI Subcommand
- **文件检查**：
  - `src/mcp/stdio.rs`: 存在（136 行）。
  - `src/cli/mod.rs:346-386`: 存在 `Commands::Mcp(McpArgs)`。
  - `src/main.rs:26-31, 906-953`: 存在全局 stderr 日志隔离与 `handle_mcp`。
- **验收标准逐条对照**：
  1. `Commands::Mcp(McpArgs)` 支持各项索引路径参数：**【真实通过】**。
  2. `mao_agent mcp` 行缓冲读写：**【真实通过】**。
  3. Tracing 日志物理分流至 `stderr`：**【真实通过】**。
     - `src/main.rs:29` 显式使用 `.with_writer(std::io::stderr)`，杜绝污染 stdout 协议帧。
  4. 管道 EOF 优雅退出：**【真实通过】**。
     - `src/mcp/stdio.rs:62` 捕获 EOF 后记录日志并返回 `Ok(())`，进程返回码 0。

---

### Task 13-5: Streamable HTTP MCP Route with Overload Protection & Metrics
- **文件检查**：
  - `src/server/handlers/mcp.rs`: 存在（194 行）。
  - `src/server/mod.rs:51-52`: 挂载 `/mcp` 与 `/api/v1/mcp`。
  - `src/server/metrics.rs`: 包含 MCP 指标字段。
- **验收标准逐条对照**：
  1. `POST /api/v1/mcp` 与 `/mcp` 返回 `application/json`：**【真实通过】**。
  2. `synthesize: true` 受控于 `state.ask_semaphore` 并在饱和时返回 `-32053`：**【真实通过】**。
     - `src/server/handlers/mcp.rs:63-79` 正确尝试获取 permit，耗尽时立即返回 `JsonRpcError::resource_exhausted`。
  3. 性能指标埋点：**【有偏差】**。
     - **偏差证据**：`tasks/todo.md` 记录为 `record_mcp_request(tool, duration, is_err)` 且 plan.md 说明包含 `mao_mcp_requests_total{tool, status}`。但实际代码在 `src/server/metrics.rs:92` 实现为 `pub fn record_mcp(&self, started: Instant, is_error: bool)`，累计的是全局 `mcp_requests`、`mcp_errors`、`mcp_latency_*`，没有对各个 `tool` 划分 Prometheus label。
  4. Bearer Token 中间件保护与内网回环放行：**【真实通过】**。
     - `src/server/auth.rs` 未将 `/mcp` 列入公有白名单；当配置 `MAO_API_TOKEN` 时强制鉴权，未配置时向回环开放。

---

### Task 13-6: Hermetic End-to-End Integration Test Suite
- **文件检查**：
  - `tests/mcp_test.rs`: 存在（622 行）。
- **验收标准逐条对照**：
  1. `test_mcp_initialize_and_tools_list_schema`：**【有偏差 (命名)】**。
     - 实际对应的测试函数为 `test_mcp_handshake_and_capabilities`（覆盖协议版本与严格 Draft 7 Schema 检查）。
  2. `test_mcp_query_principles_returns_triads`：**【有偏差 (命名)】**。
     - 实际对应的测试函数为 `test_mcp_query_with_graph_expansion`（覆盖图谱矛盾关联）。
  3. `test_mcp_citation_verification_auto_lookup`：**【有偏差 (拆分与命名)】**。
     - 实际对应 `test_mcp_verify_citation_auto_retrieval` 与 `test_mcp_verify_citation_adversarial_rejection`。
  4. `test_mcp_stdio_roundtrip`：**【虚报 / 放置位置偏差】**。
     - **虚报证据**：`tests/mcp_test.rs` 中**根本没有**名为 `test_mcp_stdio_roundtrip` 的函数，该集成测试文件主要通过 `oneshot` 模拟 HTTP 请求。模拟 Stdio 输入输出管道的测试实际上存放在 `src/mcp/stdio.rs` 内部（即 `test_stdio_loop_handshake_and_eof`）。
  5. 测试全量绿色运行：**【真实通过 (数值微调)】**。
     - `tests/mcp_test.rs` 包含 10 个测试用例，全数通过。全工程测试通过数为 165，而非 160。

---

### Checkpoint 13-2: Dual Transport & SRE Verification
- **验收项比对**：
  - 全量测试通过：**【真实通过】**（实测 165 passed）。
  - 双通道传输覆盖：**【真实通过】**（Stdio 与 HTTP `/api/v1/mcp` + `/mcp` 均覆盖）。
  - 并发限流与错误码映射验证：**【真实通过】**（`test_mcp_overload_protection_on_synthesis` 覆盖）。

---

### Task 13-7: DSH cordis.patch.yml Configuration & Operational Runbook
- **文件检查**：
  - `docs/dsh/cordis.patch.example.yml`: 存在（52 行）。
  - `docs/ops/mcp_sre_guide.md`: 存在（153 行）。
- **验收标准逐条对照**：
  1. `cordis.patch.example.yml` 格式规范：**【真实通过】**。
     - 严格符合 `@deepseek-ai/dsh-mcp-client` 的 `- insert:` 语法，分别提供了 Stdio 本地子进程启动模版与 Streamable HTTP 集中守护进程启动模版。
  2. `mcp_sre_guide.md` 运维指南：**【真实通过】**。
     - 覆盖架构拓扑图、双通道传输说明、严格 JSON Schema 保证、Google SRE 错误码映射表、Prometheus 告警建议与排障手册。

---

### Task 13-8: Dialectical Counselor System Persona Charter
- **文件检查**：
  - `docs/dsh/dialectical_counselor_persona.md`: 存在（222 行）。
- **验收标准逐条对照**：
  1. 系统提示词人设完整性：**【真实通过】**。
     - 包含 Meta Persona、六大工程防御维度（上下文/工具/流程FSM/持久化/安全/可观测性）。
  2. 与 DSH `dsh-pai-lite` 闸门规则对齐：**【真实通过】**。
     - 严格定义“主矛 / 最小切片 / 验收”三要素冻结机制，前置拦截冒进行为。
  3. 工具调用指引与实战模版：**【真实通过】**。
     - 包含明确的何时调用 `query_dialectical_principles` 和 `verify_historical_citation` 的指导范式。

---

### Checkpoint 13-3: Full Production Gate & Delivery
- **验收项比对**：
  - `cargo fmt --check`: **【真实通过】**
  - `cargo clippy --no-default-features --all-targets -- -D warnings`: **【真实通过】**
  - `cargo test --no-default-features`: **【真实通过】** (165 passed)
  - 交付状态：**【就绪】**

---

## 修正建议与改进清单 (Actionable Adjustments)

为保持文档的一致性与代码库的极客严谨性，建议在后续文档维护中进行以下轻量修正：

1. **更新 `tasks/todo.md` 中的测试用例名称与归属**：
   - 将 Task 13-6 中的 `test_mcp_stdio_roundtrip` 说明修正为：“Stdio roundtrip verified via unit tests in `src/mcp/stdio.rs` (`test_stdio_loop_handshake_and_eof`)”。
   - 将 Task 13-6 中对 `test_mcp_initialize_and_tools_list_schema`、`test_mcp_query_principles_returns_triads` 等的描述同步为当前代码中实际的函数名。
2. **校准测试通过统计数**：
   - 将 Checkpoint 13-1 的 `(9/9 passed)` 更新为 `(16/16 passed)`。
   - 将 Checkpoint 13-2 与 13-3 中的 `160 tests` 更新为实测准确的 `165 tests`。
3. **指标方法文档对齐**：
   - 将 Task 13-5 验收项中的 `record_mcp_request(tool, duration, is_err)` 改写为 `record_mcp(started, is_err)`，或在未来迭代中若确实需要按工具细分统计，再引入带 `tool` label 的细粒度指标。