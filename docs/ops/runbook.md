# Operator runbook (B-grade intranet)

## Goal

Bring a process from empty disk → green readiness → serving search/ask.

## 1. Prepare config (optional)

```bash
cp config.example.toml config.toml
# Fill [siliconflow].api_key for production embeddings (BAAI/bge-m3, 1024-dim).
# Optional [cohere].api_key for chat/rerank only (missing → offline ask + skip rerank).
# Optional [server] cors_origins / api_token / max_concurrent_asks.
```

Env overrides (preferred for secrets):

| Concern | CLI | Env |
|--------|-----|-----|
| CORS allowlist | `--cors-origins` | `MAO_CORS_ORIGINS` |
| API bearer token | `--api-token` | `MAO_API_TOKEN` |
| Ask concurrency | `--max-concurrent-asks` | `MAO_MAX_CONCURRENT_ASKS` |
| SiliconFlow embed key | `--embed-api-key` / `--embed-provider siliconflow` | `SILICONFLOW_API_KEY` |
| Cohere chat/rerank key | `--api-key` | `COHERE_API_KEY` |

See ADR 0004 (CORS), ADR 0005 (auth + concurrency).

## 2. Ingest

```bash
cargo run -- init-samples          # or point at your corpus/
cargo run -- ingest --offline      # offline deterministic embed for smoke (512-dim)
# production SiliconFlow (1024-dim). Free-tier ~2000 RPM / 500k TPM:
# keep --batch-size 16~32; remote batches are paced 100ms apart.
cargo run -- ingest --embed-provider siliconflow --batch-size 16
```

Artifacts: `data/vector_store.bin`, `data/tantivy_index/`. Dimension must match the embedder used at ingest (SiliconFlow 1024 ≠ offline 512 ≠ Gemini 768).

## 2b. Knowledge graph (optional)

```bash
py scripts/build_knowledge_graph.py --mock --output data/graph_store.json
cargo run --no-default-features -- ingest-graph --input data/graph_store.json --output data/graph_store.bin
```

- Missing `data/graph_store.bin` is a no-op: hybrid prints a one-line notice and continues dual BM25+Vector.
- Cross-domain smoke (`主要矛盾与阿姆达尔定律` → Mao + Amdahl titles) requires a **full** ingest; the default 59-chunk sample has no engineering docs (see `evals/retrieval/GRAPH.md`).

## 3. Health green

```bash
cargo run -- serve --offline --bind 127.0.0.1:3000
curl -sS http://127.0.0.1:3000/live     # always 200 if process up
curl -sS -o /dev/null -w "%{http_code}\n" http://127.0.0.1:3000/health
# expect 200 once index has vectors; 503 if empty
```

- `/live` = liveness (restart probe)
- `/health` = readiness (traffic probe) — ADR 0002

## 4. Serve (intranet)

```bash
# Loopback, no token — open for local dev
cargo run -- serve --bind 127.0.0.1:3000

# Non-loopback: set a token
export MAO_API_TOKEN=replace-me
export MAO_CORS_ORIGINS=http://intranet-app.local:5173
cargo run -- serve --bind 0.0.0.0:8080
curl -H "Authorization: Bearer replace-me" \
  -H "Content-Type: application/json" \
  -d '{"query":"持久战","top_k":3,"mode":"hybrid"}' \
  http://127.0.0.1:8080/api/v1/search
```

Metrics: `GET /metrics` (Prometheus), `GET /api/v1/metrics` (JSON). Includes `mao_llm_fallback_total` when online LLM falls back offline.

## 5. Shutdown

Ctrl+C / SIGTERM triggers graceful drain (`GracefulShutdown`).
