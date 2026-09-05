# Plan: 检索栈对齐升级 (Rerank → Eval → HNSW)

## TL;DR (For humans)

P0: Cohere Rerank 精排 — `src/rerank/cohere.rs` 直连 `https://api.cohere.com/v2/rerank` (rerank-v3.5)，`fuse(top_k*2) → rerank → top_k`，无 key 降级跳过。
P1: 检索评测 harness — `evals/retrieval/queries.jsonl` (~100 条自动生成) + `src/eval/mod.rs` (Recall/MRR/NDCG@k) + CLI `eval-retrieval`。
P2: HNSW ANN — `hnswlib-rs`，阈值 5000 条切 ANN，load 时重建，过滤不足回退暴力扫描，评测对比 recall 回归 <1%。
范围: P0-P2 全量；后端: 仅 Cohere API；顺序: 评测先行→HNSW；评测集: 语料自动生成。

> Approval: 2026-09-03 已批准 P0-P2/仅Cohere/评测先行/自动生成。Worker 可直接执行，无需再访谈。

---

## Context & Scope

**Goal:** 将本项目数据检索与向量栈拉齐到 2025-2026 主流 RAG 标准。

**Gaps closed:**
1. 线性暴力扫描 → HNSW ANN (规模墙)
2. 无 cross-encoder rerank → 质量墙 (RRF 后最大增益点)
3. 零检索评测 → `top_k*2` / `0.5:0.5` 全靠拍板

**Scope:** P0-P2 全量分三批递进。P3 (bge-m3/增量 upsert) 不在本计划。

**Key files (read-before-touch):**
- `src/index/hybrid.rs:18` `HybridSearchCoordinator` + `src/index/hybrid.rs:24` `k_constant=60.0`
- `src/main.rs:332` `search_hybrid` (fuse call site `main.rs:364`), `src/agent/engine.rs:105` `DialecticalAgent::ask` fuse site `engine.rs:115`
- `src/cli/mod.rs:74/135` `SearchArgs`/`AskArgs`, `src/main.rs:42/48` key 解析链
- `src/vector/store.rs` `VectorStore`, `src/vector/index.rs` `VectorIndex`, `src/vector/persist.rs` snapshot

---

## Decisions (Locked 2026-09-03)

| Fork | Decision | Rationale |
|------|----------|-----------|
| 范围 | P0-P2 全量分批 | 调研结论，一次规划分批执行 |
| Rerank 后端 | 仅 Cohere API | 用户拍板；复用既有 key 链；零本地模型下载 |
| 顺序 | P1 评测先行，P2 HNSW | 用评测证明 HNSW 无召回回归 |
| 评测集 | 语料自动生成 ~100 条 | 用户拍板；静态 `queries.jsonl` 入库可复现 |

**Defaults adopted (non-owner, worker follows):**
- Rerank endpoint 独立常量 `COHERE_RERANK_URL = "https://api.cohere.com/v2/rerank"` (兼容层 `api.cohere.ai/compatibility/v1` 不含 rerank)
- Model 默认 `rerank-v3.5`，无覆盖时走 `COHERE_RERANK_MODEL` env / `config.toml [cohere].rerank_model`
- Rerank doc 输入用 `chunk.raw_text` (不含面包屑头，避免 header 污染相关性打分)
- CLI 默认: 有 key 自动 rerank，无 key/失败时 warn+跳过 (同 `main.rs:356` BM25 失败降级模式)
- HNSW 库选 `hnswlib-rs` (2026-01 活跃、自带 bincode save/load、纯 Rust 无 C++ 依赖)，阈值 `HNSW_THRESHOLD=5000`、M=16、efConstruction=200、efSearch=100，快照格式不变 (rebuild-on-load)

---

## Todos

> 每个 todo 含 References / Acceptance / QA / Commit。Worker 按序执行，每项独立可验收。

### Batch 1 — P0 Cohere Rerank 精排 (highest ROI)

#### TODO-01: 新增 `src/rerank/` 模块与 `Reranker` trait

