# Tasks: New-user README and offline embed defaults

- [x] Task 1: `resolve_embed_dimension`
  - Acceptance: offline+None → 512; online+None → 1536; `Some(n)` wins
  - Verify: `cargo test --no-default-features test_resolve_embed_dimension`
  - Files: `src/vector/embedder/mod.rs`, re-exports
  - Scope: S

- [x] Task 2: CLI + `get_embedder`
  - Acceptance: `--embed-dim` optional; omitted uses Task 1 helper
  - Verify: same test command + `cargo clippy --no-default-features --all-targets -- -D warnings`
  - Files: `src/cli/mod.rs`, `src/main.rs`, `src/lib.rs`, `src/vector/mod.rs`
  - Scope: S
  - Dependencies: Task 1

- [x] Task 3: README
  - Acceptance: test count matches suite (44); ingest/search same backend; no-network path has `--offline` on both
  - Verify: grep README for `34` (zero hits) and `--offline` on ingest/search examples
  - Files: `README.md`
  - Scope: XS
  - Dependencies: Task 2 (document actual flags)

## Checkpoint

- [x] `cargo test --no-default-features` (44 passed, tester GO)
- [x] `cargo fmt --check`
- [x] `cargo clippy --no-default-features --all-targets -- -D warnings`
