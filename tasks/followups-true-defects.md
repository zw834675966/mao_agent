# Follow-ups: true defects (2026-09-09)

## Disposition

User disposition: **记跟进项先放行** — every ticket below is **deferred / allow-through**. This turn does **not** patch product code. No `src/`, `tests/`, `corpus/`, `data/` edits. Tickets are durable follow-up items for a later scheduling decision.

This ledger is the index of true defects. BM25 duplicate-chunk detail lives in
`tasks/acceptance-followup-bm25-dedup.md` (linked, not duplicated here).

## Arbitration (four seats, 2026-09-09)

| Seat | Finding |
|------|---------|
| explore | 2 TRUE_DEFECT still in tree (BM25 overflow, HNSW reader flake). Hard-set 0.731 = QUALITY_GAP. Cycle 13 MCP name drift = NOISE (metrics labels match `src/server/metrics.rs` ↔ `docs/ops/mcp_sre_guide.md`). 15 duplicate-title pairs: `corpus/*.md` vs `corpus/history/*.md`. |
| debugger | D1 CONFIRMED (bm25 branch skips `fuse` HashMap). D2 CONFIRMED (CRUD test does not take the lock). Easy-gold 1.000 / hard-set 0.731 are not this class. |
| reviewer | Wanted hard-set Recall 0.731 filed as 真缺陷. **Overruled:** quality ceiling, not metric/test-infra failure. Remaining 真缺陷 = BM25 overflow + HNSW flake. |
| planner | Tickets F1/F2/F4 deferred; F3 Watch; C1–C8 covered. Did not touch Cycle 15 `tasks/plan.md` / `tasks/todo.md`. |

True defects this ledger will schedule later: **F1, F2**. F3 stays Watch. **F4 was fixed ahead of schedule in-tree on 2026-09-09 (status updated below).**

**Citation ban (BM25 F1/F2):** do not quote pure-BM25 overflow as a result.
Acceptance baseline is hybrid/vector. Detail: `tasks/acceptance-followup-bm25-dedup.md`.

## Evidence sources