- **References:** `src/vector/embedder/mod.rs` (trait 模式参考), `src/index/hybrid.rs:6` `HybridSearchResult`, `src/error/mod.rs` `VectorError`
- **Actions:**
  - 创建 `src/rerank/mod.rs`：定义 `#[async_trait] pub trait Reranker: Send+Sync { fn model_name(&self)->&str; async fn rerank(&self, query:&str, candidates:&[HybridSearchResult], top_k:usize)->Result<Vec<HybridSearchResult>>; }`
  - 创建 `src/rerank/cohere.rs` 占位 (TODO-02 实现)
  - `src/lib.rs` 暴露 `pub mod rerank;` + re-export `Reranker`
  - `src/error/mod.rs` 新增变体 `RerankError(String)` (或复用 `Other`)
  - 常量: `src/vector/embedder/mod.rs` 旁或 `src/rerank/cohere.rs` 内 `pub const COHERE_RERANK_URL: &str = "https://api.cohere.com/v2/rerank";` + `COHERE_RERANK_MODEL: &str = "rerank-v3.5";`
- **Acceptance:**
  - `cargo check` 通过；`Reranker` trait 可被 `Arc<dyn Reranker>` 持有；无新增 feature gate (Cohere 仅用 reqwest 已有依赖)
- **QA:**
  - `cargo fmt --check` / `cargo clippy --no-default-features --all-targets -- -D warnings` 通过 (不含新增弃用警告)
  - 单测: `src/rerank/mod.rs` mock impl 通过 trait 对象调用 (无网络)
- **Commit:** `feat(rerank): add Reranker trait and module scaffold`

#### TODO-02: 实现 `CohereReranker` (POST /v2/rerank)

- **References:** `src/agent/engine.rs:162` `call_llm_api` (reqwest bearer_auth 模式), `src/vector/embedder/openai.rs` (远程 API 封装参考), `src/main.rs:42` key 解析
- **Actions:**
  - `src/rerank/cohere.rs`:
    ```rust
    pub struct CohereReranker { client: reqwest::Client, api_key: String, model: String, base_url: String }
    impl CohereReranker { pub fn new(api_key:String, model:Option<String>, base_url:Option<String>)->Self }
    // req: {model, query, documents:[String], top_n: usize}
    // resp: {results:[{index:usize, relevance_score:f32}], id?}
    // documents 映射自 candidates.iter().map(|c| c.chunk.raw_text)
    // relevance_score 写入 HybridSearchResult.rerank_score = Some(score)，并按 score 降序重排，截断 top_k
    // HTTP 非 2xx => VectorError::Other("Cohere rerank HTTP {status}: {body}")
    ```
  - 超时 30s (同 embedder)，`tracing::warn` 不 panic
  - 支持 `base_url` 覆盖 (默认 `COHERE_RERANK_URL`，便于测试 mock server)
- **Acceptance:**
  - 给定 3 candidates + mock server 返回 `[1,0,2]` 重排顺序，`rerank()` 输出按 relevance_score 降序且 `rerank_score.is_some()`；top_k 生效
  - 无 key 时不构造 (由工厂返回 None，见 TODO-04)
- **QA:**
  - Happy: `cargo test --no-default-features rerank -- --nocapture` mock 重排单测通过
  - Failure: 4xx/5xx + 超时走 `Err(VectorError)`, 调用方降级路径见 TODO-04
  - `cargo test --no-default-features` 全绿
- **Commit:** `feat(rerank): implement CohereReranker v2/rerank client`

#### TODO-03: 扩展 `HybridSearchResult` 与精排集成点

- **References:** `src/index/hybrid.rs:6` `HybridSearchResult` struct, `src/index/mod.rs` re-exports
- **Actions:**
  - `HybridSearchResult` 新增 `pub rerank_score: Option<f32>` (加 `#[serde(default)]` 以兼容任何历史序列化，虽为瞬态结构但加守卫)
  - `HybridSearchCoordinator::fuse` 保持不变 (RRF 仍为 60.0/0.5:0.5)
  - `src/rerank/mod.rs` 新增辅助 `pub fn rerank_or_fallback(candidates: Vec<HybridSearchResult>, reranker: Option<&dyn Reranker>, query:&str, top_k:usize) -> Vec<HybridSearchResult>` — 有 reranker 则调 `rerank`，失败时 `tracing::warn` 返回原序截断
- **Acceptance:**
  - 既有 `hybrid.rs:129` `test_rrf_fusion` 仍通过；新字段 `None` 时序列化/反序列化 round-trip 通过
- **QA:**
  - 同 TODO-01 gates + `cargo test --no-default-features --lib index::hybrid`
- **Commit:** `feat(rerank): add rerank_score to HybridSearchResult and fallback helper`

