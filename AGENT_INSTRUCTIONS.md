# Mao Agent - Agent Operational & Tool Specification
<!-- Machine & Autonomous Agent Capability Specification -->

> **Version**: 0.1.0  
> **Language**: Rust (2024 Edition)  
> **Crate Type**: Single crate (`bin` + `lib`)  
> **Domain**: Chinese Historical Literature Vector DB, Hybrid Search (Dense + BM25 RRF), and Dialectical Cognitive Deduction Engine.

---

## 1. Executive Summary for Autonomous Agents

This document is the **machine-readable and agent-executable interface specification** for `mao_agent`. Any AI agent entering this workspace can use this specification to autonomously:
1. **Identify** the domain capabilities of this repository;
2. **Execute** deterministic CLI commands or invoke Rust library APIs;
3. **Parse** structured outputs (search results, dialectical reasoning stages, citation verification reports);
4. **Self-heal** across edge cases (missing indexes, offline fallbacks, secret safety).

---

## 2. Core Capabilities Matrix

| Capability ID | Capability Name | Description | Primary CLI Subcommand | Rust API Module |
| :--- | :--- | :--- | :--- | :--- |
| `CAP-SEARCH-HYBRID` | **Dual-Path Hybrid Search** | Combines Dense Vector embeddings and Tantivy BM25 inverted index with Jieba segmentation using Reciprocal Rank Fusion (RRF). | `search "<query>"` | [`mao_agent::index::HybridSearchCoordinator`](file:///D:/rust/mao_agent/src/index/hybrid.rs) |
| `CAP-REASON-DIALECTIC` | **Dialectical Q&A Engine** | Synthesizes historical evidence through the 4 materialist dialectic stages: Investigation, Contradiction Analysis, Synthesis, and Practice. | `ask "<question>"` | [`mao_agent::agent::DialecticalAgent`](file:///D:/rust/mao_agent/src/agent/engine.rs) |
| `CAP-VERIFY-CITATION` | **Citation Grounding Guard** | Verifies quoted texts against retrieved physical corpus chunks using exact substring & sliding-window fuzzy confidence algorithms to eliminate hallucinations. | `ask "<question>"` | [`mao_agent::agent::CitationVerifier`](file:///D:/rust/mao_agent/src/agent/verifier.rs) |
| `CAP-INGEST-PIPELINE` | **CJK Corpus Ingestion** | Cleans OCR spaces, extracts YAML frontmatter, applies hierarchical semantic chunking, and produces vector/BM25 snapshots. | `ingest` / `init-samples` | [`mao_agent::corpus::ChineseSemanticChunker`](file:///D:/rust/mao_agent/src/corpus/chunker.rs) |
| `CAP-STORE-STATS` | **Index Health & Metrics** | Reports indexed chunk count, active vector dimensionality, and storage paths. | `stats` | [`mao_agent::vector::VectorStore`](file:///D:/rust/mao_agent/src/vector/store.rs) |

---

## 3. Autonomous Execution Protocol (CLI Subprocesses)

### 3.1 The Golden Rule for AI Agents
> [!IMPORTANT]
> **Always append `--no-default-features` to all `cargo run` and `cargo test` invocations.**  
> - Why: The default feature (`local-embed`) compiles heavy ONNX dependencies and attempts to download a multi-hundred MB model on first run.  
> - `--no-default-features` switches to deterministic, fast, offline hash embeddings without network dependency.

---

### 3.2 Command Reference & Invocation Schemas

#### A. Document & Literature Retrieval (`search`)
Use this tool to find relevant historical passages, quotes, or evidence.

```bash
cargo run --no-default-features -- search "<query>" [OPTIONS]
```

*   **Parameters**:
    *   `<query>` *(string, required)*: The search topic or keywords (e.g. `"持久战的三个阶段"`).
    *   `-k, --top-k <N>` *(integer, optional, default: 5)*: Number of context chunks to return.
    *   `--mode <mode>` *(string, optional, default: "hybrid")*:
        *   `hybrid`: Best overall accuracy. Dense vector + Tantivy BM25 with RRF fusion.
        *   `vector`: Semantic similarity only.
        *   `bm25`: Keyword and exact term matching only.
    *   `--period <period>` *(string, optional)*: Filter by historical period (e.g. `"抗日战争时期"`, `"土地革命战争时期"`, `"建国以后"`).
    *   `--volume <volume>` *(string, optional)*: Filter by selected volume (e.g. `"毛泽东选集第二卷"`).
    *   `--category <category>` *(string, optional)*: Filter by document category (e.g. `"军事"`, `"哲学"`, `"党建"`).

*   **Expected Output Format** (hybrid mode):
    ```text
    🔍 执行检索 [模式: hybrid]: "<query>" (Top-3)
    ⚡ 双路混合 (BM25 + 向量 RRF) 检索耗时: 1.01s，融合召回 3 条结果

    🏆 [Rank 1] RRF得分: 0.01639 (向量: 0.4713, BM25: 5.48) | 《论持久战》 (1938-05-26)
    📌 时期: 抗日战争时期 (1937-1945) | 卷册: 毛泽东选集第二卷

    📖 原文段落:
    ...中日战争的特点是：日本是帝国主义强国，但退步野蛮...
    ```

---

#### B. Dialectical Consultation & Grounded Answering (`ask`)
Use this tool to obtain structured, epistemologically grounded analyses for strategic, historical, or philosophical questions.

```bash
cargo run --no-default-features -- ask "<question>" [OPTIONS]
```

*   **Parameters**:
    *   `<question>` *(string, required)*: The consultation prompt (e.g. `"抗日战争为什么是持久战？"`).
    *   `--offline` *(flag, optional)*: Force offline deterministic reasoning without calling remote LLM APIs.
    *   `-k, --top-k <N>` *(integer, optional, default: 3)*: Context chunks retrieved to support deduction.
    *   `--period <period>` *(string, optional)*: Restrict evidence retrieval to a specific historical epoch.

*   **Output Structure**:
    The stdout is divided into three deterministic sections:
    1. **`### 一、 调查研究 (Fact-Finding & Evidence)`**: Concrete facts and context extracted from corpus.
    2. **`### 二、 主要矛盾分析 (Principal Contradiction)`**: Fundamental contradictions and principal aspects.
    3. **`### 三、 理论综合 (Dialectical Synthesis)`**: Internal vs external conditions and strategic phases.
    4. **`### 四、 指导实践与方针策略 (Action Policy & Conclusions)`**: Concrete action doctrines.
    5. **`🔍 引用溯源与真子串核验报告 (Attribution Verification)`**:
       - Status: `✅ [真子串核验通过]` (Confidence 100.0%) or `⚠️ [存疑/未匹配]`.
    6. **`📚 支撑文献依据 (Retrieved Context)`**: List of matching chunks with document titles and dates.

---

#### C. Health & Status Inspection (`stats`)
Use this tool before performing operations to verify whether indexes are loaded and healthy.

```bash
cargo run --no-default-features -- stats
```

*   **Healthy output**: prints chunk/document counts, vector dimension, character totals, memory estimate, and period/volume distributions.
*   **Missing index**: prints `❌ 索引文件未找到: <path>` to stderr; the process still exits `0`, so probe `data/vector_store.bin` existence (or check for the `❌` line) before search/ask. Trigger the Ingestion Pipeline below to rebuild.

---

#### D. Index Initialization & Rebuild Pipeline (`ingest-pipeline`)
Use this tool if `stats` indicates missing indexes (`data/vector_store.bin` or `data/tantivy_index/`).

```bash
# Step 1: Initialize corpus Markdown files (creates corpus/*.md)
cargo run --no-default-features -- init-samples

# Step 2: Build vector store snapshot & Tantivy inverted index
cargo run --no-default-features -- ingest --corpus-dir corpus --batch-size 32
```

---

## 4. Programmatic Rust API Protocol (Direct Crate Integration)

If your agent develops or compiles Rust code depending on `mao_agent`, import via `src/lib.rs`:

```rust
use std::sync::Arc;
use mao_agent::index::HybridSearchCoordinator;
use mao_agent::vector::{EmbedderSelection, VectorStore, resolve_embedder};
use mao_agent::agent::DialecticalAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Resolve deterministic offline embedder (zero network)
    let selection = EmbedderSelection {
        offline: true,
        api_key: None,
        base_url: None,
        model: "deterministic".into(),
        dimension: 1536,
    };
    let embedder = resolve_embedder(&selection, None)?;
    
    // 2. Load persisted vector store snapshot (identity-checked against the embedder)
    let store = Arc::new(VectorStore::load_from_file("data/vector_store.bin", embedder)?);
    
    // 3. Open Tantivy full-text index
    let tantivy = Arc::new(mao_agent::index::FullTextIndex::new_in_dir("data/tantivy_index")?);
    
    // 4. Dual-path hybrid search with RRF fusion (mirrors CLI `search --mode hybrid`)
    let vec_results = store.search("持久战", 10, None).await?;
    let bm25_results = tantivy.search("持久战", 10, None)?;
    let coordinator = HybridSearchCoordinator::default();
    let results = coordinator.fuse(vec_results, bm25_results, 5);
    
    // 5. Run dialectical deduction and citation verification
    //    (no API key configured → deterministic offline answer template)
    let agent = DialecticalAgent::new(store, Some(tantivy), None, None, None);
    let answer = agent.ask("抗日战争为什么是持久战？", 3, None).await?;

    println!("Reasoning: {}", answer.content);
    println!("Fully grounded: {}", answer.is_fully_grounded);
    Ok(())
}
```

---

## 5. Output Parsing Guide for Agents

### 5.1 Parsing Search Results
*   **Result entry line** — the score label depends on `--mode`:
    *   Hybrid: `🏆 [Rank 1] RRF得分: 0.01639 (向量: 0.4713, BM25: 5.48) | 《论持久战》 (1938-05-26)`
    *   Vector: `🏆 [Rank 1] 相似度得分: 0.4713 | 《论持久战》 (1938-05-26)`
    *   BM25: `🏆 [Rank 1] BM25得分: 5.4772 | 《论持久战》 (1938-05-26)`
    *   Regex (hybrid): `^\s*🏆 \[Rank (\d+)\] RRF得分: ([\d.]+) \(向量: ([\d.]+|N/A), BM25: ([\d.]+|N/A)\) \| 《([^》]+)》 \(([^)]+)\)`
        *   Group 1: Rank · Group 2: RRF score · Group 3: vector score (`N/A` if the chunk had no BM25 hit) · Group 4: BM25 score (`N/A` likewise) · Group 5: Document Title · Group 6: Date (`YYYY-MM-DD`)
*   **Metadata line**: `📌 时期: 抗日战争时期 (1937-1945) | 卷册: 毛泽东选集第二卷`
*   **Chunk ID**: printed only by `ask` in the `📚 支撑文献依据` section — regex `Chunk ID:\s+([a-zA-Z0-9_]+)`. `search` output does not include chunk IDs.

### 5.2 Parsing Dialectical Deduction Stages
*   Split stdout by headings:
    *   `### 一、 调查研究` $\to$ Objective evidence.
    *   `### 二、 主要矛盾分析` $\to$ Contradiction breakdown.
    *   `### 三、 理论综合` $\to$ Strategic synthesis.
    *   `### 四、 指导实践与方针策略` $\to$ Action policies.
    *   `🔍 引用溯源与真子串核验报告` $\to$ Verification stats.

---

## 6. Self-Healing & Failure Recovery Runbook

| Failure Symptom | Root Cause | Automated Self-Healing Action for Agent |
| :--- | :--- | :--- |
| `❌ 向量索引文件未找到: data/vector_store.bin`（search/stats/ask 的友好提示） | Indexes have not been generated yet. | Execute: `cargo run --no-default-features -- init-samples && cargo run --no-default-features -- ingest --corpus-dir corpus` |
| Command hangs for minutes during first build or run | Default feature compiled `fastembed` and is downloading ONNX model. | Kill task and re-run with `--no-default-features`. |
| `Missing API Key` or `Remote API error: 401 Unauthorized` | Remote LLM key not provided in `config.toml` or env. | Append `--offline` flag to `ask` command to switch to deterministic dialectical reasoning engine. |
| Verification Report shows `⚠️ [存疑/未匹配]` | The generated answer contained a fabricated quote not found in ground truth. | Check chunk ID in `📚 支撑文献依据` and inspect raw text in [`corpus/`](file:///D:/rust/mao_agent/corpus) directly. |
| `TargetFile: data/vector_store.bin access denied` | Hook safety policy blocked manual alteration. | Do not manually edit files in `data/`. Regenerate via `cargo run --no-default-features -- ingest`. |

---

## 7. Operational Boundaries & Governance
1. **Never edit `data/` artifacts directly**: Always use the ingest pipeline.
2. **Never commit API keys**: Keep `config.toml` gitignored and rely on environment variables (`COHERE_API_KEY`).
3. **Pre-commit Gate**: Always verify with `cargo test --no-default-features` before concluding changes.
