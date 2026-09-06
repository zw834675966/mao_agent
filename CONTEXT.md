# CONTEXT.md — mao_agent

## What this system is

`mao_agent` is a single-crate Rust binary/library that turns a Chinese historical corpus (primarily Mao Zedong's writings) into a **retrieval + dialectical answering** service:

- **Ingest**: markdown corpus → semantic chunks → vector snapshot + Tantivy BM25 index
- **Retrieve**: hybrid (vector + BM25 RRF) with optional Cohere rerank
- **Answer**: dialectical four-stage synthesis grounded in retrieved evidence, with citation verification
- **Serve**: Axum REST + SSE for intranet clients

It is intentionally small-team oriented: one deployable binary, file-backed indexes, optional remote Cohere for embed/chat/rerank, deterministic offline fallbacks for tests and degraded operation.

## Production target: **B — small-team intranet**

"B-grade" means what it takes to run safely for a small team on a private network:

| Expectation | B-grade bar |
|---|---|
| Readiness | `/health` reflects whether the vector index is actually loaded (fail closed) |
| Lifecycle | Graceful shutdown on Ctrl+C / SIGTERM so in-flight requests can drain |
| Degraded LLM | If a Cohere/chat key is configured but the call fails, fall back to offline dialectical template instead of hard-failing the ask path |
| Filters | Metadata filters must not silently admit unknown/empty dates |
| Surface | Default bind `127.0.0.1`; CORS allowlist (localhost defaults + `--cors-origins` / `MAO_CORS_ORIGINS` / config) — ADR 0004 |
| Ops | Request-id (`X-Request-Id`), `/metrics` + `/api/v1/metrics`, Cohere chat/rerank retries; docs + CI gates |

B is **not** multi-tenant SaaS, HA clusters, or public internet hardening.

## Boundaries

**In scope**

- Local or shared-intranet serving of search / ask / verify
- Offline-capable tests and CLI (`--offline`, deterministic embedder, 512-dim)
- Production embeddings: SiliconFlow `BAAI/bge-m3` (1024-dim); Cohere optional for chat/rerank only
- File indexes under `data/` (gitignored artifacts)

**Out of scope / non-goals**

- Public internet exposure with authN/Z, rate limits, WAF
- Multi-node index replication / consensus
- Guaranteed LLM online availability (offline template is a valid degraded mode)
- Perfect historical scholarship UI; this is an engineering retrieval agent
- Changing corpus IP / licensing assumptions beyond what operators already control

## Data flow (sketch)

```
corpus/*.md
    │ ingest                          python scripts/build_knowledge_graph.py --mock
    ▼                                 │ ingest-graph
data/vector_store.bin  +  data/tantivy_index/  +  optional data/graph_store.bin
    │ serve / ask / search --graph-file (missing = dual-only)
    ▼
Hybrid retrieve (BM25 + Vector RRF)
    │ optional 1–2 hop graph bonus (not a third RRF stream, not --mode graph)
    ▼
optional Cohere rerank ──► LlmClient
                         │
              Online (key) ──fail──► Offline template
                         │
                         ▼
               CitationVerifier ──► JSON / SSE
```

## Related docs

- `AGENTS.md` — agent/dev command map and test conventions
- `docs/adr/` — locked runtime decisions for B-grade P0/P1 (0001–0004)
- `tasks/todo.md` — cycle checklist + remaining gaps (P0–P3)
