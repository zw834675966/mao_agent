# ADR 0004 — Configurable CORS allowlist (B-grade P1)

## Context

ADR 0003 accepted a permissive `allow_origin(Any)` CORS layer for early intranet SPA prototypes. For B-grade small-team intranet, operators still want a **default-deny against arbitrary browser origins** while keeping local development frictionless.

## Decision

1. Replace `AllowOrigin::any()` with an **explicit allowlist**.
2. **Default allowlist** (when CLI/env/config unset):  
   `http://localhost:3000`, `http://127.0.0.1:3000`, `http://localhost:5173`, `http://127.0.0.1:5173`, `http://localhost:8080`, `http://127.0.0.1:8080`.
3. Configuration precedence: `--cors-origins` / `MAO_CORS_ORIGINS` (comma-separated) → `config.toml` `[server].cors_origins` → localhost defaults.
4. Allowed methods remain GET/POST/OPTIONS; allowed headers include `content-type`, `authorization`, `x-request-id`.
5. Non-browser clients (no `Origin` header) are unaffected.

## Consequences

- Browser calls from disallowed origins do not receive `Access-Control-Allow-Origin`.
- Intranet teams add their SPA origin via env/CLI/config without code changes.
- Public-internet posture (credentials mode, authn) remains out of B scope (see gap list P2/P3).

## Related

- Extends ADR 0003 (bind + CORS tradeoff).
- Implemented by `server::cors::CorsAllowlist`.
