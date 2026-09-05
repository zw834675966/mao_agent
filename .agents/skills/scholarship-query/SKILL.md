---
name: scholarship-query
description: Guides searching, cross-referencing, and synthesizing academic scholarship viewpoints from renowned Chinese universities (Peking, Tsinghua, Renmin University) and international Sinologists (Stuart Schram, Maurice Meisner, Nick Knight, John King Fairbank, Frederic Wakeman). Use when researching modern philosophical perspectives, academic debates, or comparative Sinology on Mao Zedong Thought.
---

# Academic Scholarship Query Skill

Guides the process of querying, cross-referencing, and synthesizing modern academic scholarship on Mao Zedong Thought from top Chinese universities (Peking University, Tsinghua University, Renmin University of China) and international Sinologists.

## Dual-Track Architecture Principle

> [!IMPORTANT]
> The `mao_agent` corpus maintains a strict dual-track separation:
> 1. **Primary Canonical Works (`category: "哲学与调查研究"`, `"唯物辩证法"`, etc.)**: Pure historical works by Mao Zedong for rigorous citation grounding.
> 2. **Academic Scholarship (`category: "高校学术研究"`, `"海外中国学研究"`)**: Academic papers and authoritative monographs from contemporary scholars.

---

## Step-by-Step Retrieval Workflows

### 1. Querying Domestic University Scholars

To retrieve viewpoints from scholars at Peking, Tsinghua, or Renmin University:

```bash
# Search for Renmin University Prof. Chen Xianda's analysis of contradiction theory
cargo run --no-default-features -- search "陈先达 矛盾论" --top-k 3

# Search for Tsinghua University Prof. Wang Hui's protracted war philosophy
cargo run --no-default-features -- search "汪晖 持久战" --top-k 3

# Search for Peking University Prof. Zhang Yixing's categorical system
cargo run --no-default-features -- search "张翼星 哲学范畴" --top-k 3
```

### 2. Querying International Sinologists & Overseas Scholars

To retrieve analyses from renowned international experts:

```bash
# Query Stuart Schram on Sinification of Marxism and two essays
cargo run --no-default-features -- search "施拉姆 实践论" --top-k 3

# Query Frederic Wakeman on Wang Yangming unity of knowledge and action
cargo run --no-default-features -- search "魏斐德 知行合一" --top-k 3

# Query John King Fairbank on the Yan'an institutional model
cargo run --no-default-features -- search "费正清 延安模式" --top-k 3

# Query Nick Knight's rebuttal of voluntarism
cargo run --no-default-features -- search "奈特 唯意志论" --top-k 3
```

### 3. Cross-Referencing Primary Works with Secondary Commentary

To compare how academic scholarship evaluates a specific primary concept:

1. **Step A**: Query the primary text:
   ```bash
   cargo run --no-default-features -- search "实事求是" --period "抗日战争时期" --top-k 2
   ```
2. **Step B**: Query contemporary scholars' synthesis:
   ```bash
   cargo run --no-default-features -- search "陈先达 实事求是 哲学意蕴" --top-k 2
   ```
3. **Step C**: Verify citation grounding integrity by ensuring the agent attributes primary quotes only to primary canon while citing academic analyses separately.

---

## Validation Checklist

- [ ] Hybrid search returns appropriate scholarship documents from `data/tantivy_index/` and `data/vector_store.bin`.
- [ ] Primary canon quotes maintain 100% true-substring match without confusing secondary scholars with primary authors.
- [ ] Category filtering (`--category "高校学术研究"` or `--category "海外中国学研究"`) correctly isolates secondary literature when needed.