#### TODO-04: CLI 与 Agent 接线 (search/ask 双路径)

- **References:** `src/cli/mod.rs:74` `SearchArgs`, `src/cli/mod.rs:135` `AskArgs`, `src/main.rs:284` `get_embedder`, `src/main.rs:42` `resolve_*_api_key`, `src/main.rs:332` `search_hybrid`, `src/agent/engine.rs:30` `DialecticalAgent`
- **Actions:**
  - `src/cli/mod.rs`:
    - `SearchArgs` / `AskArgs` 各加:
      - `#[arg(long, default_value_t = true)] pub rerank: bool` (或 `--rerank/--no-rerank` via `ArgAction::Set`)
      - `#[arg(long, env="COHERE_RERANK_MODEL")] pub rerank_model: Option<String>`
      - 实际形态用 clap 4 `#[arg(long)] pub no_rerank: bool` + 逻辑合并；保证 `--no-rerank` 可禁用
  - `src/vector/embedder/mod.rs` 或 `src/config/mod.rs` 加 `pub const COHERE_RERANK_MODEL` (同 COHERE_EMBED_MODEL 模式)
  - `src/main.rs`:
    - `fn resolve_rerank_api_key(cli_key: Option<String>) -> Option<String>` (复用 `config_cohere_api_key()` + `COHERE_API_KEY` env，同 `resolve_chat_api_key` 逻辑，offline=false 语义)
    - `fn make_reranker(args: &SearchArgs|&AskArgs) -> Option<Arc<dyn Reranker>>` — 仅当 `!no_rerank && resolve.is_some()` 时 `Some(Arc::new(CohereReranker::new(key, rerank_model, None)))`
    - `search_hybrid` 改为: `let fused = coordinator.fuse(vec_results, bm25_results, top_k*2); let results = rerank_or_fallback(fused, reranker.as_deref(), &args.query, args.top_k).await;` 并在 rerank 前后打印耗时 (`⚡ Rerank 耗时`)
    - `handle_ask` 同步：`DialecticalAgent::new` 新增参数 `reranker: Option<Arc<dyn Reranker>>` (见 TODO-04b)
  - `src/agent/engine.rs`:
    - `DialecticalAgent` 新增字段 `reranker: Option<Arc<dyn Reranker>>`
    - `DialecticalAgent::new` 签名加 `reranker: Option<Arc<dyn Reranker>>` (保持向后：调用方传 `make_reranker`)
    - `ask()` 内 `self.reranker.as_ref()` 分支: 有则 `rerank_or_fallback(fused, ...)`；无则 `fused.into_iter().map(|r| r.chunk).collect()` 原逻辑
    - 保持 `fulltext_index: None` 时仍走 vector-only 原分支 (不 rerank 跨两种路径一致)
- **Acceptance:**
  - `cargo run -- search "持久战" --no-rerank` 走原 RRF 路径；有 key 且无 `--no-rerank` 时日志含 `Rerank`
  - `cargo run -- ask "为什么是持久战"` 同步支持 `--no-rerank` / `--rerank-model rerank-v3.5`
  - 无 key 时不报错，warn 后返回融合结果 (降级)
- **QA:**
  - Happy: `cargo test --no-default-features` 全绿；`cargo run -- search --help` 展示新 flags
  - Failure: 无 key + `rerank=true` → warn+原序；Cohere 5xx → warn+原序；offline 模式隐式 no_rerank
  - Gates: `cargo fmt --check`, `clippy --no-default-features --all-targets -- -D warnings`
- **Commit:** `feat(rerank): wire Cohere rerank into search/ask with --no-rerank fallback`

#### TODO-05: P0 回归与文档

- **References:** `README.md`, `AGENTS.md` Commands, `.github/workflows/ci.yml`
- **Actions:**
  - `README.md` 在 Features/CLI 章节补充 `search`/`ask` 的 `--no-rerank` / `COHERE_RERANK_MODEL` 说明
  - 确认 `cargo test --no-default-features` 无网络依赖 (Cohere 相关测试均为 mock/构造测试)
- **Acceptance:** README 与实际 CLI `--help` 一致；CI 三门本地通过
- **QA:** `cargo fmt --check && cargo clippy --no-default-features --all-targets -- -D warnings && cargo test --no-default-features`
- **Commit:** `docs: document rerank flags and COHERE_RERANK_MODEL`

