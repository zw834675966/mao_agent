# Cross-Artifact Analysis — Cycle 11 Knowledge Graph Candidate Expansion

Date: 2026-09-06. Method: read-only plan review + live rerun (`cargo test`, `cargo fmt`, `cargo clippy`, CLI smoke).
Artifacts: `tasks/plan.md` (PLAN), `tasks/todo.md` (TASKS), `README.md` + `CONTEXT.md` + `docs/superpowers/specs/*` (SPEC surrogate — repo has no top-level SPEC.md), `src/` + `tests/` + `scripts/` + `evals/` (Code).

> Repo has no `SPEC.md` / `TASKS.md` at root. Mapping used: SPEC ≈ README + CONTEXT.md + `evals/retrieval/*` + superpowers specs; PLAN = `tasks/plan.md`; TASKS = `tasks/todo.md`. This naming gap is itself finding C-7.

## 1. Coverage Matrix (Plan Task → Code)

| Requirement (plan.md) | Task(s) | Code evidence | Coverage |
|---|---|---|---|
| petgraph 0.8 + `Entity/Relationship/SourceRef`, no `Community` | Task 1 | `Cargo.toml:23` `petgraph 0.8 serde-1`; `src/graph/model.rs:9-50`; `src/graph/mod.rs:1-9` no Community; `src/lib.rs:23-26` | ✅ Full |
| `GraphStore` DiGraph, Jieba seed, 1–2 hop, JSON load, bincode atomic save/load, skip unknown edge | Task 2 | `src/graph/store.rs:28-195` (`from_document` skips unknown endpoints `47-52`, `find_seed_entities 114-120`, `expand 123-195` capped `min(2)`); `cut_search` reuse; `tests/graph_store_test.rs` 6 tests | ✅ Full |
| `HybridSearchResult.graph_paths` serde default + `union_graph_bonus` (cap 8, tail 2, rrf 0.0, no `fuse` change) | Task 3 | `src/index/hybrid.rs:16-18,52-108` fuse signature unchanged `(vec,bm25,top_k)`; `src/graph/expand.rs:34-37,54-128`; `tests/graph_expand_test.rs` 4 tests (empty/overlap/zero-rrf/tail) | ✅ Full |
| stdlib-only `--mock` extractor + golden JSON with 主要矛盾↔阿姆达尔 `aligned_with`, titles not hashes | Task 4 | `scripts/build_knowledge_graph.py:1-100` stdlib only; `py ... --mock` → 6 entities / 5 rels (verified 2026-09-06); `evals/graph/golden_graph.json:106-120` aligned_with edge; no requirements.txt | ✅ Full |
| `ingest-graph` + `--graph-file` on search/ask/serve, missing=no-op, no `--mode graph` | Task 5 | `src/cli/mod.rs:133-135,202-204,225-227,307-317,324-325`; `src/main.rs:383-422` `try_load_graph` missing→None + warn-only; `search --help` mode still hybrid/vector/bm25; smoke: missing file no-error verified | ✅ Full |
| `DialecticalAgent::with_graph` + `AppState::with_graph`, hybrid HTTP expands, `mode=graph` rejected, verifier ignores triples, ≤16 triples | Task 6 | `src/agent/engine.rs:98-101,103-142` (`graph_triples` `.take(16)`), `expand_fused 170`, prompt `210`, verify-only-chunks `219`; `src/server/state.rs:33,112-119` (`new/with_ops` untouched, `graph: None`); `src/server/handlers/search.rs:70-74,144-169`; `src/server/handlers/ask.rs:81,169` | ✅ Full |
| Docs (AGENTS/CONTEXT) + one gold eval query (jsonl row and/or GRAPH.md titles, no chunk hashes, no “zero C/C++”) | Task 7 | `AGENTS.md:40,46,56` expander documented; `CONTEXT.md:48-64` sketch includes `graph_store.bin`; `evals/retrieval/GRAPH.md:1-13` titles table; no zero-C++ claim | ⚠ Partial — see C-1, C-2 |
| Checkpoints: fmt/clippy/test + smoke with/without graph | CP1-3 | `cargo fmt --check` PASS; `cargo clippy --no-default-features --all-targets -- -D warnings` PASS (0.84s); `cargo test --no-default-features` **134 passed / 0 failed** (80 lib + 54 integration); smoke with-graph on sample index does NOT yield Amdahl chunk (see C-1) | ⚠ Partial |

No orphaned tasks. No plan component without code. `OUT` list honored: no `Community`, no `fuse_tri_stream`/`graph_weight`, no `--mode graph`, no `UnGraph`, no pip/LightRAG, no chunk_id hashes in JSON (grep verified).

## 2. Output-Quality Deep Check (live rerun 2026-09-06)

