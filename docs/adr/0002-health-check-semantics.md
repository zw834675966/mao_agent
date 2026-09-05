# ADR 0002 — Health check semantics

## Context

`GET /health` and `GET /api/v1/health` always returned HTTP 200 with `status: "ok"` while exposing `index_loaded` as a boolean. Load balancers and intranet probes that only look at status codes would route traffic to a process that cannot answer search/ask usefully (empty vector store).

## Decision

Health is **fail-closed on index readiness**:

- If `total_vectors == 0` → HTTP **503** `SERVICE_UNAVAILABLE`, `status: "unavailable"`, `index_loaded: false`
- If `total_vectors > 0` → HTTP **200**, `status: "ok"`, `index_loaded: true`

Tantivy presence remains informational (`tantivy_loaded`) and does not alone determine the status code in this ADR (vector index is the primary readiness gate for B-grade).

## Consequences

- Orchestrators can rely on HTTP status alone for readiness.
- Fresh processes before ingest/load will intentionally fail probes until indexes are present.
- `/api/v1/stats` stays a separate diagnostic endpoint and is not required to return 503.
