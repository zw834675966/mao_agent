# Goal Summary

Delivered a full-chain agent software-engineering governance pass on the `mao_agent` Rust project (vector DB + dialectical retrieval engine), executed via agent-skills + subagents on third-party models, with the MCP dual-channel (stdio + Streamable HTTP) engine and DSH integration verified complete.

## How to use it

- Run the gates: `cargo fmt --check`, `cargo clippy --no-default-features --all-targets -- -D warnings`, `cargo test --no-default-features` (165 tests, all green).
- Start the HTTP API: `cargo run -- serve --offline --bind 127.0.0.1:3000`, then `POST /api/v1/search` / `/api/v1/ask/stream`.
- Run the MCP stdio server: `cargo run -- mcp --offline`.
- Read the governance artifacts: `tasks/verification_report.md`, `docs/ops/mcp_sre_guide.md`, `docs/dsh/cordis.patch.example.yml`.