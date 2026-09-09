# Acceptance Follow-up: BM25 duplicate chunk_ids + eval metric overflow (2026-09-09)

Source:竣工验收复跑 (commit 45cec36). Disposition per user: 记跟进项先放行 (deferred, conditional pass).

## Symptom

Full-corpus offline index (442 docs / 4811 chunks / 512-dim) + `eval-retrieval --mode bm25`:

| Mode   | Recall@5 | MRR@5 | NDCG@5 |
|--------|----------|-------|--------|
| hybrid | 1.000    | 0.989 | 0.992  |
| vector | 1.000    | 0.964 | 0.973  |
| bm25   | **1.962**| 0.975 | **1.593** |

Recall > 1 and NDCG > 1 are impossible for correct metrics. `--json` rows show
`retrieved_chunk_ids` like `[X, X, Y, Y, Z]` with `recall_at_k: 2.0` for a
single-element expected set.

## Root cause (evidence-backed)

1. `corpus/` contains 12+ duplicate-title groups (pwsh scan: identical `title:`
   frontmatter lines in 2 files each). `MarkdownParser` derives
   `doc.id = sha256(title + \0 + date + \0 + volume)` (`src/corpus/parser.rs:123-129`),
   so same title/date/volume in two files yields the same `doc.id`, and
   `chunk_id = format!("{doc.id}_chunk_{idx:04}")` (`src/corpus/chunker.rs:116`)
   collides. Tantivy then holds two documents with the same `chunk_id`.
2. `FullTextIndex::search` (`src/index/fulltext.rs:245-465`) has no
   `chunk_id` dedup: `TopDocs::with_limit(top_k)` returns both copies, so pure
   BM25 top-K can contain the same chunk twice (slot waste, user-visible in
   `search --mode bm25` output).
3. `HybridSearchService` is unaffected: `HybridSearchCoordinator::fuse`
   (`src/index/hybrid.rs:52-109`) keys the score map by `chunk_id`, so dupes
   collapse there (hybrid numbers above are valid).
4. `eval::recall_at_k` / `dcg_at_k` (`src/eval/mod.rs:12-56`) count every
   retrieved position without dedup, so duplicated hits inflate Recall/NDCG
   beyond 1.0. Unit tests only cover unique-id inputs.

## Proposed minimal fix (not applied; awaits scheduling)

1. `src/index/fulltext.rs::search`: dedup by `chunk_id`, keep first (best-score)
   occurrence, then truncate to `top_k`.
2. `src/eval/mod.rs`: dedup `retrieved` preserving order (or clamp outputs to
   `[0,1]`); add regression test with duplicated retrieved ids
   (e.g. `[a,a,b]` vs expected `[a]` must give recall 1.0, not 2.0).
3. Optional corpus hygiene: warn on `doc.id` collision at ingest time
   (`CorpusScanner::load_documents_from_dir` or `handle_ingest`).
4. Re-run: `cargo fmt --check`, `cargo clippy --no-default-features
   --all-targets -- -D warnings`, `cargo test --no-default-features`,
   offline `eval-retrieval` (hybrid/vector/bm25), refresh
   `evals/retrieval/BASELINE.md` (currently stale: claims 59 chunks / 15 docs
   from Cycle 2; full corpus is now 442 docs / 4811 chunks).

## Acceptance impact

None blocking: committed baseline for conditional pass is hybrid/vector
(RRF path dedups). Pure-BM25 eval numbers from this corpus are invalid until
fixed; do not quote `bm25 Recall 1.962` as a result.
