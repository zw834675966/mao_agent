# Implementation Plan: Cycle 13 — Dialectical MCP Engine & DSH Integration (SRE & Google Engineering Aligned)

## Overview

将已有的 `mao_agent`（毛选/辩证唯物主义混合检索、图谱关联与引用校验引擎）转化为工业级 Model Context Protocol (MCP) 服务端，作为 DeepSeek Harness (DSH) 等通用 Agent 运行时的“思维中枢与行为宪章外脑”。

本方案全面对齐 **Google SRE 系列**（生产稳定性、过载保护、SLO/SLI 可观测性）、**Google Engineering Practices**（小粒度单责任 CL、密封测试、严格代码审查）以及 **Google API Design Guide**（规范化错误模型、输入防御性校验与幂等性设计），确保服务在本地 stdio 管道与高并发 HTTP 场景下均具备生产级健壮性。

---

## Google SRE, Eng Practices & API Design 对齐矩阵

### 1. Google SRE 生产稳定性与可靠性规范 (SRE Books Alignment)
- **过载保护与背压 (Handling Overload - Chapter 21)**:
  - 在 Axum HTTP 模式下，当调用方请求 `synthesize: true`（触发内部大模型辩证推演）时，强制复用 `AppState.ask_semaphore` 并发信号量（默认 32），超出限制时返回标准的 JSON-RPC `Overload / ResourceExhausted` 错误，严防耗尽连接池导致服务雪崩。
- **故障域隔离 (Failure Domain Isolation - Chapter 22)**:
  - 核心业务逻辑沉淀在 `McpDispatcher`，传输层（Stdio 与 Axum HTTP）作为轻量外壳物理隔离。
  - Stdio 管道传输中，任何内部错误只返回 JSON-RPC 错误对象，严禁触发进程 `panic!`；当宿主进程关闭管道（stdin EOF）时，子进程必须通过优雅关闭流程安全释放文件描述符与内存映射。
- **可观测性与 SLO/SLI 体系 (Monitoring Distributed Systems - Chapter 6)**:
  - 指标埋点：在 `metrics.rs` 中增加 MCP 专属指标：`mao_mcp_requests_total{tool, status}`、`mao_mcp_duration_seconds{tool}`。
  - 链路追踪与日志：继承 `X-Request-Id`，所有 stdio 模式日志定向至 `stderr`，确保 `stdout` 仅流动确定性的 JSON-RPC 协议帧。