| Function | Command | Result |
|---|---|---|
| `stats --offline` | `cargo run --no-default-features -- stats --offline` | ✅ 59 vectors / 15 docs from `data/vector_store.bin` (sample scale, not 668-doc full corpus) |
| `search hybrid` | `search --offline --no-rerank --top-k 3 "主要矛盾"` | ✅ Top1《矛盾论》RRF 0.01639 (vec 0.516 + BM25 9.73), ~2.1s, 59 vec no HNSW |
| `search vector / bm25` | `--mode vector`, `--mode bm25 "主要矛盾"` | ✅ both return Mao hits; vector/bm25 correctly ignore graph |
| `search` missing graph | `--graph-file data/missing_graph.bin` | ✅ no error, dual-only (by design) |
| `search` with graph, cross-domain | `--graph-file data/graph_store.bin "主要矛盾与阿姆达尔定律"` | ⚠ runs, but **BM25 N/A + no Amdahl chunk** — two stacked causes: (a) Tantivy QueryParser default AND → long-sentence zero-recall (P2, `src/index/fulltext.rs:211-218`); (b) default 59-chunk sample index contains **zero engineering docs**, so `source_refs` → Amdahl title unresolvable. Plan smoke “Mao + Amdahl” only achievable on full 668-doc ingest, not documented |
| `ask --offline` | `ask --offline --no-rerank "什么是主要矛盾？"` | ✅ 4-stage dialectical answer, 11.8ms inference, citation `[真子串核验通过] 100.0%`《矛盾论》, 3 chunks with IDs |
| `eval-retrieval` | `eval-retrieval --offline --no-rerank --k 5` | ✅ 105 queries: Recall@5 1.000 / MRR 0.984 / NDCG 0.988. Matches `BASELINE.md` but signal is **saturated/weak**: auto gold is lexically aligned; hard set (`queries_hard.jsonl` 26) is the real bar — do not cite 1.000 as semantic quality |
| `ingest-graph` | `--input evals/graph/golden_graph.json --output data/graph_store.bin` | ✅ 6 entities / 5 edges; artifact was missing before this analysis, now regenerated (gitignored, must rebuild per machine) |
| `mock` extractor | `py scripts/build_knowledge_graph.py --mock` | ✅ exit 0, schema-valid, contains 主要矛盾 + 阿姆达尔定律 + aligned_with |
| `serve` | live probe 2026-09-06 on default index (port 3210): `/live` 200, `/health` 200 (59/15/512), `/metrics` 200, `mode=graph` → 400, hybrid annotated by graph | ✅ GO — former unrun gate closed; Nit: seed-annotated dual hits serialize `graph_paths: []` instead of absent (cosmetic) |
| `init-samples` / `ingest` / `stats` / `eval --force-brute` | help + code inspection | ✅ all 8 subcommands present (`src/cli/mod.rs:319-344`), `--force-brute` correctly eval-only |

Citation quality: adversarial suite expects 100% reject on synonym/reorder/fabricated/cross-doc/noise — suite green. Verifier grounds only against chunks, never triples (`engine.rs:219`), so ≤16-triple prompt injection cannot hallucinate citations by construction.

## 3. Consistency Issues

| ID | Issue | Severity | Evidence | Resolution |
|---|---|---|---|---|
| C-1 | Task 7 smoke “hybrid … returns Mao + Amdahl” **cannot pass on default sample index** (59 chunks, 0 engineering docs). Plan does not scope the smoke to full corpus | High | `stats` 59/15; `corpus/hacker_laws/laws_amdahls_law.md` exists in 668-doc tree but not in sample index; live with-graph query returned 0 Amdahl | Amend `tasks/plan.md` Checkpoint 3 + `GRAPH.md`: smoke requires full ingest (`ingest --offline --corpus-dir corpus` → 5000+ vectors) OR document that sample-index smoke only asserts “Mao hit + no error, bonus possibly empty”. Add one `queries.jsonl` cross-domain row whose expected titles (not hashes) are checked only when full index present |
| C-2 | `evals/retrieval/queries.jsonl` (105 lines) has **zero cross-domain graph rows** (`amdahl/阿姆达尔/graph` grep empty). Task 7 allowed “jsonl and/or GRAPH.md”, so literal PASS, but coverage intent (regression for expander) is unmet | Medium | grep empty; `GRAPH.md` titles table exists as the “or” branch | Add 1 row `{"query":"主要矛盾与阿姆达尔定律", ...}` with title-based expectation documented in GRAPH.md, or mark file as intentionally dual-only and keep graph regression in `graph_expand_test.rs` + `GRAPH.md`. Either way, close the ambiguity in `tasks/todo.md` Task 7 |
| C-3 | `AGENTS.md:61` says “Full suite **126 tests**”, header `:9` says **134**. `README.md:38` says **123**. Truth is **134** (80 lib + 54 integration, verified) | Medium | live `cargo test` sum = 134; `docs/运行分析报告-2026-09-06.md` already flagged 126→130 drift, now 134 | Fix two stale numbers (1-line edits, applied in §5) |
| C-4 | BM25 long-query AND zero-recall still in code (`QueryParser` default). Hybrid degrades to vector-only on colloquial sentences; `BASELINE.md` BM25 0.781 (quoted gold) overstates CLI long-sentence behavior | Medium | `src/index/fulltext.rs:211-218`; live `主要矛盾与阿姆达尔定律` BM25 N/A | Not Cycle 11 scope (dual behavior frozen by design). File as P2 follow-up: consider `Occur::Should` / minimum-should-match or Jieba-tokenized Should-clauses for user queries; keep eval gold path unchanged. Do NOT change `fuse()` weights as side effect |
| C-5 | 2026-09-06 report P0 “`--offline` dim polluted to 768 by Gemini config” is **already fixed** in tree (`src/main.rs:190-204` offline ignores provider config dim; live run shows `Deterministic 512-dim`). Report is stale, code is correct | Low (docs) | live log `Using offline Deterministic Embedder (512-dim)`; `resolve_embed_dimension_with_provider` offline branch | Mark report superseded for P0; no code change. Keep `--embed-dim 512` explicit only when opening legacy `offline_run2` 512 snapshots vs default 512 — actually consistent now |
| C-6 | `data/graph_store.bin` missing at analysis start (graph silently dual-only). No startup warning visible in CLI output; operator cannot tell expander is off | Low | `Test-Path data/graph_store.bin` False → True after regen; `try_load_graph` only `tracing::debug` | Add one `eprintln`/info line when hybrid runs without graph file (“graph file missing; dual-only”), or document in runbook. No behavior change |
| C-7 | Command vocabulary mismatch: user asks for SPEC.md/PLAN.md/TASKS.md, repo uses `tasks/plan.md` + `tasks/todo.md` + README/CONTEXT as spec | Low | glob: no top-level SPEC/PLAN/TASKS | Document mapping in this file (done above) or add symlinks/aliases. No new spec system needed |

