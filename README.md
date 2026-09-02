# Mao Agent (毛泽东文献辩证认知推理引擎与向量数据库)

> High-performance Vector Database & Knowledge Retrieval Engine for Chinese Historical Literature.  
> 基于 Rust (2024 edition) 构建的高性能双路混合检索（Dense 向量 + Tantivy BM25 倒排索引）与辩证认知推演引擎。

---

## 📖 项目概述 (Overview)

`mao_agent` 是一个专为中文历史文献（如《毛泽东选集》、《毛泽东文集》）设计的单体（Single-crate bin + lib）知识检索与辩证认知推演引擎。

### 核心特性
- **双路混合检索 (Hybrid RRF Retrieval)**：
  - **Dense 向量检索**：高维稠密语义向量检索，支持本地 FastEmbed (ONNX BGE-small-zh-v1.5) 与云端 Cohere `embed-v4.0`；
  - **Sparse 全文检索**：基于 Tantivy 0.22 倒排索引与 Jieba 搜索引擎模式分词（`cut_for_search`）；
  - **RRF 排序融合**：倒数排名融合算法（Reciprocal Rank Fusion），自适应合并关键字与语义相关度。
- **历史文献结构化分块与清洗 (Corpus Pipeline)**：
  - YAML Frontmatter 元数据提取（历史时期、卷册、分类、作者、成文时间）；
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

# 运行完整测试套件 (34 个单元与集成测试)
cargo test --no-default-features

# 编译发布版本
cargo build --release
```

### 2. 配置说明 (Configuration)
复制配置文件模板：
```bash
cp config.example.toml config.toml
```
如需启用云端 Cohere `embed-v4.0` 嵌入与 `command-r7b` 问答模型，请在 `config.toml` 中填入你的 API Key（支持通过 `COHERE_API_KEY` 环境变量或 CLI 参数 `--api-key` 覆盖）。

---

## 🛠️ CLI 命令详解 (CLI Usage)

```
        ┌──────────────┐
        │ init-samples │ ──> 生成 4 篇经典文献 Markdown 语料 (corpus/*.md)
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
# 在 corpus/ 目录下生成《论持久战》、《矛盾论》、《实践论》、《星星之火，可以燎原》
```

### 2. 语料库摄取与索引构建
```bash
# 使用本地/离线嵌入器摄取
cargo run -- ingest --corpus-dir corpus --batch-size 32
```

### 3. 多模式文献检索 (Search)
```bash
# 默认混合检索 (Hybrid BM25 + Vector RRF)
cargo run -- search "持久战的三个阶段" --top-k 3

# 纯向量检索 (Dense Vector Only)
cargo run -- search "主要矛盾和矛盾的主要方面" --mode vector

# 纯全文关键词检索 (BM25 Only)
cargo run -- search "星星之火可以燎原" --mode bm25

# 结合历史时期与卷册过滤
cargo run -- search "统一战线" --period "抗日战争时期" --volume "毛泽东选集第二卷"
```

### 4. 向量数据库状态与健康度统计 (Stats)
```bash
cargo run -- stats
```

### 5. 辩证认知推演与引文核验问答 (Ask)
```bash
cargo run -- ask "抗日战争为什么是持久战？最后的胜利为什么属于中国？"
```

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
│   ├── assets/samples/            # 预置经典历史文献模板
│   ├── corpus/                    # 文档解析、CJK清洗与语义分块器
│   ├── vector/                    # 稠密向量存储、内存索引与 Embedding 模型
│   ├── index/                     # Tantivy 全文倒排索引与 RRF 融合协调器
│   └── agent/                     # 辩证认知推演引擎与引文真实性核验器
└── tests/                         # 5 个模块集成测试套件 (E2E & Store)
```
