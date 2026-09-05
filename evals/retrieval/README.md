# Retrieval Evaluation Set

Gold queries for offline retrieval metrics (Recall@k / MRR@k / NDCG@k).

## Format (`queries.jsonl`)

UTF-8 NDJSON, one object per line:

| Field | Type | Notes |
|-------|------|-------|
| `query` | string | Natural-language Chinese question |
| `expected_chunk_ids` | string[] | Non-empty gold chunk IDs (`{doc_id}_chunk_{idx:04}`) |
| `filter` | object \| null | Optional `{ "period": "抗日战争时期" }` (volume/category reserved) |

## Generation

Deterministic (no LLM). Pipeline:

1. `CorpusScanner` loads `corpus/*.md` (15 sample docs)
2. `ChineseSemanticChunker` matching ingest (`max_chars=600`, `min_chars=100`, `overlap_chars=50`)
3. Template questions from key sentences → map to `chunk_id`

Regenerate:

```bash
cargo run --no-default-features --example gen_retrieval_queries
```

## Coverage

- **105** queries (target 90–110)
- All **15** sample documents
- Periods (≥5 each for the locked four): 抗日战争 / 土地革命战争 / 全国解放战争 / 社会主义革命和建设 (also includes 第一次国内革命战争)

## Human spot-check (5 examples)

Verified that the quoted stem appears in the source markdown and the gold `chunk_id` was produced by the production chunker.

1. **抗日战争** — query: `请检索《论持久战》中阐述「伟大抗日战争的一周年纪念，快要到了」的段落。` → `doc_021ce9c4d7271db1_chunk_0000` (filter: 抗日战争时期)
2. **土地革命** — query: `毛泽东在《星星之火，可以燎原》里关于「对于时局的估量，必须认识到中国是一个许多帝国主义」提出了哪些论断？` → `doc_2bb308274c50ccec_chunk_0000`
3. **解放战争** — query: `毛泽东在《论人民民主专政》里关于「一九四九年的七月一日这一个日子表示，中国共产党已」提出了哪些论断？` → `doc_f7aafbe9d2349f46_chunk_0000`
4. **社会主义建设** — query: `根据《关于正确处理人民内部矛盾的问题》，如何理解「关于正确处理人民内部矛盾的问题，是一个总题目」？` → `doc_7bad38ab80f64d58_chunk_0000`
5. **土地革命 / 哲学** — query: `《实践论》中「马克思主义者认为人类的生产活动是最基本的实践活动」相关论述的核心观点是什么？` → `doc_1912121e30896ac4_chunk_0000`

## Baseline

See [`BASELINE.md`](./BASELINE.md) for offline Hybrid / Vector / BM25 metrics (`--no-default-features`, DeterministicEmbedder).
## Hard subset (`queries_hard.jsonl`)

Hand-authored adversarial queries (**≥20**) that avoid long quoted stems:

| `kind` | Intent |
|--------|--------|
| `paraphrase` | Ask for the concept without copying chunk wording |
| `cross_doc` | Prefer scholarship / secondary literature over primary Mao essays |
| `hard_negative` | Wording that can pull sibling docs; gold is the intended chunk |

Guarded by `tests/retrieval_hard_eval_test.rs`: easy auto-gold stays lexically saturated; hard gold must fail a title-stripped 8-gram containment Recall@5 gate (&lt; 0.35) plus per-query LCS&lt;8.
