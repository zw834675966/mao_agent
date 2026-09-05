# ADR 0005 — API bearer auth and ask concurrency

## Context

B-grade small-team intranet may bind beyond loopback. Without a shared secret, search/ask are open on the LAN. Unbounded concurrent `/ask` / SSE streams can exhaust LLM/upstream and memory.

## Decision

### Optional bearer auth

- Configure via `--api-token` / env `MAO_API_TOKEN` / `config.toml` `[server].api_token`.
- **When a token is configured**, protected routes require `Authorization: Bearer <token>` (exact match). Missing/wrong → HTTP **401**.
- **When no token is configured**, auth is off (loopback local-dev default stays open).
- Public (unauthenticated even with token): `/live`, `/api/v1/live`, `/health`, `/api/v1/health`, `/metrics`, `/api/v1/metrics`, `/api/v1/stats`.
- Protected: `/api/v1/search`, `/api/v1/ask`, `/api/v1/ask/stream`, `/api/v1/verify`, `/api/v1/citation/verify`.
- When API auth is enabled, `Authorization: Bearer` is reserved for the API token; Cohere/LLM keys must come from request body `api_key` or server config (not the Bearer header).

### Ask concurrency limit

- Configure via `--max-concurrent-asks` / env `MAO_MAX_CONCURRENT_ASKS` / `[server].max_concurrent_asks` (CLI/env default **32**).
- Implemented with a `tokio::sync::Semaphore` on `AppState`.
- Exceeded → HTTP **429** on `/api/v1/ask` and `/api/v1/ask/stream`.
- Permit is held for the full SSE stream lifetime.

## Non-goals (YAGNI)

- Cookie / credentials-mode CORS SPA auth.
- Public-internet TLS, global rate limits, audit logging (out of B scope).
- mTLS.

## Consequences

- Intranet binds can be gated with a shared secret without rewriting handlers.
- Operators must pass the bearer token from clients when `MAO_API_TOKEN` is set.
- Probes remain usable without the token.
