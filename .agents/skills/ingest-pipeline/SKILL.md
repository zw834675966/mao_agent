---
name: ingest-pipeline
description: Builds or rebuilds corpus sample files, dense vector store snapshots, and Tantivy full-text index artifacts using the offline deterministic pipeline. Use when setting up a fresh environment, modifying markdown documents in corpus/, or rebuilding search indexes after chunking or schema updates.
---

# Ingestion Pipeline Skill

Guides the end-to-end process of generating sample corpus documents, processing markdown texts into semantic chunks, building dense vector store snapshots (`data/vector_store.bin`), and compiling the Tantivy BM25 inverted index (`data/tantivy_index/`).

## Crucial Caveats & Rules

> [!CAUTION]
> Artifacts under `data/` (`data/vector_store.bin` and `data/tantivy_index/`) are committed binary/index outputs. **Never manually edit, patch, or corrupt these files directly.** Always regenerate them via this pipeline.

---

## Step-by-Step Execution

Run the steps in order from the repository root (`D:/rust/mao_agent`):

### 1. Initialize or Verify Sample Corpus
Generates or verifies standard Chinese markdown documents with YAML frontmatter in `corpus/*.md`:

```bash
cargo run --no-default-features -- init-samples
```

- **Verification**: Confirm `corpus/` contains 15 standard Markdown files covering both primary canonical works (《论持久战》、《矛盾论》、《实践论》、《反对本本主义》、《中国社会各阶级的分析》、《关于正确处理人民内部矛盾的问题》等) and scholarship collections (北大/清华/人大名家与海外权威研究).

### 2. Run Ingestion Pipeline
Parses frontmatter, applies CJK cleaning and semantic chunking, computes embeddings, indexes chunks into Tantivy BM25, and persists both vector store and search index to disk:

```bash
cargo run --no-default-features -- ingest --corpus-dir corpus --batch-size 32
```

- **Arguments**:
  - `--corpus-dir corpus`: Directory containing markdown files.
  - `--batch-size 32`: Batch size for chunk embedding calculation.
  - `--no-default-features`: Runs without compiling or downloading the heavy local FastEmbed ONNX model.
  - `--offline`: Optional flag to force offline deterministic embeddings instead of remote API.

### 3. Validate Database & Index Integrity
Verifies vector store chunk counts, vector dimensions, and Tantivy document counts:

```bash
cargo run --no-default-features -- stats
```

- **Expected Output**:
  - Reports 15 indexed documents and 59 total chunks.
  - Covers all historical periods (大革命时期、土地革命时期、抗战时期、解放战争时期、社会主义建设时期及当代学术研究).
  - Storage paths pointing to `data/vector_store.bin` and `data/tantivy_index/`.

### 4. Verify Artifact Existence and Consistency
Ensure output files exist on the filesystem:

- `data/vector_store.bin`: Bincode snapshot of the vector store (non-empty binary file).
- `data/tantivy_index/`: Tantivy directory containing index files (e.g., `meta.json` and segment files).

Both stores must represent the exact same chunk corpus.

---

## Validation Checklist

- [ ] `cargo run --no-default-features -- init-samples` completed with exit code `0`.
- [ ] `cargo run --no-default-features -- ingest --corpus-dir corpus --batch-size 32` completed with exit code `0`.
- [ ] `cargo run --no-default-features -- stats` confirms chunks > 0 and displays healthy index status.
- [ ] `data/vector_store.bin` exists and is non-empty.
- [ ] `data/tantivy_index/` exists and contains Tantivy metadata (`meta.json`).
- [ ] No manual edits were applied to `data/`.
