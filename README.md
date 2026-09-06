# Mao Agent (毛泽东文献辩证认知推理引擎与向量数据库)

> High-performance Vector Database & Knowledge Retrieval Engine for Chinese Historical Literature.  
> 基于 Rust (2024 edition) 构建的高性能双路混合检索（Dense 向量 + Tantivy BM25 倒排索引）与辩证认知推演引擎。

---

## 📖 项目概述 (Overview)

`mao_agent` 是一个专为中文历史文献（如《毛泽东选集》、《毛泽东文集》）设计的单体（Single-crate bin + lib）知识检索与辩证认知推演引擎。

### 核心特性
- **双路混合检索 (Hybrid RRF Retrieval)**：
  - **Dense 向量检索**：高维稠密语义向量检索，支持 **Google Gemini (`gemini-embedding-2`, 推荐 768 维)**、云端 Cohere `embed-v4.0` (1536 维) 与本地 FastEmbed (ONNX BGE-small-zh-v1.5, 512 维)；
  - **Sparse 全文检索**：基于 Tantivy 0.22 倒排索引与 Jieba 搜索引擎模式分词（`cut_for_search`）；
  - **RRF 排序融合**：倒数排名融合算法（Reciprocal Rank Fusion），自适应合并关键字与语义相关度。
  - **HNSW ANN**：索引达到 5000 向量后自动启用 hnswlib-rs API（经 hnsw-stable 稳定版依赖）近似近邻；快照不持久化图、加载时重建；召回对比请用 `eval-retrieval --force-brute`（该 flag 仅存在于 eval-retrieval，`search` 不支持）。
  - **Cohere Rerank 精排**：hybrid 默认 `fuse(top_k*2) → rerank-v3.5 → top_k`；`--no-rerank` / `COHERE_RERANK_MODEL` 可覆盖，offline 或无 key 自动降级。
- **多领域文献与工程知识库 (Corpus Pipeline)**：
  - 覆盖毛泽东经典文献、Hacker Laws（编程法则与原则）、Papers We Love（经典学术论文与原版 PDF）、Awesome Scalability（高可用分布式架构）与 Hello Algo（数据结构与算法）；
  - 统一 YAML Frontmatter 元数据提取（历史时期、卷册、分类、作者、成文时间、标签）；
  - 针对历史文献排版与 OCR 扫描的 CJK 标点/空格清洗；
  - 层次化语义分块与元数据注入。
- **辩证认知推演与真子串引文核验 (Dialectical Agent & Verifier)**：
  - 基于唯物辩证法四大认识论阶段（调查研究、主要矛盾分析、理论综合、指导实践）生成论证推演；
  - 自动抽取引文并基于 `strsim` / 准确子串算法对正文文献进行证据溯源与真实性核验（Citation Grounding）。

---

## 🚀 快速上手 (Quick Start)

### 1. 环境准备与编译
```bash
# 检查编译 (使用轻量 hash 嵌入器，无需下载模型)
cargo check --no-default-features

# 运行完整测试套件 (134 个单元与集成测试，含 Gemini / API / HNSW / Graph / citation adversarial)
cargo test --no-default-features

# 编译发布版本
cargo build --release
```

### 2. 配置说明 (Configuration)
复制配置文件模板：
```bash
cp config.example.toml config.toml
```
生产嵌入走 **硅基流动 SiliconFlow**（`BAAI/bge-m3`，1024 维）。在 `config.toml` 填写（或 `SILICONFLOW_API_KEY`），**不要把真实密钥写进仓库**：
```toml
[siliconflow]
api_key = ""
base_url = "https://api.siliconflow.cn/v1"
model = "BAAI/bge-m3"
dimension = 1024
```
批量入库建议 `--batch-size 16~32`（默认 32）。免费额度约 2000 RPM / 500k TPM；远程批次之间 CLI 会间隔 100ms，降低 HTTP 429。

可选：`[gemini]` 做 768 维嵌入；`[cohere]` 仅用于 chat / rerank。无 Cohere key 时 `ask` 走离线四阶段模板。`--embed-provider` 可强制指定。多个 key 同时存在时 **SiliconFlow 优先于 Gemini**。

---

## 🛠️ CLI 命令详解 (CLI Usage)

```
        ┌──────────────┐
        │ init-samples │ ──> 生成 15 篇经典文献与学术研究语料 (corpus/*.md)
        └──────┬───────┘
               │
               ▼
        ┌──────────────┐
        │    ingest    │ ──> 构建向量索引快照 (data/vector_store.bin)
        └──────┬───────┘     与 Tantivy 倒排索引 (data/tantivy_index/)
               │
       ┌───────┴───────┐
       ▼               ▼
┌──────────────┐┌──────────────┐
│    search    ││     ask      │
└──────────────┘└──────────────┘
```