## 4. Dependency & Testability Check

- Order Tasks 1→2→3→4/5→6→7 matches code layers (types → store → fuse-union → extractor/CLI → agent/server → docs). Task 4 correctly parallelizable after Task 1 — plan states this.
- Every task has acceptance criteria + verification commands; checkpoints CP1-3 map to `cargo test … graph/hybrid`, `cargo clippy`, `cargo fmt --check` — all runnable, all green this round.
- Testability per task: Task 1 contract test, Task 2 round-trip + seed + DiGraph assert, Task 3 four union cases (incl. `r= min(2,bonus,top_k)`), Task 4 mock-command + `from_json` load, Task 5 help + missing-file no-op + mode-ignore, Task 6 `AppState::new` compat + `mode=graph` reject + ≤16 triples, Task 7 file-exists checks. Each is falsifiable.
- HNSW (≥5000 vectors) irrelevant on sample index; `--force-brute` correctly eval-only (`src/cli/mod.rs:287-289`, absent from SearchArgs). Full-corpus recall comparison still unrun on this machine — do not claim ANN parity.

## 5. Fixes Applied / Recommended

Applied (trivial doc drift, no behavior change):
- [x] `AGENTS.md:61` 126 → 134 (header already 134; verified 80+54).
- [x] `README.md` 123 → 134 (same suite).
- [x] Regenerated gitignored `data/graph_store.bin` from golden JSON (6/5) so local hybrid smoke actually exercises expander path.

Recommended (need human/operator decision, NOT auto-applied):
- [ ] C-1: scope Task 7 smoke to full corpus OR relax assertion for sample index. Propose new Task 8 (P2): “full-corpus graph smoke + doc”.
- [ ] C-2: add 1 cross-domain `queries.jsonl` row (title-based) or explicitly keep graph regression in unit tests + GRAPH.md.
- [ ] C-4: BM25 Should-clause follow-up (separate cycle; frozen `fuse()` weights must not shift).
- [ ] C-6: user-visible “graph missing → dual-only” notice + runbook line (`docs/ops/runbook.md`).
- [ ] Re-probe `serve` `/live` + `/health` + `/api/v1/search mode=graph → 400` against current default index before release note.

## 6. Verdict

- **Implementability: GO.** All 8 CLI subcommands exist, parse, and run offline; hybrid/vector/bm25 + missing-graph no-op + offline ask + eval + ingest-graph all demonstrated; `serve` live probe now green (`/live`, `/health`, `/metrics` 200, `mode=graph` → 400, graph annotation active over HTTP). No feature is stubbed or uncallable.
- **Plan health: CONDITIONAL GO.** Tasks 1–6 fully implemented and tested (134/134 green, fmt/clippy clean). Task 7 is the only partial: docs exist but eval-row + smoke-scope ambiguity plus two stale test-count numbers (now fixed). Close C-1/C-2 wording and this cycle is shippable.
- **Output quality: GO with caveats.** Retrieval correctness (Top1《矛盾论》, 100% true-substring citations, saturated 105-query eval) is real but measured on a 59-chunk sample; cross-domain graph value (Mao↔Amdahl) is proven at unit level (`graph_expand_test`) and resolver level, NOT yet at end-to-end sample-index level by construction. Do not present sample-index numbers as full-corpus quality. Full-corpus (668 docs / 5000+ vectors / HNSW active) + `serve` live probe remain the two unrun gates.
