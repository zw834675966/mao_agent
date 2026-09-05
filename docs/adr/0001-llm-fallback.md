# ADR 0001 — LLM fallback when online call fails

## Context

`DialecticalAgent` historically called the Cohere-compatible chat API whenever an API key was present, and only used the offline dialectical template when **no** key was set. For small-team intranet (B-grade) operation, a configured key does not guarantee upstream availability: timeouts, 5xx, auth blips, or network partitions would turn `/api/v1/ask` into hard errors even though retrieved evidence was already in hand.

## Decision

Introduce an `LlmClient` trait with:

- `OnlineLlmClient` — OpenAI-compatible `chat/completions`
- `OfflineLlmClient` — deterministic four-stage dialectical template (static `generate`)
- `FallbackLlmClient` — prefer online when a key exists; on **any** online failure, log a warning and return the offline template

`DialecticalAgent` holds `Arc<dyn LlmClient>` constructed via `FallbackLlmClient::from_api_key(...)`.

## Consequences

- Ask/SSE remain available under degraded LLM conditions (answer quality drops to template, but citations still verify against retrieved chunks).
- Operators must treat offline-template answers as degraded mode (monitor logs for fallback warnings).
- Tests can assert fallback with `wiremock` 500 without network to Cohere.