---

### Batch 2 — P1 检索评测 harness (评测先行，为 P2 提供回归防线)

#### TODO-06: 指标库 `src/eval/mod.rs` (pure functions)

- **References:** 新模块，无既有依赖；测试参考 `src/index/hybrid.rs:103` tests
- **Actions:**
  - 创建 `src/eval/mod.rs` + `src/lib.rs` 暴露 `pub mod eval;`
  - 函数签名 (deterministic, 无网络):
    - `pub fn recall_at_k(retrieved: &[String], expected: &[String], k:usize)->f32`
    - `pub fn mrr_at_k(retrieved: &[String], expected: &[String], k:usize)->f32`
    - `pub fn dcg_at_k(retrieved: &[String], expected: &HashSet<String>, k:usize)->f32` + `pub fn ndcg_at_k(...)->f32`
    - `expected` 为 gold chunk_id 集合；`retrieved` 为按 rank 排序的 chunk_id 列表
  - 处理 edge: `k==0` / 空 expected => 0.0；`k > len` 截断语义
- **Acceptance:**
  - 手算值单测: `retrieved=[a,b,c], expected={b,c} => recall@2=0.5, recall@3=1.0, mrr@3=0.5, ndcg@3` 与公式一致
  - 零 cargo feature 依赖
- **QA:**
  - `cargo test --no-default-features eval -- --nocapture` 新单测全绿 + 边界用例 (空集/k>len)
  - `cargo clippy --no-default-features --all-targets -- -D warnings` 零警告
- **Commit:** `feat(eval): add recall/mrr/ndcg@k metrics`

#### TODO-07: 评测集 `evals/retrieval/queries.jsonl` (语料自动生成，~100 条)

- **References:** `corpus/*.md` (15 docs sample, `handle_init_samples` 映射), `src/corpus/chunker.rs` chunk 产出, `src/model.rs` `DocumentChunk`
- **Actions:**
  - 创建目录 `evals/retrieval/`
  - 生成脚本/流程 (worker 执行时落地, 可为临时 Python 或直接手写 JSONL):
    - 输入: `corpus/` 全量 chunk (或 `cargo run -- init-samples` 后 `corpus/`); 按 `doc_title` 分桶，每档 5-8 条，覆盖 15 篇 + 4 时期 + 3 类别
    - 每条记录: `{"query":"中文问题","expected_chunk_ids":["<chunk_id>"],"filter":null|{"period":"抗日战争时期"}}` — `expected_chunk_ids` 为 gold，`filter` 少数用例带 period/volume 约束
    - 生成方式: 从 chunk `raw_text` 提取关键句改写为问题 (如 "抗日战争为什么是持久战的三个阶段是什么" ← chunk 含 "战略防御、战略相持、战略反攻")；无需 LLM，确定性模板+人工抽检 20 条
    - 总数 90-110 条，文件 `evals/retrieval/queries.jsonl` (newline JSON, UTF-8)
  - 同目录 `evals/retrieval/README.md` 说明字段与抽检清单 (列出 5 条人工复核过的 query→chunk 映射)
- **Acceptance:**
  - `wc -l evals/retrieval/queries.jsonl` in [90,110]；每行合法 JSON；`expected_chunk_ids` 非空且对应真实 chunk_id 前缀可追溯 (抽检 5 条在 corpus 中定位到原文)
  - 覆盖: `period` 分布含 WarOfResistance/AgrarianRevolutionaryWar/LiberationWar/SocialistConstruction 至少各 5 条
- **QA:**
  - `python3 -c "import json; [json.loads(l) for l in open('evals/retrieval/queries.jsonl')]"` 无异常
  - `cargo test --no-default-features` 不读取该文件 (指标库与评测集解耦)
- **Commit:** `feat(eval): add retrieval queries.jsonl (~100 auto-generated from corpus)`

#### TODO-08: CLI 子命令 `eval-retrieval` + BASELINE

