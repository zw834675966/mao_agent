# AGENTS.md

## Overview

Single-crate Rust project (bin + lib, no workspace): vector database + retrieval agent engine for a Chinese historical corpus (Mao's writings). Rust **2024 edition**.

- Entry: `src/main.rs` (CLI), `src/lib.rs` (public API)
- Modules: `corpus` (markdown parse / CJK clean / semantic chunk), `vector` (embedders, store, HNSW ANN index), `index` (Tantivy BM25 + hybrid RRF fusion), `rerank` (Cohere Reranker + fallback), `eval` (Recall/MRR/NDCG@k), `agent` (LLM dialectical reasoning + citation verifier), `server` (Axum REST + SSE), `cli` (clap definitions), `model` (shared types)
- Docs & CI: `README.md`, CI workflow in `.github/workflows/ci.yml`; ~77 tests under `--no-default-features`
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

Use `--no-default-features` for routine test runs. Tests use `DeterministicEmbedder` (hash-based, no network); only the CLI binary path uses FastEmbed or Cohere.

## Features

`default = ["local-embed"]` pulls in `fastembed` (ONNX BGE-small-zh-v1.5). `FastEmbedder::try_new()` **downloads the model on first use** (network required). Embedder chain: `--offline` → Deterministic (no `.embedcache`); CLI/env/`config.toml` Cohere key → `embed-v4.0` at `https://api.cohere.ai/compatibility/v1` (1536-dim) wrapped with `{index_file}.embedcache`; else FastEmbed; **FastEmbed init failure is a hard error** (no silent hash fallback). Without `local-embed` and without a key, the CLI errors and asks for `--offline` or a Cohere key. Never assume a test needs the ONNX model or a Cohere key.

## CLI binary flow

`cargo run -- ` subcommands: `init-samples`, `ingest`, `search`, `stats`, `ask`, `serve`, `eval-retrieval`.

- Ingest order matters: `init-samples` (creates `corpus/*.md`) → `ingest` (builds `data/vector_store.bin` + `data/tantivy_index/`) → `search`/`ask`/`eval-retrieval`/`serve` read those artifacts. Search commands error politely if indexes are missing.
- `data/` artifacts are gitignored build outputs; regenerate via `cargo run -- ingest` after corpus changes. Don't hand-edit.
- Cohere key resolution: `--api-key` / `--embed-api-key` → `COHERE_API_KEY` / `EMBED_API_KEY` → `config.toml` `[cohere].api_key`. Copy `config.example.toml` → `config.toml` (gitignored). Never commit the key. `ask` default model `command-r7b-12-2024`; without a key it falls back to `generate_offline_dialectical_answer`.
- `search --mode` accepts `hybrid` (default, RRF fusion → optional Cohere rerank-v3.5 → top_k), `vector`, `bm25`. Use `--no-rerank` / `--rerank-model` / `COHERE_RERANK_MODEL`; offline or missing key skips rerank.
- Vector ANN: HNSW (`hnswlib-rs` / hnsw-stable) activates at ≥5000 vectors; snapshot does not persist the graph (rebuild on load); `--force-brute` forces exact scan for recall comparison.
- `eval-retrieval`: offline Recall/MRR/NDCG@k over `evals/retrieval/queries.jsonl` (~100+); see `evals/retrieval/BASELINE.md`.
- `serve`: Axum REST + SSE; ask stream events `retrieved → reranked → delta → citation → done`.

## Corpus / data conventions

- Corpus docs: Chinese markdown with YAML frontmatter (title/author/date/period/volume/category). Sample generator: `cargo run -- init-samples`.
- `scripts/build_corpus.py`: hardened preprocessing pipeline (path-traversal / YAML-injection guards, CJK OCR whitespace cleanup). Python 3, stdlib only.
- Index format: bincode snapshot (`vector_store.bin`) + Tantivy directory. Dimensions must match across save/load. Cohere `embed-v4.0` is 1536-dim; local BGE-small-zh-v1.5 is 512-dim. Compatibility API does not accept `dimensions` — do not send it. Never commit API keys.

## Testing notes

- Integration tests in `tests/` (7 files: api, chunker, config, e2e_ingest, hnsw_regression, hybrid_and_agent, vector_store) use `tempfile::tempdir()`; no fixtures, no external services, no network. Full suite ≈77 tests with `--no-default-features`.
- Vector dim in tests varies (64/128/256) — construct stores via `VectorStore::new_deterministic(dim)` rather than copying CLI's 512 constant.
- Rerank unit tests use mocks only (no Cohere network). Citation adversarial suite expects 100% reject on synonym/reorder/fabricated/cross-doc/noise.
