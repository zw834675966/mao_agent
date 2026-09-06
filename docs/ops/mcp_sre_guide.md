# Mao Agent MCP Server SRE & Operations Guide

> **Protocol Conformance**: Model Context Protocol (MCP) Specification `2024-11-05` / JSON-RPC 2.0  
> **Target Runtimes**: DeepSeek Harness (DSH), OpenCode, Claude Desktop, Cursor, and custom agent harnesses.  
> **Engineering Standard**: Google SRE (Site Reliability Engineering), Failure Domain Isolation, Hermetic Verification.

---

## 1. Architecture & Transport Topologies

`mao_agent` provides dual-transport Model Context Protocol (MCP) capabilities to serve as the dialectical thinking brain and citation verifier for frontier agent loops:

```
                      ┌────────────────────────────────────────┐
                      │    Agent Harness (DSH / OpenCode)      │
                      └────┬───────────────────────────────┬───┘
                           │ (Stdio: Line-delimited JSON)   │ (HTTP POST: /api/v1/mcp)
                           ▼                               ▼
                 ┌──────────────────┐            ┌──────────────────┐
                 │  mao_agent mcp   │            │  mao_agent serve │
                 │ (Stdio Transport)│            │ (HTTP Transport) │
                 └─────────┬────────┘            └─────────┬────────┘
                           │                               │
                           └───────────────┬───────────────┘
                                           ▼
                                ┌─────────────────────┐
                                │    McpDispatcher    │
                                └──────────┬──────────┘
                                           │
         ┌─────────────────────────────────┼─────────────────────────────────┐
         ▼                                 ▼                                 ▼
┌──────────────────┐             ┌──────────────────┐             ┌──────────────────┐
│ Hybrid RRF Search│             │ Knowledge Graph  │             │Citation Verifier │
│ (Vector+Tantivy) │             │ (Triad Expansion)│             │ (Character Match)│
└──────────────────┘             └──────────────────┘             └──────────────────┘
```

### Stdio Transport (`cargo run -- mcp`)
- **Process Isolation**: Spawns directly under agent supervisor.
- **Framing Integrity**: All diagnostic traces (`tracing-subscriber`) are strictly routed to `stderr`. `stdout` is exclusively reserved for valid, single-line JSON-RPC 2.0 messages.
- **Stream Termination**: Cleanly handles `EOF` without panics or dangling threads.

### HTTP Transport (`/api/v1/mcp` and `/mcp`)
- **Multiplexed Multi-Agent Service**: Multiple external agent instances can connect concurrently to a single central `mao_agent` node.
- **Authentication**: Supports standard `Authorization: Bearer <token>` via `MAO_API_TOKEN` / `--api-token`.
- **CORS & Observability**: Standardized `X-Request-Id` tracking and Prometheus metrics exposition at `/metrics`.

---

## 2. Tools & Schema Guarantees

All exposed tools are generated with strict JSON Schema Draft 7 conformance to eliminate DSH / OpenCode Go schema parsing panics:
- Root-level `"type": "object"`
- Direct property dictionary under `"properties"`
- Root-level required string array under `"required"` (zero property-level `"required": true`)

### 1. `query_dialectical_principles`
- **Purpose**: Retrieves philosophical methodology, contradiction triads, and historical strategic paradigms.
- **Input Parameters**:
  - `query` (*string, required*): The contradiction scenario, architectural dilemma, or philosophical query.
  - `top_k` (*integer, optional, default: 3, range: 1..20*): Number of canonical chunks to retrieve.
  - `period` (*string, optional*): Filter by historical period (e.g. `土地革命战争时期`, `抗日战争时期`, `解放战争时期`, `社会主义建设时期`).
  - `volume` (*string, optional*): Filter by Selected Works volume (e.g. `毛泽东选集第一卷`).
  - `synthesize` (*boolean, optional, default: false*): When true, triggers in-depth DialecticalAgent synthesis (primary contradiction breakdown and strategic roadmap).
- **Execution Pipeline**:
  `Vector Store + Tantivy BM25` → `RRF Fusion` → `Knowledge Graph Triad Expansion` → `Optional Cohere Rerank` → `Optional Synthesis`.

### 2. `verify_historical_citation`
- **Purpose**: Authenticates historical quotes and citations against canon with character-level accuracy.
- **Input Parameters**:
  - `quote` (*string, required*): The exact quote or proposition statement to verify.
  - `claimed_title` (*string, required*): The claimed canonical work title (e.g., `《反对本本主义》`, `《矛盾论》`, `《实践论》`).
  - `context_chunks` (*array of strings, optional*): Explicit text chunks for comparison. If omitted or empty, the server automatically executes a title-indexed reverse lookup against the local corpus.
  - `min_confidence` (*number, optional, default: 0.85*): Confidence threshold (0.0 to 1.0).
