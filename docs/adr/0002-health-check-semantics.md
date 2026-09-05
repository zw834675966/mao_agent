# ADR 0002 — Health check semantics

## Context

`GET /health` and `GET /api/v1/health` always returned HTTP 200 with `status: "ok"` while exposing `index_loaded` as a boolean. Load balancers and intranet probes that only look at status codes would route traffic to a process that cannot answer search/ask usefully (empty vector store).

Orchestrators often need **both** liveness (process up) and readiness (able to serve).

## Decision

Split probes:

### Liveness — `GET /live` / `GET /api/v1/live`

- Always HTTP **200** if the process is up.
- Body may still report `index_loaded` / vector counts for humans; HTTP status does **not** depend on the index.

### Readiness — `GET /health` / `GET /api/v1/health` (unchanged contract)

Fail-closed on index readiness:

- If `total_vectors == 0` → HTTP **503** `SERVICE_UNAVAILABLE`, `status: "unavailable"`, `index_loaded: false`
- If `total_vectors > 0` → HTTP **200**, `status: "ok"`, `index_loaded: true`

Tantivy presence remains informational (`tantivy_loaded`) and does not alone determine the readiness status code (vector index is the primary readiness gate for B-grade).

## Consequences

- Orchestrators can use `/live` for restart decisions and `/health` for traffic routing.
- Existing `/health` clients keep the Cycle 8 readiness contract.
- Fresh processes before ingest/load fail readiness until indexes are present, while liveness stays green.
- `/api/v1/stats` stays a separate diagnostic endpoint and is not required to return 503.