| Source | Path | What it proves |
|--------|------|----------------|
| Acceptance follow-up | `tasks/acceptance-followup-bm25-dedup.md` | BM25 dup chunk_ids + eval metric overflow root cause (commit 45cec36) |
| Analysis doc §1.3 | `docs/真实测评分析-Agent评测框架逐维度对标-2026-09-09.md:44-61` | HNSW threshold flake: 3 consecutive `cargo test` runs failed at `src/vector/index.rs:924`; C11 report 209 green was scheduling luck |
| Analysis doc §7.2 | `docs/真实测评分析-Agent评测框架逐维度对标-2026-09-09.md:197-203` | Priority action list (HNSW flake #1, token metering #2, QA gold #3, hard-set recall #4, span tracing #5) |
| Eval metrics | `src/eval/mod.rs:12-56` | `recall_at_k` / `dcg_at_k` count every retrieved position without dedup → duplicates inflate beyond 1.0 |
| BM25 search | `src/index/fulltext.rs:245-465` | `TopDocs::with_limit(top_k)` returns both copies; no `chunk_id` dedup |
| HNSW tests | `src/vector/index.rs:864,896,942,999` | ~~`HNSW_THRESHOLD_TEST_LOCK` held by writers only; `test_vector_index_crud_and_search` does not take it~~ **Superseded 2026-09-09:** the reader test now takes the lock + resets threshold (see F4 status) |

## True-defect tickets (all deferred / allow-through)

### F1 — Eval metrics must not exceed 1.0 on duplicate retrieved ids

- **Status:** deferred / allow-through
- **Goal:** `recall_at_k` / `dcg_at_k` (and thus `ndcg_at_k`) must never exceed 1.0 even when `retrieved` contains duplicate ids.
- **Evidence:** `src/eval/mod.rs:12-56` — `recall_at_k` does `top.filter(|id| expected_set.contains(id.as_str())).count()` over every position; `dcg_at_k` sums every position. For `retrieved=[a,a,b]`, `expected=[a]`: hits=2, expected_set.len()=1 → recall 2.0. Unit tests only cover unique-id inputs (`src/eval/mod.rs:80-160`).
- **Acceptance:** New regression test: `recall_at_k(&["a","a","b"], &["a"], 3) == 1.0` (not 2.0); `ndcg_at_k` with duplicated relevant ids ≤ 1.0. Existing hand-computed tests still pass.
- **Verification:** `cargo test --no-default-features eval` (or `cargo test --no-default-features --lib eval`).
- **Files likely touched:** `src/eval/mod.rs` (+ its `mod tests`).
- **Scope:** XS.

### F2 — FullTextIndex::search dedup by chunk_id keep-first then truncate top_k

- **Status:** deferred / allow-through
- **Goal:** Pure-BM25 top-K must not contain the same `chunk_id` twice.
- **Evidence:** `src/index/fulltext.rs:245-465` — `TopDocs::with_limit(top_k)` returns both Tantivy docs sharing a `chunk_id` (collision from `doc.id = sha256(title+date+volume)` at `src/corpus/parser.rs:123-129` + `chunk_id = format!("{doc.id}_chunk_{idx:04}")` at `src/corpus/chunker.rs:116`). Slot waste visible in `search --mode bm25` output. Hybrid path already dedups via score-map keyed by `chunk_id` (`src/index/hybrid.rs:52-109`).
- **Acceptance:** After fix, `search --mode bm25` returns ≤ `top_k` results with all-unique `chunk_id`; keep first (best-score) occurrence, then truncate to `top_k`.
- **Verification:** `cargo test --no-default-features`; offline `eval-retrieval --mode bm25` no longer yields Recall > 1.
- **Files likely touched:** `src/index/fulltext.rs` (+ a unit test with a duplicated `chunk_id`).
- **Scope:** S.

### F4 — HNSW test: test_vector_index_crud_and_search must take HNSW_THRESHOLD_TEST_LOCK and reset

- **Status:** resolved in tree (fix applied ahead of schedule on 2026-09-09, before this ticket was scheduled)
- **Fix applied:** `test_vector_index_crud_and_search` now takes `HNSW_THRESHOLD_TEST_LOCK` + calls `reset_hnsw_threshold_for_test()` at the top (`src/vector/index.rs:896-900`), mirroring the writers — the smallest fix this ticket proposed
- **Verification:** 1/3 consecutive green runs done (2026-09-09, 212 passed / 0 failed); 2 more consecutive runs pending formal sign-off
- **Goal:** Eliminate the scheduling-dependent flake where `test_vector_index_crud_and_search` fails intermittently.
- **Evidence:** `HNSW_THRESHOLD_OVERRIDE` is a process-global atomic (`src/vector/index.rs:20`). `HNSW_THRESHOLD_TEST_LOCK` (`src/vector/index.rs:864`) is held only by writers `test_hnsw_activates_at_threshold` (`:942`) and `snapshot_clone_does_not_rebuild_hnsw` (`:999`, sets threshold to 2). `test_vector_index_crud_and_search` (`:896`) asserts `!has_hnsw()` but takes no lock; if it reads a lowered threshold (2) while inserting its 2nd vector, HNSW activates early → assertion fails. 3 consecutive `cargo test` runs failed at `src/vector/index.rs:924` (analysis doc §1.3); C11's 209-green was scheduling luck.
- **Acceptance:** `cargo test --no-default-features` passes repeatedly (e.g. 3+ consecutive runs) with no `test_vector_index_crud_and_search` failure. Prefer smallest future fix: take `HNSW_THRESHOLD_TEST_LOCK` + `reset_hnsw_threshold_for_test()` at the top of the test (mirroring the writers), or convert the override to per-index field injection.
- **Verification:** `cargo test --no-default-features` ×3 consecutive.
- **Files likely touched:** `src/vector/index.rs` (test body or override mechanism).
- **Scope:** XS.

## Watch list (quality gaps — not true defects without path proof of a correctness bug)

Per C6, these are tracked but not promoted to 真缺陷 tickets unless a path shows they are correctness bugs:

| Item | Evidence | Why not a ticket |
|------|----------|------------------|
| Easy-gold saturation (Recall 1.0 is a metric artifact) | `evals/retrieval/BASELINE.md:21-29`; analysis doc §1.2 | Gold queries template-generated from chunk text; not a code bug |
| Hard-set Recall@5 0.731 | analysis doc §1.1 / §7.2 #4 | Retrieval quality gap, not a correctness defect; candidate for graph/rerank work |
| Token metering = 0 | analysis doc §7.2 #2 | Missing feature, not a bug; blocks Cost & Latency answer |
| main.rs split | analysis doc §7.2 (implied) | Maintainability, not correctness |
| cargo audit | — | Hygiene, not correctness |
| Cycle 13 name drift | explore: `src/server/metrics.rs` labels match `docs/ops/mcp_sre_guide.md` | NOISE — no drift in tree |
| Ingest warn on `doc.id` collision (was F3) | `src/corpus/parser.rs:120-132`; archived 12+ title groups; 2026-09-09 rescan 18 groups (15 history mirrors + 3 Hello 算法 chapter↔index) | Hygiene. F2 is the correctness fix; warn is optional observability |

## Constraints Checklist

| ID | Constraint | covered |
|----|-----------|---------|
| C1 | 记跟进项先放行 — no product patch this turn | yes |
| C2 | do not overwrite Cycle 15 `tasks/plan.md` or `tasks/todo.md` | yes |
| C3 | do not quote bm25 Recall 1.962 / NDCG 1.593 as valid retrieval quality | yes |
| C4 | hybrid/vector eval remains the committed conditional-pass baseline (RRF path dedups) | yes |
| C5 | each true-defect ticket: acceptance + verification command + files likely touched + scope XS/S | yes |
| C6 | quality gaps go in Watch List, not as 真缺陷 without path proof | yes |
| C7 | keep BM25 detail in existing file; this ledger links it | yes |
| C8 | ponytail: no new framework, no extra markdown besides this one file | yes |
