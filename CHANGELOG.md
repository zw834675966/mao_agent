# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Multi-domain taxonomy: `Domain::History` and `Domain::Engineering` with explicit filtering and auto-inference.
- Corpus directory partitioning separating historical texts (`corpus/history/`) and engineering literature (`corpus/engineering/`).
- Default author attribution based on domain and category rules.
- Pre-commit hygiene hardening in `.gitignore` covering `.env*` credentials and OS artifacts.

### Changed
- Refactored knowledge graph query expansion to preserve candidate pool rankings with bonus scoring.
- Aligned MCP dispatcher tool definitions with multi-domain taxonomy and citation verification.

## [0.1.0] - 2026-09-07

### Added
- Dialectical reasoning engine with offline four-stage reasoning structure and LLM fallback.
- Hybrid retrieval engine combining Tantivy BM25 full-text search with dense vector HNSW ANN indexing via RRF fusion.
- Multi-domain knowledge graph (`DiGraph`) with seed matching and 1–2 hop query expansion.
- Model Context Protocol (MCP 2024-11-05) support with stdio JSON-RPC server and Axum HTTP endpoint (`/mcp`).
- Historical citation grounding verifier with adversarial quote rejection.
- Production operations features:
  - Liveness (`/live`) and readiness (`/health`) HTTP probes.
  - Prometheus metrics reporting (`/metrics`).
  - `X-Request-Id` tracing propagation.
  - Configurable CORS origin allowlist and bearer token authentication.
  - Ask query concurrency control limiting simultaneous agent runs.
- Multi-gate CI pipeline enforcing rustfmt, clippy (`-D warnings`), and 165+ offline unit and integration tests.
