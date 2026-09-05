# ADR 0003 — Bind address and CORS for B-grade intranet

## Context

The `serve` CLI defaults to binding `127.0.0.1:3000` and the Axum router installs a permissive CORS layer (`allow_origin(Any)`, broad methods/headers) to unblock local SPA / intranet frontends during early adoption.

## Decision

For production target **B (small-team intranet)**:

1. **Default bind remains loopback** (`127.0.0.1:3000`). Operators who need LAN access must explicitly pass `--bind 0.0.0.0:<port>` (or a host firewall-controlled interface).
2. **CORS**: P0 used permissive `Any` for prototypes; **superseded for allowlist by ADR 0004** (localhost defaults + configurable list). Bind guidance below still stands.
3. Moving to public exposure requires a follow-up ADR (authn, tighter CORS allowlist, TLS termination, rate limits) — out of scope for B P0.

## Consequences

- Accidental public bind is an operator action, not the default.
- Intranet browsers/tools can call the API without CORS friction.
- Gap list retains P2/P3 items for auth and CORS lockdown if the deployment boundary expands beyond trusted intranet.


## Update

See **ADR 0004** for the P1 CORS allowlist decision.