- **References:** `src/cli/mod.rs:173` `Commands`, `src/main.rs:16` dispatch, `src/vector/store.rs:194` `load_from_file`
- **Actions:**
  - `src/cli/mod.rs` 新增:
    ```rust
    #[derive(Args, Debug, Clone)]
    pub struct EvalRetrievalArgs {
        #[arg(long, default_value="evals/retrieval/queries.jsonl")] pub queries_file: PathBuf,
        #[arg(long, default_value_t=5)] pub k: usize,
        #[arg(long, default_value="hybrid")] pub mode: String, // hybrid/vector/bm25
        #[arg(long)] pub no_rerank: bool,
        #[arg(long)] pub force_brute: bool, // 预留供 P2 对比
        #[arg(long)] pub json: bool,
        #[command(flatten)] pub embedder: EmbedderArgs,
        #[arg(short, long, default_value="data/vector_store.bin")] pub index_file: PathBuf,
        #[arg(long, default_value="data/tantivy_index")] pub tantivy_dir: PathBuf,
    }
    // Commands::EvalRetrieval(EvalRetrievalArgs)
    ```
  - `src/main.rs` 新增 `async fn handle_eval_retrieval(args:&EvalRetrievalArgs)`:
    - 读 `queries.jsonl` → 加载 store+tantivy (复用 `load_store_interactive`/`FullTextIndex::new_in_dir`)
    - 对每 query 按 `mode` 调 `store.search` / `ft_index.search` / `search_hybrid` 内联逻辑 (含 rerank 分支，`force_brute` 透传)
    - 调 `eval::{recall_at_k,mrr_at_k,ndcg_at_k}` 聚合均值，打印表格:
      `| metric | @k | value |` + per-query 明细 (json 模式输出 NDJSON)
    - 非 `--json` 时打印: `Recall@k / MRR@k / NDCG@k` 均值 (3  decimal)
  - `src/lib.rs` 保证 `eval` 模块在 `--no-default-features` 下可编译 (pure rust, 无 tantivy/reqwest 依赖)
  - 产出 `evals/retrieval/BASELINE.md` (worker 在真实索引上 `cargo run -- eval-retrieval --mode hybrid --k 5` 跑一次后填写，含 Recall/MRR/NDCG 三值 + `--no-rerank` 对比 + 时间戳/commit)
- **Acceptance:**
  - `cargo run -- eval-retrieval --help` 可见；`cargo run -- eval-retrieval --json --k 5` 在已 `ingest` 的真实索引上输出合法 JSON 且含 `recall_at_k`
  - `--no-rerank` 与默认 rerank 的指标差可量化 (rerank 开启时 NDCG@5 预期提升，至少表格可对比)
  - `BASELINE.md` 含 hybrid/vector/bm25 三行基线 (占位可先填 `--no-default-features` 下 mock 索引的 smoke 值 + 备注 "real-index TBD")
- **QA:**
  - Happy: 在 Deterministic 小索引上 (2 docs) smoke: `eval-retrieval --mode vector --k 2 --json` 返回 1.0 recall (gold 命中)
  - Failure: queries_file 缺失 => `eprintln + exit 1`；index 缺失 => 友好提示 (同 search 路径)
  - Gates: `cargo fmt --check`, `clippy --no-default-features`, `cargo test --no-default-features` 全绿
- **Commit:** `feat(eval): add eval-retrieval CLI and BASELINE`

---

### Batch 3 — P2 HNSW ANN 索引

#### TODO-09: 集成 `hnswlib-rs` 到 `VectorIndex` (阈值切流 + 回退)

