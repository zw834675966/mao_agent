# Spec: New-user README and offline embed defaults

Confirmed intent (interview-me, explicit yes): a clone-and-run user without a Cohere key can follow README through `init-samples → ingest → search` without mixing embedder backends or hitting a default-dimension trap. Docs match code.

## Objective

- **Who:** Someone who just cloned the repo and has no API key.
- **Why:** README still says “本地/离线嵌入器” while the command omits `--offline` (FastEmbed 512). CLI `--embed-dim` defaults to Cohere 1536. Mixing `--offline` on only one of ingest/search raises `IdentityMismatch`.
- **Success:** README numbers and commands match `src/`. `--offline` without an explicit `--embed-dim` uses 512 (same as local BGE). Remote/Cohere default stays 1536. Explicit `--embed-dim` always wins.

## Tech Stack

Existing crate: Rust 2024, clap 4.5, `mao_agent` 0.1.0. No new dependencies.

## Commands

```
cargo test --no-default-features
cargo test --no-default-features test_resolve_embed_dimension
cargo fmt --check
cargo clippy --no-default-features --all-targets -- -D warnings
```

## Project Structure

- `src/vector/embedder/mod.rs` — dimension default helper + unit tests
- `src/cli/mod.rs` — `--embed-dim` optional
- `src/main.rs` — wire helper into `get_embedder`
- `src/lib.rs` / `src/vector/mod.rs` — re-export
- `README.md` — user-facing commands

## Code Style

Match existing embedder helpers (`resolve_embedder`, `join_openai_path`): one small function, no new types.

## Testing Strategy

Unit tests in `src/vector/embedder/mod.rs` (`#[cfg(test)]`). Prove:

1. `--offline`, no explicit dim → `LOCAL_EMBEDDING_DIM` (512)
2. not offline, no explicit dim → `COHERE_EMBEDDING_DIM` (1536)
3. explicit `Some(n)` wins in both modes

No network. `cargo test --no-default-features`.

## Boundaries

- **Always:** Keep `cargo test --no-default-features` green; identity mismatch still errors on mixed backends (hash vs FastEmbed vs Cohere).
- **Ask first:** Changing Cohere default dim; adding dependencies; CI cache.
- **Never:** Rewrite RRF / Agent / Verifier; silent hash fallback; commit secrets.

## Success Criteria

- [x] `resolve_embed_dimension` tests exist and pass
- [x] CLI: omitted `--embed-dim` + `--offline` → 512; omitted without `--offline` → 1536
- [x] README test count matches `cargo test --no-default-features` (currently 44, not 34)
- [x] README ingest/search examples use the same backend; no-network path includes `--offline` on both
- [x] Mixing backends still fails closed (`IdentityMismatch`) — not changed

## Out of scope

CI cache, unknown `--mode` falling through to hybrid, BM25 fail-open in Ask, citation Jaro-Winkler threshold, embedder algorithm rewrite, `AGENT_INSTRUCTIONS.md` (stale ingest line noted, not this spec).

## Open Questions

None — intent confirmed.