- **Output Verdicts**:
  - `ExactMatch`: Character-level authentic match (confidence ≥ 0.999).
  - `FuzzyMatch`: Partial or variant wording match above `min_confidence`.
  - `UnverifiedOrFabricated`: Rejected as fabricated or distorted quotation.
  - `DocNotFound`: Claimed document title does not exist in canonical corpus.

---

## 3. Google SRE Reliability & Overload Protection

### Concurrency Gating
Dialectical LLM synthesis (`synthesize: true`) is gated by an atomic semaphore (`ask_semaphore`).
- Default concurrency capacity: `32` (configurable via `--max-concurrent-asks` or `MAO_MAX_CONCURRENT_ASKS`).
- Search and verification operations are zero-cost local operations and are **never** blocked by LLM inference saturation.

### Standardized Error Code Matrix

| JSON-RPC Code | SRE Canonical Code | Meaning / Scenario |
|:---|:---|:---|
| `-32700` | `INVALID_ARGUMENT` | Parse Error: Malformed JSON payload received. |
| `-32600` | `INVALID_ARGUMENT` | Invalid Request: Envelope missing `jsonrpc` or `method`. |
| `-32601` | `UNIMPLEMENTED` | Method Not Found: Requested RPC method does not exist. |
| `-32602` | `INVALID_ARGUMENT` | Invalid Params: Missing required fields or schema validation failure. |
| `-32053` | `RESOURCE_EXHAUSTED`| Concurrency Limit Exceeded: `ask_semaphore` capacity exhausted. |
| `-32603` | `INTERNAL` | Internal Error: Storage corruption or unexpected runtime panic. |

---

## 4. Observability & Prometheus Metrics

All MCP operations report real-time Prometheus counters and latency gauges at `GET /metrics`:

```promql
# HELP mao_mcp_requests_total Total /api/v1/mcp requests
# TYPE mao_mcp_requests_total counter
mao_mcp_requests_total 1284

# HELP mao_mcp_errors_total Total /api/v1/mcp errors
# TYPE mao_mcp_errors_total counter
mao_mcp_errors_total 3

# HELP mao_mcp_latency_ms_sum Sum of /api/v1/mcp latency in milliseconds
# TYPE mao_mcp_latency_ms_sum counter
mao_mcp_latency_ms_sum 15420

# HELP mao_mcp_latency_ms_count Count of /api/v1/mcp latency samples
# TYPE mao_mcp_latency_ms_count counter
mao_mcp_latency_ms_count 1284

# HELP mao_mcp_latency_ms_max Max /api/v1/mcp latency in milliseconds
# TYPE mao_mcp_latency_ms_max gauge
mao_mcp_latency_ms_max 142
```

### Alerting Recommendations
- **High Error Rate**: `rate(mao_mcp_errors_total[5m]) / rate(mao_mcp_requests_total[5m]) > 0.05`
- **P99 Latency Spill**: `mao_mcp_latency_ms_max > 2000` (indicates remote LLM timeout or disk contention)
- **Overload Spikes**: Frequent `-32053 RESOURCE_EXHAUSTED` responses indicate need to increase `--max-concurrent-asks` or horizontal replica scaling.

---

## 5. Troubleshooting Runbook

### Issue 1: DSH / OpenCode UI crashes with 400 on startup
- **Cause**: Tool schema contained property-level `required: true` or top-level `type` was missing.
- **Resolution**: `mao_agent` strictly conforms to JSON Schema Draft 7 with root `required: [...]` array. Ensure client configuration points to `mao_agent` version `≥ 0.1.0`.

### Issue 2: Agent reports "Broken pipe" or Stdio framing corruption
- **Cause**: Background log messages printed to `stdout` instead of `stderr`.
- **Resolution**: In `src/main.rs`, verify `tracing_subscriber` is initialized with `.with_writer(std::io::stderr)`.

### Issue 3: Verification returns `DocNotFound`
- **Cause**: The `claimed_title` provided by the model does not match titles in `data/vector_store.bin`.
- **Resolution**: Run `cargo run -- stats` to inspect all loaded document titles. The title matching algorithm automatically normalizes book brackets (e.g. `《矛盾论》` ↔ `矛盾论`).

### Issue 4: Windows binary lock (`os error 5 拒绝访问`)
- **Cause**: A previously launched daemon or test process is still holding `target/debug/mao_agent.exe`.
- **Resolution**: Run `Get-Process | Where-Object { $_.ProcessName -like "*mao_agent*" } | Stop-Process -Force`.