- **References:** `src/vector/index.rs` (VectorIndex struct + `search` 线性扫描), `src/vector/persist.rs` snapshot, `Cargo.toml:32` `tantivy`
- **Actions:**
  - `Cargo.toml` 加 `hnswlib-rs = "0.10"` (无 optional, 纯 Rust 轻量)
  - `src/vector/index.rs`:
    - `pub const HNSW_THRESHOLD: usize = 5000;` `const HNSW_M: usize = 16;` `const HNSW_EF_CONSTRUCTION: usize = 200;` `const HNSW_EF_SEARCH: usize = 100;`
    - `VectorIndex` 扩展:
      ```rust
      // 内存态 HNSW，不序列化 (serde skip)，load 时重建
      #[serde(skip)]
      hnsw: Option<hnswlib_rs::Hnsw<u32, f32>>, // 或 HnswIndex 包装
      // 配套: id_to_node: HashMap<String, u32> (node id 映射)
      ```
      实际按 crate API 调整：`hnswlib-rs 0.10` 暴露 `Hnsw::new(M, ef_construction)` + `insert(node_id, vector)` + `search(vector, k, ef)`；若 API 为 `Hnswlib` 包装则对应适配 (worker 以 `cargo docs --open hnswlib-rs` 为准)
    - `insert_batch` 内: 入库后若 `entries.len() >= HNSW_THRESHOLD` 且 `hnsw.is_none()` => 全量建图；否则增量 `insert` 新节点；`len < THRESHOLD` 时保持 `hnsw=None` (暴力路径)
    - `search(&self, query:&[f32], top_k, filter)` 分流:
      - `hnsw.is_none()` => 原暴力 `dot_product` 扫描 (保留)
      - `hnsw.is_some()` => `ef = max(top_k*10, 100)` 调 `hnsw.search(query, ef)` 取候选 node_ids → 映射回 `VectorEntry` → 过滤 `filter` (post-filter)
      - 过滤后不足 `top_k` => 回退暴力扫描补齐 (保证召回正确性，语料中等规模可接受)
    - 归一化保持: 入库已 `normalize_in_place`，query 向量在 `store.search` 前已 embed，HNSW 距离用 L2 或 cosine (与暴力点积等价；选 L2 需保证归一化后等价)
  - `persist.rs` 保持不变：`hnsw` skip 序列化，`load_snapshot` 后若 `index.len() >= THRESHOLD` 则 `rebuild_hnsw()` (遍历 entries 重建图，耗时秒级可接受)
  - `compute_stats` / `clear` 同步处理 `hnsw` (clear 时 `None`)
- **Acceptance:**
  - `<5000` 条时 `hnsw.is_none()` 且 `search` 走暴力 (行为与原版一致)
  - `>=5000` 条时 `hnsw.is_some()`，`search` 走 ANN；大 k 过滤后不足时回退暴力且结果数 = `min(top_k, filtered_total)`
  - `save_to_file` → `load_from_file` 后 HNSW 自动重建，`search` 结果一致
- **QA:**
  - Happy: 合成测试 `HNSW_THRESHOLD` 附近：`4999` vs `5000`/`5001` 插入各 10 条随机 64d，search 结果集相等 (tolerance 1 位差异内，因 ANN 近似) — 实测放宽为 `recall@5 >= 0.95` 对比暴力 (见 TODO-10)
  - Failure: filter 过滤掉大部分候选 (如 period 不存在) => 回退路径返回正确空集/补齐集
  - Gates: `cargo fmt --check`, `clippy --no-default-features --all-targets -- -D warnings`, `cargo test --no-default-features` 全绿
- **Commit:** `feat(index): integrate hnswlib-rs ANN with threshold and brute-force fallback`

#### TODO-10: HNSW 回归验证与基准对比

- **References:** `evals/retrieval/queries.jsonl`, `src/eval/mod.rs`, `src/cli/mod.rs` `force_brute` flag
- **Actions:**
  - `src/cli/mod.rs` `EvalRetrievalArgs` 的 `force_brute: bool` 接线到 `VectorIndex`：当 true 时 `search` 强制走暴力分支 (新增 `VectorIndex::search_brute` 或 `search_with_force` 参数)
  - 在真实索引上 (或合成 6000 条 Deterministic 索引) 运行:
    ```
    cargo run -- eval-retrieval --mode vector --k 5 --json > /tmp/brute.json
    cargo run -- eval-retrieval --mode vector --k 5 --force-brute --json > /tmp/ann.json # 实际为 ANN vs brute 对比，flag 语义注意
    ```
    比较两者 `recall@5` / `ndcg@5` 差值
  - 更新 `evals/retrieval/BASELINE.md` 追加一行 `ANN (hnswlib-rs, threshold 5000, ef=100)` 指标 + `delta vs brute` ，断言 `|delta_recall@5| < 0.01` 且 `|delta_ndcg@5| < 0.015`
  - `README.md` 追加 "向量索引" 小节说明阈值与 `force_brute` 调试开关
- **Acceptance:**
  - `cargo run -- eval-retrieval --mode vector --k 5` 与 `--force-brute` 的 Recall@5 差值 <1%，NDCG@5 差值 <1.5% (或记录实测 delta 并说明容忍)
  - `BASELINE.md` 已更新含 ANN 行，commit 中可复现
- **QA:**
  - 在合成 6000×64d 索引上自动化：`cargo test --no-default-features --test hnsw_regression` (新增集成测试，DeterministicEmbedder 构造 6000 向量，ANN vs brute recall>=0.95)
  - Gates 全绿
