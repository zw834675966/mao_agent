# Loop State — mao_agent

Last run: 2026-09-09T10:31:47Z (full)
Mode: report-only L1

## Corpus & Index State Snapshot

| Item | Value |
|------|-------|
| Active corpus markdown (`raw/` skipped) | 462 |
| Formal docs with `title:` | 441 |
| Nav markdown (README/INDEX/CHANGELOG) | 21 |
| Raw assets (`corpus/**/raw/**`) | 993 |
| Indexes | default bin 19312222 bytes; offline_run2 bin 22895636 bytes; graph_store.bin 2169 bytes |

## Retrieval Baseline Gate

Status: **PASS** (gate Recall@5 ≥ 0.99)

| Metric | Value |
|--------|-------|
| Recall@5 | 1.000 |
| MRR@5 | 0.989 |
| NDCG@5 | 0.992 |
| n_queries | 105 |
| k | 5 |
| mode | hybrid |
| index | `data/vector_store.bin` |
| flags | `--offline --no-rerank --json` |

## High Priority

- (empty)

## Watch List

- `corpus/engineering/hacker_laws/references/INDEX.md` has no YAML `title:` (nav file; ingest falls back to stem).
- BM25 overflow / dedup follow-up: `tasks/acceptance-followup-bm25-dedup.md`.
- Live Cohere rerank on `queries_hard.jsonl` still needs a CI secret (deferred).

## Recent Noise

- Easy gold Recall@5 = 1.000 is lexical saturation, not semantic quality.
- Missing `--graph-file` is a documented no-op (dual BM25+Vector only).
