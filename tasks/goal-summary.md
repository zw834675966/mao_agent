# Goal Summary

Delivered Cycle 14 (Pi Coding Agent MCP integration hardening) on the `mao_agent` Rust project (vector DB + dialectical retrieval engine): Pi contract tests, doc/probe retargeting from DSH, and client-mount governance, executed via agent-skills with Claude Code final acceptance (verdict: GO).

## What changed (Cycle 14)

- `tests/mcp_test.rs`: 3 Pi-client contract tests (handshake, `query_dialectical_principles`, `verify_historical_citation`); dispatcher stays client-agnostic (no `clientInfo` branch).
- `src/mcp/stdio.rs`: docstring retargeted to Pi Coding Agent via `pi-mcp-adapter` (runtime unchanged).
- `scripts/verify_pi_stdio.py` / `scripts/verify_pi_http.py`: stdlib-only probes (both exit 0).
- Docs: `docs/pi/` (README + `mcp.json` + persona), `docs/ops/mcp_sre_guide.md` retargeted, `docs/dsh/*` deprecated with banners (bodies kept), `docs/ops/runbook.md` §6, `AGENTS.md` client-mount line.

## How to use it

- Run the gates: `cargo fmt --check`, `cargo clippy --no-default-features --all-targets -- -D warnings`, `cargo test --no-default-features` (212 tests, all green).
- Verify Pi mounts: `python scripts/verify_pi_http.py --mock` and `python scripts/verify_pi_stdio.py`.
- Start the HTTP API: `cargo run -- serve --offline --bind 127.0.0.1:3000`, then `POST /api/v1/search` / `/api/v1/ask/stream`.
- Run the MCP stdio server: `cargo run -- mcp --offline`.
- Read the governance artifacts: `tasks/plan.md` / `tasks/todo.md` (Cycle 14), `docs/pi/README.md` (primary mount), `docs/ops/mcp_sre_guide.md`; DSH files under `docs/dsh/` are archived reference only.
