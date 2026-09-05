# Implementation Plan: New-user README and offline embed defaults

## Overview

Align CLI `--offline` default dimension with local BGE (512) when `--embed-dim` is omitted, and make README commands use one backend end-to-end.

## Architecture Decisions

- Detect “user omitted `--embed-dim`” with `Option<usize>` (clap). A hardcoded `default_value_t = 1536` cannot distinguish omit vs explicit 1536.
- Put `resolve_embed_dimension(offline, Option<usize>) -> usize` next to the existing dim constants. CLI and tests share it. No new trait.
- Do not change FastEmbed or Cohere construction. FastEmbed still uses model width; this only affects DeterministicEmbedder via `EmbedderSelection.dimension`.
- Mixed backends remain an error (`IdentityMismatch` on model name). Matching 512 does not make hash and BGE interchangeable.

## Task List

### Phase 1: Foundation (TDD)

- [x] Task 1: `resolve_embed_dimension` + unit tests (RED → GREEN)

### Checkpoint: Foundation

- [x] Focused tests pass

### Phase 2: Wire + docs

- [x] Task 2: CLI `Option<usize>` + `get_embedder`
- [x] Task 3: README ingest/search/`--offline` + test count 56

### Checkpoint: Complete

- [x] `cargo test --no-default-features`
- [x] `cargo fmt --check`
- [x] `cargo clippy --no-default-features --all-targets -- -D warnings`

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Existing `--offline` snapshots at 1536 | Med | Explicit `--embed-dim 1536` still works; CLI already prints IdentityMismatch + re-ingest hint |
| README over-explains three backends | Low | Two copy-paste recipes: `--offline` pair, then FastEmbed pair |

## Open Questions

None.