### 1. 初始化示例语料
```bash
cargo run -- init-samples
```
在 `corpus/` 目录下生成全部 15 篇示例语料（13 篇经典原著 + 2 篇权威学术研究辑要）：
1. 《中国社会各阶级的分析》 (1925)
2. 《湖南农民运动考察报告》 (1927)
3. 《星星之火，可以燎原》 (1930)
4. 《反对本本主义》 (1930)
5. 《实践论》 (1937)
6. 《矛盾论》 (1937)
7. 《论持久战》 (1938)
8. 《改造我们的学习》 (1941)
9. 《关于领导方法的若干问题》 (1943)
10. 《在中国共产党第七届中央委员会第二次全体会议上的报告》 (1949)
11. 《论人民民主专政》 (1949)
12. 《关于正确处理人民内部矛盾的问题》 (1957)
13. 《人的正确思想是从哪里来的？》 (1963)
14. 《当代名校名家毛泽东思想与辩证法研究论著集萃》（北京大学、清华大学、中国人民大学等国内学界代表性成果）
15. 《海外著名学者毛泽东思想与辩证法研究代表性论著集萃》（施拉姆、迈斯纳、奈特、费正清、魏斐德等国际海外汉学代表性成果）

### 2. 语料库摄取与索引构建
ingest 与 search/ask **必须使用同一嵌入后端**。混用 `--offline` 与 FastEmbed/Cohere 会因模型或维数不匹配而失败（需重新 ingest）。

```bash
# 使用 Google Gemini 向量模型（推荐 768 维，支持自动读取 config.toml 或 GEMINI_API_KEY）
cargo run -- ingest --corpus-dir corpus --batch-size 32

# 无网 / 无 API key：全程 --offline（确定性 hash，默认 512 维）
cargo run -- ingest --offline --corpus-dir corpus --batch-size 32

# 本地 FastEmbed ONNX BGE-small-zh-v1.5（默认 feature，512 维；首次会下载模型）
# 不要与 --offline 混用
cargo run -- ingest --corpus-dir corpus --batch-size 32
```

### 3. 多模式文献检索 (Search)
Hybrid 模式默认在 RRF 融合后调用 Cohere Rerank（`rerank-v3.5`，`POST https://api.cohere.com/v2/rerank`）。有 `COHERE_API_KEY` / `EMBED_API_KEY` / `config.toml [cohere].api_key` 时自动启用；`--offline`、`--no-rerank` 或无 key 时跳过并保留融合顺序。可用 `--rerank-model` / `COHERE_RERANK_MODEL` 覆盖模型。

```bash
# 使用 Gemini 向量进行混合检索
cargo run -- search "墨菲定律与高可用容灾" --top-k 5

# 与上一节 --offline ingest 配对（offline ⇒ 不 rerank）
cargo run -- search --offline "持久战的三个阶段" --top-k 3

# 与 FastEmbed ingest 配对（不要加 --offline）
cargo run -- search "持久战的三个阶段" --top-k 3

# 显式关闭精排 / 指定模型
cargo run -- search "持久战的三个阶段" --no-rerank
cargo run -- search "持久战的三个阶段" --rerank-model rerank-v3.5

# 纯向量 / 纯 BM25
cargo run -- search --offline "主要矛盾和矛盾的主要方面" --mode vector
cargo run -- search --offline "星星之火可以燎原" --mode bm25

# 结合历史时期与卷册过滤
cargo run -- search --offline "统一战线" --period "抗日战争时期" --volume "毛泽东选集第二卷"
```

### 4. 向量数据库状态与健康度统计 (Stats)
```bash
# 与 --offline ingest 配对
cargo run -- stats --offline
```

### 5. 辩证认知推演与引文核验问答 (Ask)
无 API key 时走离线辩证模板。嵌入后端仍须与 ingest 一致。Hybrid 召回同样支持 Cohere Rerank（`--no-rerank` / `--rerank-model` / `COHERE_RERANK_MODEL`，语义同 search）：

```bash
cargo run -- ask --offline "抗日战争为什么是持久战？最后的胜利为什么属于中国？"
cargo run -- ask "抗日战争为什么是持久战？" --no-rerank
```

### 6. 检索质量评估 (Eval Retrieval)
离线 gold 查询集上计算 Recall / MRR / NDCG@k（默认 `evals/retrieval/queries.jsonl`，约 100+ 条）。可选 `--force-brute`（仅 `eval-retrieval`，不在 `search`）关闭 HNSW 做召回对比；`--no-rerank` 保留融合顺序基线。

```bash
cargo run -- eval-retrieval --offline --no-rerank --k 5
cargo run -- eval-retrieval --offline --force-brute --json
```

