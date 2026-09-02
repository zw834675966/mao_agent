# AGENTS.md

## Overview

Single-crate Rust project (bin + lib, no workspace): vector database + retrieval agent engine for a Chinese historical corpus (Mao's writings). Rust **2024 edition**.

- Entry: `src/main.rs` (CLI), `src/lib.rs` (public API)
- Modules: `corpus` (markdown parse / CJK clean / semantic chunk), `vector` (embedders, store, index), `index` (Tantivy BM25 + hybrid RRF fusion), `agent` (LLM dialectical reasoning + citation verifier), `cli` (clap definitions), `model` (shared types)
- No README, no CI, no commits yet on `master`

## Commands

```bash
cargo build                                # debug build (default features)
cargo test                                 # run all tests (slow first build: compiles fastembed)
cargo test --no-default-features           # skip fastembed dep tree — tests still all pass
cargo test --no-default-features <name>    # single test, e.g. test_persistence_atomic_save_and_reload
cargo test --no-default-features --test vector_store_test   # single integration file
```

Use `--no-default-features` for routine test runs. Tests use `DeterministicEmbedder` (hash-based, no network); only the CLI binary path uses FastEmbed or Cohere.

## Features

`default = ["local-embed"]` pulls in `fastembed` (ONNX BGE-small-zh-v1.5). `FastEmbedder::try_new()` **downloads the model on first use** (network required). Embedder chain: `--offline` → Deterministic (no `.embedcache`); CLI/env/`config.toml` Cohere key → `embed-v4.0` at `https://api.cohere.ai/compatibility/v1` (1536-dim) wrapped with `{index_file}.embedcache`; else FastEmbed; **FastEmbed init failure is a hard error** (no silent hash fallback). Without `local-embed` and without a key, the CLI errors and asks for `--offline` or a Cohere key. Never assume a test needs the ONNX model or a Cohere key.

## CLI binary flow

`cargo run -- ` subcommands: `init-samples`, `ingest`, `search`, `stats`, `ask`.

- Ingest order matters: `init-samples` (creates `corpus/*.md`) → `ingest` (builds `data/vector_store.bin` + `data/tantivy_index/`) → `search`/`ask` read those artifacts. Search commands error politely if indexes are missing.
- `data/` artifacts are committed outputs; regenerate via `cargo run -- ingest` after corpus changes. Don't hand-edit.
- Cohere key resolution: `--api-key` / `--embed-api-key` → `COHERE_API_KEY` / `EMBED_API_KEY` → `config.toml` `[cohere].api_key`. Copy `config.example.toml` → `config.toml` (gitignored). Never commit the key. `ask` default model `command-r7b-12-2024`; without a key it falls back to `generate_offline_dialectical_answer`.
- `search --mode` accepts `hybrid` (default, RRF fusion), `vector`, `bm25`.

## Corpus / data conventions

- Corpus docs: Chinese markdown with YAML frontmatter (title/author/date/period/volume/category). Sample generator: `cargo run -- init-samples`.
- `scripts/build_corpus.py`: hardened preprocessing pipeline (path-traversal / YAML-injection guards, CJK OCR whitespace cleanup). Python 3, stdlib only.
- Index format: bincode snapshot (`vector_store.bin`) + Tantivy directory. Dimensions must match across save/load. Cohere `embed-v4.0` is 1536-dim; local BGE-small-zh-v1.5 is 512-dim. Compatibility API does not accept `dimensions` — do not send it. Never commit API keys.

## Testing notes

- Integration tests in `tests/` use `tempfile::tempdir()` for persistence tests; no fixtures, no external services, no network.
- Vector dim in tests varies (64/128/256) — construct stores via `VectorStore::new_deterministic(dim)` rather than copying CLI's 384 constant.
