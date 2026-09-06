# AGENTS.md

## Overview

Single-crate Rust project (bin + lib, no workspace): vector database + retrieval agent engine for a Chinese historical corpus (Mao's writings). Rust **2024 edition**.

- Entry: `src/main.rs` (CLI), `src/lib.rs` (public API)
- Modules: `corpus` (markdown parse / CJK clean / semantic chunk), `vector` (embedders, store, HNSW ANN index), `index` (Tantivy BM25 + hybrid RRF fusion), `graph` (DiGraph knowledge graph + query expansion), `rerank` (Cohere Reranker + fallback), `retry` (bounded exponential backoff), `eval` (Recall/MRR/NDCG@k), `agent` (LLM dialectical reasoning + LlmClient fallback + citation verifier), `server` (Axum REST + SSE + request-id/metrics/CORS), `cli` (clap definitions), `model` (shared types)
- Docs & CI: `README.md`, CI workflow in `.github/workflows/ci.yml`; 134 tests under `--no-default-features`
- Customizations: Project-specific Antigravity customizations (Skills, Rules, Hooks) reside in `.agents/`

## Commands

```bash
cargo build                                # debug build (default features)
cargo test                                 # run all tests (slow first build: compiles fastembed)
cargo test --no-default-features           # skip fastembed dep tree — tests still all pass
cargo test --no-default-features <name>    # single test, e.g. test_persistence_atomic_save_and_reload
cargo test --no-default-features --test vector_store_test   # single integration file
```

Official CI code quality gates:

```bash
cargo fmt --check
cargo clippy --no-default-features --all-targets -- -D warnings
cargo test --no-default-features
```

Use `--no-default-features` for routine test runs. Tests use `DeterministicEmbedder` (hash-based, no network); only the CLI binary path uses FastEmbed, Gemini, or Cohere.

## Features

`default = ["local-embed"]` pulls in `fastembed` (ONNX BGE-small-zh-v1.5). `FastEmbedder::try_new()` **downloads the model on first use** (network required). Embedder chain: `--offline` → Deterministic 512-dim (no `.embedcache`, ignores provider config dim); else `--embed-provider` wins; else auto-select **SiliconFlow first** (`SILICONFLOW_API_KEY` / `config.toml [siliconflow]`, `BAAI/bge-m3` 1024-dim at `https://api.siliconflow.cn/v1`) then Gemini (`gemini-embedding-2` 768-dim) then Cohere `embed-v4.0` 1536-dim; else FastEmbed. Remote paths wrap `{index_file}.embedcache`. **FastEmbed init failure is a hard error** (no silent hash fallback). Without `local-embed` and without a key, the CLI errors and asks for `--offline` or a SiliconFlow/Gemini/Cohere key. Never assume a test needs an external model or key. Bulk SiliconFlow ingest: `--batch-size` 16~32 (default 32); remote batches paced 100ms (free-tier ~2000 RPM / 500k TPM).

## CLI binary flow

`cargo run -- ` subcommands: `init-samples`, `ingest`, `ingest-graph`, `search`, `stats`, `ask`, `serve`, `eval-retrieval`.

- Ingest order matters: `init-samples` (creates `corpus/*.md`) → `ingest` (builds `data/vector_store.bin` + `data/tantivy_index/`) → optional `python scripts/build_knowledge_graph.py --mock` + `ingest-graph` (builds `data/graph_store.bin`) → `search`/`ask`/`eval-retrieval`/`serve` read those artifacts. Search commands error politely if indexes are missing. Missing `--graph-file` is a no-op (dual BM25+Vector only).
- `data/` artifacts are gitignored build outputs; regenerate via `cargo run -- ingest` after corpus changes. Don't hand-edit.
- Key resolution:
  - SiliconFlow (production embed): `--embed-api-key` / `--embed-provider siliconflow` → `SILICONFLOW_API_KEY` → `EMBED_API_KEY` → `config.toml [siliconflow].api_key` (model `BAAI/bge-m3`, 1024-dim). Does **not** fall through to `COHERE_API_KEY`.
  - Gemini: `--gemini-api-key` → `GEMINI_API_KEY` → `config.toml [gemini].api_key` (model `gemini-embedding-2`, default 768-dim)
  - Cohere chat/rerank only: `--api-key` → `COHERE_API_KEY` → `config.toml [cohere].api_key`. Copy `config.example.toml` → `config.toml` (gitignored). Never commit keys. `ask` default model `command-r7b-12-2024`; without a key it falls back to `generate_offline_dialectical_answer`.
- `search --mode` accepts `hybrid` (default, RRF fusion → optional graph candidate expansion → optional Cohere rerank-v3.5 → top_k), `vector`, `bm25`. There is **no** `--mode graph`. `--graph-file` (default `data/graph_store.bin`) expands hybrid candidates via 1–2 hop `DiGraph`; vector/bm25 modes ignore it. Use `--no-rerank` / `--rerank-model` / `COHERE_RERANK_MODEL`; offline or missing key skips rerank.
- Vector ANN: HNSW (`hnswlib-rs` / hnsw-stable) activates at ≥5000 vectors; snapshot does not persist the graph (rebuild on load).
- `eval-retrieval`: offline Recall/MRR/NDCG@k over `evals/retrieval/queries.jsonl` (~100+); `--force-brute` forces exact vector scan (disables HNSW) for recall comparison — **eval-retrieval only**, not on `search`; see `evals/retrieval/BASELINE.md`.
- `serve`: Axum REST + SSE; ask stream events `retrieved → reranked → delta → citation → done`. Ops: `X-Request-Id`, `GET /live` (liveness) + `/health` (readiness), `GET /metrics` + `/api/v1/metrics` (incl. `mao_llm_fallback_total`), CORS allowlist (`--cors-origins` / `MAO_CORS_ORIGINS`), optional bearer auth (`--api-token` / `MAO_API_TOKEN`), ask concurrency (`--max-concurrent-asks` / `MAO_MAX_CONCURRENT_ASKS`, default 32), Cohere chat/rerank retries then offline fallback. Runbook: `docs/ops/runbook.md`.

## Corpus / data conventions

- Corpus docs: Chinese markdown with YAML frontmatter (title/author/date/period/volume/category). Sample generator: `cargo run -- init-samples`.
- Additional domains: `corpus/hacker_laws/` (programming laws & design principles), `corpus/papers_we_love/` (classic CS papers with original PDFs in `raw/`), `corpus/awesome_scalability/` (system architecture), `corpus/hello_algo/` (algorithms & data structures).
- `scripts/build_corpus.py`: hardened preprocessing pipeline (path-traversal / YAML-injection guards, CJK OCR whitespace cleanup). Python 3, stdlib only.
- `scripts/build_knowledge_graph.py`: stdlib extractor (`--mock` deterministic rules, or `--api-key/--base-url/--model` OpenAI-compatible). Emits `evals/graph/golden_graph.json` / `data/graph_store.json` with `source_refs.doc_title` (never ingest-hash `chunk_id`). `ingest-graph` compiles JSON → `data/graph_store.bin`.
- Index format: bincode snapshot (`vector_store.bin`) + Tantivy directory. Dimensions must match across save/load. Gemini `gemini-embedding-2` is 768-dim (default); Cohere `embed-v4.0` is 1536-dim; local BGE-small-zh-v1.5 is 512-dim. Never commit API keys.

## Testing notes

- Integration tests in `tests/` (10 files: api, chunker, config, e2e_ingest, graph_expand, graph_store, hnsw_regression, hybrid_and_agent, retrieval_hard_eval, vector_store) use `tempfile::tempdir()`; no fixtures, no external services, no network. Full suite **134 tests** with `--no-default-features`.
- Vector dim in tests varies (64/128/256) — construct stores via `VectorStore::new_deterministic(dim)` rather than copying CLI's 512 constant.
- Rerank unit tests use mocks only (no Cohere network). Citation adversarial suite expects 100% reject on synonym/reorder/fabricated/cross-doc/noise.