基线说明见 `evals/retrieval/BASELINE.md`。

### 7. HTTP API 服务 (Serve, Axum + Tokio)
将混合检索 / 辩证推演 / 引文核验封装为 REST + SSE 微服务，供上游业务系统或 Agent 调用。嵌入后端仍须与 ingest 一致：

```bash
cargo run -- serve --offline --bind 127.0.0.1:3000
```

| 方法与路径 | 说明 |
|---|---|
| `GET /health` / `GET /api/v1/health` | 存活与索引状态（chunks 数、维度、tantivy 是否加载） |
| `GET /api/v1/stats` | 向量库统计（时期/卷册分布、内存预估） |
| `POST /api/v1/search` | 原子检索：`{query, top_k≤20, mode: hybrid\|vector\|bm25, period/volume/category/tags/start_date/end_date/doc_id/keyword, min_score, no_rerank}`；hybrid 结果可含 `rerank_score` |
| `POST /api/v1/ask` | 端到端推演（阻塞 JSON）：`{question, top_k≤10, period/volume, base_url/model/api_key}`，`api_key` 也可走 `Authorization: Bearer` 头 |
| `POST /api/v1/ask/stream` | 端到端推演（SSE）：事件 `retrieved → reranked → delta(stage) → citation → done` |
| `POST /api/v1/verify`（别名 `/api/v1/citation/verify`） | 引文核验：`{quote, claimed_title, context_chunks, min_confidence}`，返回真子串/模糊匹配报告 |

```bash
# 检索示例
curl -X POST http://127.0.0.1:3000/api/v1/search \
  -H 'Content-Type: application/json' \
  -d '{"query":"持久战三个阶段","top_k":3,"mode":"hybrid"}'

# SSE 推演示例（retrieved → reranked → delta → citation → done）
curl -N -X POST http://127.0.0.1:3000/api/v1/ask/stream \
  -H 'Content-Type: application/json' \
  -d '{"question":"抗日战争为什么是持久战？","top_k":2}'
```

说明：`serve` 启动时加载 `data/vector_store.bin` + `data/tantivy_index`（缺失则自动降级并告警）；LLM 走 OpenAI 兼容协议（默认 Cohere，可配 DeepSeek/Qwen/本地 Ollama），无 key 时 `ask` 自动用离线辩证模板。

---

## 🧪 测试与覆盖率度量 (Testing & Coverage)

### 运行测试
```bash
# 常规快速测试
cargo test --no-default-features

# 运行特定集成测试
cargo test --no-default-features --test vector_store_test
```

### 使用 cargo-llvm-cov 度量测试覆盖率
```bash
# 1. 安装 LLVM 覆盖率工具
cargo install cargo-llvm-cov

# 2. 生成终端覆盖率摘要
cargo llvm-cov --no-default-features

# 3. 生成完整 HTML 交互式覆盖率报告
cargo llvm-cov --no-default-features --html --output-dir target/llvm-cov-target
```

---

## 🏛️ 项目结构 (Code Architecture)

```
mao_agent/
├── .github/workflows/ci.yml       # GitHub Actions 自动化 CI 流水线 (fmt/clippy/test)
├── Cargo.toml                     # 项目依赖与 Features 定义
├── AGENTS.md                      # AI Agent 运行与协同准则
├── README.md                      # 用户手册与项目文档
├── config.example.toml            # 配置文件模板
├── corpus/                        # 示例历史语料 Markdown 文件
├── src/
│   ├── main.rs                    # CLI 主程序与命令处理入口
│   ├── lib.rs                     # 核心库导出公共 API
│   ├── config.rs                  # 配置文件解析与环境发现
│   ├── model.rs                   # 数据模型 (Document, Chunk, Period)
│   ├── error.rs                   # 统一错误处理枚举 (thiserror)
│   ├── cli/                       # Clap 强类型命令行参数定义
│   ├── corpus/                    # 文档解析、CJK清洗与语义分块器
│   ├── vector/                    # 稠密向量存储、HNSW ANN、Embedding
│   ├── index/                     # Tantivy 全文倒排索引与 RRF 融合协调器
│   ├── graph/                     # 知识图谱扩展 (DiGraph、拓扑扩展与候选注入)
│   ├── rerank/                    # Cohere Rerank trait + client（mock 可测）
│   ├── eval/                      # Recall / MRR / NDCG@k 检索指标
│   ├── agent/                     # 辩证认知推演引擎与引文真实性核验器
│   └── server/                    # Axum HTTP API：DTO/路由/检索·推演SSE·核验 handlers
├── evals/retrieval/               # gold queries.jsonl + BASELINE.md
└── tests/                         # 10 个集成测试套件 (E2E / Store / API / HNSW / Graph)
```

