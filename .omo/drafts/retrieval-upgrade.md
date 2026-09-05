# Draft: 检索栈对齐升级 (retrieval-upgrade)

- **slug:** `retrieval-upgrade`
- **intent:** CLEAR (outcome 明确: 拉齐到 2025-2026 主流 RAG 检索标准)
- **status:** approved
- **created:** 2026-09-03
- **mode_at_create:** plan (read-only) → build (2026-09-03 switch)
- **skill:** ulw-plan (Prometheus)

## Decisions Ledger

| Fork | Options | Decision | When |
|------|---------|----------|------|
| 范围 | P0仅rerank / P0+P1 / P0-P2全量 | **P0-P2全量分批** | 2026-09-03 Q&A |
| Rerank后端 | 双后端回退链 / 仅Cohere / 仅本地ONNX | **仅 Cohere API** | 2026-09-03 Q&A |
| 顺序 | P1评测先行+P2 HNSW / HNSW直接 | **评测先行** | 2026-09-03 Q&A |
| 评测集 | 自动生成 / 手工 / 混合 | **语料自动生成 ~100条** | 2026-09-03 Q&A |

Defaults adopted (non-owner):
- COHERE_RERANK_URL=https://api.cohere.com/v2/rerank, model=rerank-v3.5
- doc输入用 raw_text, 阈值 5000/M16/ef200/100, hnswlib-rs 0.10, rebuild-on-load
- 无key/失败时 warn+跳过 (同 main.rs:356 BM25降级)

## Approach

三批串行: P0 Cohere rerank (trait+cohere client+CLI+Agent 接线) → P1 eval harness (metrics+queries.jsonl+eval-retrieval CLI) → P2 HNSW (hnswlib-rs+阈值切流+回退+回归门禁)。每批独立 commit，通过 fmt/clippy -D warnings/test --no-default-features。

## Approval Gate

- **exploration:** 2026-09-03 完成 — 读 Cargo.toml/hybrid.rs/main.rs/cli/mod.rs/agent/engine.rs/vector/store.rs/index.rs + tavily 外部基准 (hybrid+rerank +17% Recall, RRF k=60, hnswlib-rs 0.10, fastembed TextRerank)
- **questions asked:** 4 forks (见上), 用户已答
- **status:** awaiting-approval → **approved** (2026-09-03 "落盘" 指令)
- **pending action:** write `.omo/plans/retrieval-upgrade.md` — **done 2026-09-03 20:09**, build mode 已解锁

## Research Ledger

- External: Cohere Rerank v2/rerank + rerank-3.5 (tavily 6 results, docs.cohere.com), hnswlib-rs 0.10 Jan 2026 bincode save/load, fastembed 5.17 TextRerank (tavily), hybrid+RRF 2025-2026 benchmarks
- Internal: VectorIndex 线性扫描 dot_product (autovectorized, 无SIMD), SnapshotIdentity + MAGIC MAOVS01, key链 resolve_embed_api_key/resolve_chat_api_key/config_cohere_api_key, tantivy 0.22 + jieba search mode

## Risk Log

- scaffold-plan.mjs 在本机缺失 (ls plugins 无 mjs) → 手工建 .omo/plans + .omo/drafts 按 spec 补偿
- fastembed 4.4 TextRerank 版本未显式验证 (仅 5.17 docs) — 本计划不依赖本地 rerank，无影响
- 评测集生成无 LLM，需保证问题→chunk 映射可追溯 (抽检 20 条)

## Next Steps for Worker

按 `.omo/plans/retrieval-upgrade.md` Todos 顺序执行 Batch1→2→3，每项单独 commit，Batch 间跑 CI 三门 + eval 基线对比。
