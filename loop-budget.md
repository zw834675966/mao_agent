# Loop Budget — mao_agent

Daily ceilings for autonomous L1 loops. Agents must read this file before calling a remote embed/chat/rerank provider. These are **policy caps**, not measured spend.

## Daily quota

| Provider | CLI surface | Max calls/day | Max tokens/day | Fallback at ≥80% of either cap |
|----------|-------------|---------------|----------------|--------------------------------|
| SiliconFlow embed (`BAAI/bge-m3`, 1024-dim) | `ingest` / `search` / `ask` `--embed-provider siliconflow` | 1 800 | 400 000 | `--offline` (DeterministicEmbedder 512-dim; rebuild index if dim changes) |
| Gemini embed (`gemini-embedding-2`, 768-dim) | `--embed-provider gemini` | 400 | 160 000 | SiliconFlow if a key remains; else `--offline` |
| Cohere chat (`command-r7b-12-2024`) | `ask` / `serve` | 80 | 160 000 | `generate_offline_dialectical_answer` (no network) |
| Cohere rerank (`rerank-v3.5`) | `search` / `ask` / `serve` | 80 | n/a (docs in, scores out) | `--no-rerank` |
| L1 inspect | `scripts/loop_l1_inspect.py` | 4 | 0 remote | always `--offline --no-rerank`; never call SiliconFlow/Cohere/Gemini |

Free-tier SiliconFlow pacing already in the CLI: `--batch-size` 16–32, remote batches 100 ms apart (~2000 RPM / 500k TPM). Loop caps sit **below** that so a sweeper cannot burn the account.

## Degrade protocol (≥80%)

1. Set subsequent retrieval/ask flags to `--offline --no-rerank`.
2. Do not rebuild a SiliconFlow/Gemini index from an L1 loop (dim would diverge).
3. Record the degrade in `STATE.md` High Priority (`- [L1] budget 80% — offline`).
4. Resume remote providers only after a human clears the High Priority row.

## Kill switch

- Env: `MAO_LOOP_PAUSE=1` — L1 inspect must exit 2 immediately (escalate, no eval, no `STATE.md` write).
- Resume: unset the env and note the resume in `STATE.md` Last run.

## Estimate (no extra tooling)

L1 `--check-integrity` is local (`cargo check` + filesystem). L1 `--full` adds offline `eval-retrieval` and `retrieval_hard_eval_test` — zero billed tokens when `--offline --no-rerank` holds.