- **Commit:** `feat(index): add HNSW regression eval and baseline delta guard`

---

## Dependency Matrix

```
TODO-01 (trait scaffold) ──┐
TODO-02 (cohere client)  ──┼──→ TODO-03 (HybridSearchResult) ──→ TODO-04 (CLI+Agent 接线) ──→ TODO-05 (docs)
                             │
TODO-06 (metrics) ───────────┼──→ TODO-07 (queries.jsonl) ──→ TODO-08 (eval-retrieval CLI) ──→ TODO-10 (HNSW regression)
                             │                                              ↑
TODO-09 (HNSW integration) ──┴──────────────────────────────────────────────┘
                                                        (TODO-09 --force_brute 需与 TODO-08 flag 联动，建议 TODO-08 先合，TODO-09 以 PR 追加 flag 接线，或同批实现)
```

**执行顺序 (串行分批):**
1. Batch 1: TODO-01 → 02 → 03 → 04 → 05 (单 PR/单 commit 链，可合并为 2 commits: 01-03, 04-05)
2. Batch 2: TODO-06 → 07 → 08 (单 PR)
3. Batch 3: TODO-09 → 10 (单 PR，依赖 Batch 2 的 eval  harness)

**并行可能性:** 无。Batch 2 依赖 Batch 1 的 rerank flag (eval 需对比 rerank on/off)；Batch 3 依赖 Batch 2 的 eval harness 做回归门禁。

---

## Verification & Gates (per-Batch)

每个 Batch 合并前必跑 (worker 本地执行，指令原样):

```bash
cargo fmt --check
cargo clippy --no-default-features --all-targets -- -D warnings
cargo test --no-default-features
# Batch 2/3 追加:
cargo run -- eval-retrieval --mode hybrid --k 5 --json | head -20
# Batch 3 追加 HNSW 回归:
cargo run -- eval-retrieval --mode vector --k 5 --force-brute --json
```

- Agent-executed QA per todo: happy + failure 双路径，精确 invocation 已在各 TODO QA 小节列出，证据路径为 `cargo test` 输出 + `evals/retrieval/BASELINE.md` 数值。

---

## Must-NOT-Have

- 测试中零网络/零模型下载/零 API key (Cohere 相关均为 mock/构造测试)
- 不改快照二进制格式 (hnsw skip 序列化，load 时重建；HybridSearchResult 新增字段仅瞬态)
- 不删暴力扫描路径 (阈值下 + 回退 + `force_brute` 调试)
- 不提交 `config.toml` / `.embedcache` / `data/` 产物
- 新增依赖仅 `hnswlib-rs = "0.10"` (纯 Rust)，不引入 C++ 绑定

---

## Risks & Mitigations

| 风险 | 缓解 |
|------|------|
| Cohere rerank 4K ctx 截断长 chunk | chunk 600 字远小于 4K；仍对超长做前端截断+ warn |
| HNSW recall 回归 | P1 评测 harness 强制 delta 门禁 (<1%)，不达标不合并 |
| 快照重建耗时 (6000+ 条) | 实测秒级；超阈才重建，小 corpus 零成本 |
| Rerank 增加延迟/费用 | `--no-rerank` 开关常驻；P1 评测量化 latency vs NDCG 权衡记录到 BASELINE |

---

## References

- `AGENTS.md:10` CI gates, `Cargo.toml:14` fastembed optional, `src/index/hybrid.rs:24` RRF defaults
- Cohere Rerank v2 docs: `https://api.cohere.com/v2/rerank`, model `rerank-v3.5`, req `{model,query,documents,top_n}`, resp `{results:[{index,relevance_score}]}`, 100+ langs, $2/1K searches (tavily 2026-09-03)
- hnswlib-rs crate: `https://crates.io/crates/hnswlib-rs` 0.10 (Jan 2026),纯 Rust HNSW, API `Hnsw::new/insert/search`, bincode save/load
- 评测指标: Recall@k/MRR@k/NDCG@k (std RAG eval)

---

## Appendix: Worker Start

```bash
# 从本计划启动执行
git checkout -b feat/retrieval-upgrade
# 按 Todos 顺序实现 Batch 1 → 2 → 3，每项单独 commit
```

> Draft 状态: `approved` — 计划已落盘 `D:\rust\mao_agent\.omo\plans\retrieval-upgrade.md`，worker 可直接按 Todos 执行。
