---
name: dialectical-eval
description: Benchmark dialectical deduction reasoning and citation grounding verification in mao_agent. Use when testing changes to src/agent/engine.rs or src/agent/verifier.rs.
---

# Dialectical Evaluation Benchmark

Benchmark dialectical deduction reasoning and citation grounding verification in `mao_agent`. This skill provides a standardized verification runbook when making changes to the reasoning engine ([`src/agent/engine.rs`](file:///D:/rust/mao_agent/src/agent/engine.rs)), system prompt templates ([`src/agent/prompt.rs`](file:///D:/rust/mao_agent/src/agent/prompt.rs)), or the citation verifier ([`src/agent/verifier.rs`](file:///D:/rust/mao_agent/src/agent/verifier.rs)).

## Overview

Mao Agent implements a dialectical materialist epistemology workflow:
1. **Hybrid Retrieval**: Queries vector store and BM25 full-text index for relevant historical literature chunks.
2. **Dialectical Synthesis**: Organizes reasoning into four structured epistemology stages.
3. **Citation Verification**: Extracts quoted passages (`"..."` or `“...”`) and performs true-substring / sliding-window fuzzy matching against retrieved chunks to prevent hallucinated citations.

---

## Evaluation Workflow

### Step 1: Run Benchmark Question

Execute the standard dialectical evaluation query using `--no-default-features` for fast compilation:

```bash
cargo run --no-default-features -- ask "抗日战争为什么是持久战？最后的胜利为什么属于中国？"
```

#### Diagnostic Variations
- **Deterministic Offline Path**: To evaluate the deterministic template in [`src/agent/engine.rs`](file:///D:/rust/mao_agent/src/agent/engine.rs) without external LLM API calls, append `--offline`:
  ```bash
  cargo run --no-default-features -- ask --offline "抗日战争为什么是持久战？最后的胜利为什么属于中国？"
  ```
- **Context Depth**: Adjust `-k <N>` (e.g. `-k 5`) to test verifier robustness under varying retrieval chunk counts:
  ```bash
  cargo run --no-default-features -- ask -k 5 "抗日战争为什么是持久战？最后的胜利为什么属于中国？"
  ```
- **Period-Filtered Verification**: Test dialectical deduction across historical periods (e.g., Socialist Construction or War of Liberation):
  ```bash
  cargo run --no-default-features -- ask "怎样正确处理人民内部矛盾？" --period "社会主义革命和建设时期"
  ```

---

### Step 2: Verify the Four Dialectical Epistemology Stages

Inspect the output under `【Mao Agent 辩证认知推理引擎】` and verify that the response strictly structures deduction across all four epistemology stages:

1. **一、 调查研究 (Investigation / Fact-Finding & Evidence)**:
   - Must cite objective historical facts and literature evidence retrieved from the corpus.
   - Must include document metadata (title, publication date, historical period, e.g. 《论持久战》 1938-05-26 · 抗日战争时期).
2. **二、 主要矛盾分析 (Contradiction Analysis / Principal Contradiction)**:
   - Must identify the fundamental opposing forces (e.g. 日本帝国主义退步野蛮 vs 中国半殖民地半封建进步正义).
   - Must analyze the principal aspect of the contradiction (矛盾的主要方面) and quantitative vs qualitative dynamics (地大物博人多 vs 国小物匮人少).
3. **三、 理论综合 (Synthesis / Dialectical Synthesis)**:
   - Must apply materialist dialectics (internal causes as fundamental basis, external causes as operating condition).
   - Must articulate the three strategic phases of protracted warfare (战略防御 -> 战略相持 -> 战略反攻).
4. **四、 指导实践与方针策略 (Practice / Action Policy & Conclusions)**:
   - Must derive concrete, actionable strategic doctrines (e.g. 坚持抗战、游击战配合、兵民是胜利之本).
   - Must conclude with an affirmative, dialectical answer to the question.

---

### Step 3: Verify Citation Attribution & Grounding

Examine the `🔍 引用溯源与真子串核验报告 (Attribution Verification)` and `📚 支撑文献依据 (Retrieved Context)` sections in the CLI output:

1. **Verification Status**:
   - For all extracted quotes, verify that the report displays:
     ```text
     ✅ [真子串核验通过] 置信度: 100.0% | 《论持久战》
     ```
     or sliding-window fuzzy confidence >= 85.0%.
   - Ensure there are no unexpected `⚠️ [存疑/未匹配]` flags on authentic excerpts.
2. **Corpus Ground-Truth Alignment**:
   - Confirm that cited text segments physically exist in [`corpus/lun_chi_jiu_zhan.md`](file:///D:/rust/mao_agent/corpus/lun_chi_jiu_zhan.md) or corresponding chunks.
   - Verify that any fabricated or mutated quotes are accurately flagged with low confidence (< 85%) or appropriate warnings (e.g., quote length < 6 characters).
3. **Source Chunks**:
   - Check that the `📚 支撑文献依据` lists valid chunk IDs from the corpus (e.g. `doc_ef42a9052621854d_chunk_0001`).

---

## Evaluation Checklist

Before committing changes to `src/agent/engine.rs` or `src/agent/verifier.rs`:

- [ ] `cargo check --no-default-features --quiet` passes with exit code 0.
- [ ] `cargo test --no-default-features` passes.
- [ ] Benchmark inquiry executes and exits with code 0.
- [ ] All four epistemology stages (调查研究, 主要矛盾分析, 理论综合, 指导实践) are present in sequence.
- [ ] Citation verifier correctly validates quotes with `is_verified: true` and confidence >= 85%.
