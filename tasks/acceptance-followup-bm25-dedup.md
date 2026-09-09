# Acceptance Follow-up: BM25 duplicate chunk_ids + eval metric overflow (2026-09-09)

Source:竣工验收复跑 (commit 45cec36). Disposition per user: 记跟进项先放行 (deferred, conditional pass).

## Citation ban (SSOT)

- Do **not** quote pure-BM25 overflow numbers as retrieval quality.
- Acceptance baseline is **hybrid / vector only** (RRF path dedups).
- Forensic `--json` shape `[X,X,Y,Y,Z]` is evidence of the bug, not a score.

## Symptom

Pure BM25 top-5 can return `[X,X,Y,Y,Z]`. That makes Recall/NDCG **> 1**, which
is impossible for a correct metric. `--json` rows and the archived pwsh
duplicate-title scan are the evidence; they are not results.

Conditional-pass numbers (valid): hybrid and vector on the same run. The
full-corpus index at that run was 442 docs / 4811 chunks / 512-dim.

## Root cause (evidence-backed)

Chain (still in tree):

1. Duplicate titles in `corpus/` — archived pwsh scan **12+** groups; 2026-09-09
   rescan of `corpus/**/*.md` excluding `raw/`: **18** title groups /
   36 files (15 `corpus/*.md` ↔ `corpus/history/*.md` mirrors with identical
   title/date/volume, plus 3 Hello 算法 chapter↔index pairs).
2. `src/corpus/parser.rs:123-129` `generate_doc_id` = sha256(title + `\0` + date
   + `\0` + volume) → same frontmatter → same `doc.id`.
3. `src/corpus/chunker.rs:116` `chunk_id = format!("{doc.id}_chunk_{idx:04}")`
   collides; Tantivy stores two docs with the same `chunk_id`.
4. `src/index/fulltext.rs` `search` (`TopDocs::with_limit(top_k)` at :362-365)
   has **no** `chunk_id` dedup → pure BM25 top-K can be `[X,X,Y,Y,Z]`.
5. `src/eval/mod.rs:12-56` `recall_at_k` / `dcg_at_k` count every position
   without dedup → Recall/NDCG > 1. Unit tests only cover unique ids.

Hybrid is **not** affected: `src/index/hybrid.rs:52-109` `fuse` keys the score
map by `chunk_id`, so copies collapse. Quote hybrid/vector, not pure BM25.

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

None blocking. Conditional pass = hybrid/vector (RRF path dedups). Pure-BM25
eval on this corpus is invalid until F1+F2. **Citation ban:** do not quote
pure-BM25 overflow as a result.

See also: `tasks/followups-true-defects.md` (F1 eval clamp, F2 BM25 search
dedup; HNSW flake is F4, not this ticket).