### 2. Google Engineering Practices 代码审查规范 (Eng Practices Alignment)
- **Small & Cohesive CLs (The Change Author's Guide)**:
  - 严禁将协议类型、分发器、两项工具逻辑与两个传输层杂糅在一个超大提交中。
  - 将实施任务拆分为 8 个高内聚、单责任的垂直切片（每个切片修改 1~3 个文件，耗时 ≤ 1 小时），每个切片具备自包含的单元测试与即时验证。
- **密封隔离测试 (Hermetic Testing - The Code Reviewer's Guide)**:
  - 集成测试使用 `DeterministicEmbedder` 和内存级 Tantivy/Graph 夹具，不依赖外部网络连接、真实 API Key 或外部进程，确保 CI 100% 可重复执行、零 Flaky 测试。

### 3. Google API Design Guide 接口设计规范 (API Design Alignment)
- **规范化错误模型 (Error Model - Section 4)**:
  - 严格映射 Google API 错误语义至 JSON-RPC 2.0 规范：
    - `INVALID_ARGUMENT` (-32602): 参数缺失、空字符串、超出合法数值范围。
    - `METHOD_NOT_FOUND` (-32601): 未知 MCP 方法。
    - `RESOURCE_EXHAUSTED` (-32053): 并发推演超限。
    - `INTERNAL` (-32603): 索引读取异常或不可恢复故障。
- **输入防御性校验与归一化 (Input Validation - Section 5)**:
  - 对 `top_k` 进行强制钳位（1~20，默认 3）；对 `min_confidence` 进行边界钳位（0.0~1.0，默认 0.85）；对文本入参进行 `trim()` 与非空校验。
- **Schema 安全契约 (Preventing OpenCode Go 400)**:
  - 遵循标准 OpenAPI/Draft 7 规范：根级必须为 `"type": "object"`，属性定义在 `properties` 下，必填字段声明在根级的 `required: ["field1", ...]` 数组中，严禁在属性内声明 `required: true`。

---

## 架构演进与依赖图谱 (Architecture Dependency Graph)

```
[Task 13-1: types.rs & JSON Schema] ──┐
                                     ▼
                      [Task 13-2: McpDispatcher 核心 & 辩证方法论检索]
                                     │
                                     ├──► [Task 13-3: 引用校验与原典自闭环反查]
                                     │
                                     ├──► [Task 13-4: Stdio 管道传输 (mao_agent mcp)]
                                     │
                                     └──► [Task 13-5: Axum HTTP MCP 端点 (POST /api/v1/mcp)]
                                                    │
                                                    ▼
                                    [Task 13-6: 密封端到端集成测试套件]
                                                    │
                                                    ▼
                                    [Task 13-7 & 13-8: DSH 挂载配置与行为宪章]
```

---

## Refined Task Breakdown (垂直切片列表)

### Phase 1: 协议基础与错误模型
- **Task 13-1**: MCP JSON-RPC 2.0 协议类型、错误模型与严格 Draft 7 Schema
- **Task 13-2**: `McpDispatcher` 核心架构与 `query_dialectical_principles` 方法论检索
- **Task 13-3**: `verify_historical_citation` 引用校验与自闭环篇名/原典正文反查
- **Checkpoint 13-1**: 核心分发器与工具单元测试全绿，Schema 契约断言通过

### Phase 2: 双通道传输与过载保护
- **Task 13-4**: Stdio 传输子命令与日志隔离 (`mao_agent mcp`，EOF 优雅退出)
- **Task 13-5**: Axum Streamable HTTP 端点 (`POST /api/v1/mcp`)、并发信号量保护与可观测性埋点
- **Task 13-6**: 编写全量密封集成测试 `tests/mcp_test.rs`
- **Checkpoint 13-2**: 139+ 全量测试常绿，Stdio/HTTP 双通道覆盖，过载熔断验证

### Phase 3: DSH 生产集成与行为宪章
- **Task 13-7**: 编写 DSH `cordis.patch.yml` 生产配置模版与运维操作手册
- **Task 13-8**: 制定《辩证参谋行为宪章》（DSH System Persona Prompt 规范）
- **Checkpoint 13-3**: 官方质量门禁（`fmt`、`clippy -D warnings`、`test`）全数通过

---

## Risks and Mitigations (SRE 风险防控矩阵)

| 风险类别 | 潜在故障表现 | SRE 预防与自愈机制 |
| :--- | :--- | :--- |
| **管道通信死锁 (Stdio)** | 模型输出过长阻塞缓冲区，或 EOF 未捕获导致僵尸进程 | 采用异步非阻塞缓冲读取 `tokio::io::BufReader`；遇到 EOF 即刻触发 break 优雅退出；日志强制分流至 `stderr`。 |
| **高并发推演雪崩 (HTTP)** | 大量并发请求调用 `synthesize: true` 耗尽内存或触发上游限流 | 严格受控于 `AppState.ask_semaphore`；当并发饱和时立即返回 `RESOURCE_EXHAUSTED` 错误，保护核心检索通道。 |
| **Schema 格式错误 (Client)** | DSH OpenCode Go 控制台返回 400 导致整个会话瘫痪 | 单元测试内增加严格的 JSON Schema 递归断言，确保没有任何属性包含非标准的 `required: true`。 |
| **引文反查跨篇混淆** | 虚构篇名在检索库中召回了无关篇目并误判通过 | 必须先校验 `claimed_title` 是否在库中有明确的文档标题命中；若无直接返回 `DocNotFound` 判定，置信度设为 0.0。 |
