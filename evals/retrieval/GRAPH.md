# Cycle 11 graph expander — expected titles

Do **not** pin ingest-hash `chunk_id`s here; they change on re-chunk. Join is `source_refs.doc_title`.

| Query | Graph should surface (doc titles) |
|---|---|
| 主要矛盾 | 矛盾论 (seed) |
| 主要矛盾 | 阿姆达尔定律 (Amdahl's Law) via `aligned_with` hop-1 |
| 主要矛盾与阿姆达尔定律 | both titles above in hybrid `--no-rerank` when `graph_store.bin` is loaded |

Fixture: `evals/graph/golden_graph.json`. Compile: `cargo run --no-default-features -- ingest-graph --input evals/graph/golden_graph.json --output data/graph_store.bin`.

Missing `--graph-file` / missing file: dual BM25+Vector only, no error (CLI prints a one-line notice via `try_load_graph`).

## Scope note (Cycle 12)

- Default sample index (`data/vector_store.bin`, 59 chunks / 15 docs) contains **zero** engineering docs, so the Amdahl rows above only resolve on a **full** ingest (`corpus/`, 500+ docs, HNSW active). Sample-index smoke asserts: Mao hit + no error (graph bonus possibly empty).
- `queries.jsonl` intentionally has **no** cross-domain row: it pins ingest-hash `chunk_id`s, which are scope-specific and would score 0 on the other index scope. Graph regression lives in `tests/graph_expand_test.rs` + `tests/graph_store_test.rs` and the title table above.
