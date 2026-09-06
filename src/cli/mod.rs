use crate::vector::embedder::{COHERE_CHAT_MODEL, COHERE_COMPAT_BASE_URL, COHERE_EMBED_MODEL};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Shared embedder selection for ingest / search / stats / ask.
#[derive(Args, Debug, Clone)]
pub struct EmbedderArgs {
    /// Use deterministic offline embedder instead of ONNX / remote API
    #[arg(long)]
    pub offline: bool,

    /// OpenAI-compatible embeddings base URL (SiliconFlow: https://api.siliconflow.cn/v1,
    /// Cohere: https://api.cohere.ai/compatibility/v1)
    #[arg(long, env = "EMBED_BASE_URL")]
    pub embed_base_url: Option<String>,

    /// Embeddings API key. SiliconFlow provider chain: `--embed-api-key` →
    /// `SILICONFLOW_API_KEY` → `EMBED_API_KEY` → `config.toml [siliconflow].api_key`.
    /// Cohere path additionally honors `COHERE_API_KEY` / `[cohere].api_key`.
    /// Never commit this value.
    #[arg(long, env = "EMBED_API_KEY")]
    pub embed_api_key: Option<String>,

    /// Remote embeddings model name (SiliconFlow BAAI/bge-m3 is 1024-dim;
    /// Cohere embed-v4.0 is 1536-dim)
    #[arg(long, default_value = COHERE_EMBED_MODEL)]
    pub embed_model: String,

    /// Embedding dimension. Omitted: 512 with `--offline`; 1024 for SiliconFlow;
    /// 768 for Gemini; 1536 for Cohere.
    #[arg(long)]
    pub embed_dim: Option<usize>,

    /// Embedding provider: siliconflow, gemini, cohere, openai, or local
    #[arg(long, env = "EMBED_PROVIDER")]
    pub embed_provider: Option<String>,

    /// Google Gemini API key (`GEMINI_API_KEY` or `config.toml` `[gemini].api_key`). Never commit this value.
    #[arg(long, env = "GEMINI_API_KEY")]
    pub gemini_api_key: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "mao_agent")]
#[command(author = "mao_agent developers")]
#[command(version = "0.1.0")]
#[command(
    about = "High-performance Vector Database & Knowledge Retrieval Engine for Historical Literature"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose logging output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

/// Ingest a corpus directory of Markdown documents into the vector database
#[derive(Args, Debug, Clone)]
pub struct IngestArgs {
    /// Path to corpus directory containing .md files
    #[arg(short, long, default_value = "corpus")]
    pub corpus_dir: PathBuf,

    /// Output index snapshot file path
    #[arg(short, long, default_value = "data/vector_store.bin")]
    pub index_file: PathBuf,

    /// Path to Tantivy full-text index directory
    #[arg(long, default_value = "data/tantivy_index")]
    pub tantivy_dir: PathBuf,

    /// Batch size for embedding and indexing. SiliconFlow bulk ingest: 16~32
    /// (default 32); smaller batches reduce HTTP 429 rate-limit pressure.
    #[arg(short, long, default_value_t = 32)]
    pub batch_size: usize,

    /// Max characters per chunk
    #[arg(long, default_value_t = 600)]
    pub max_chars: usize,

    #[command(flatten)]
    pub embedder: EmbedderArgs,
}

/// Search the corpus using Hybrid (Vector + BM25), Vector-only, or BM25-only retrieval
#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    /// Semantic or keyword search query text
    pub query: String,

    /// Path to vector index snapshot file
    #[arg(short, long, default_value = "data/vector_store.bin")]
    pub index_file: PathBuf,

    /// Path to Tantivy full-text index directory
    #[arg(long, default_value = "data/tantivy_index")]
    pub tantivy_dir: PathBuf,

    /// Retrieval mode: hybrid (BM25 + Vector RRF), vector, or bm25
    #[arg(long, default_value = "hybrid")]
    pub mode: String,

    /// Top-K results to retrieve
    #[arg(short = 'k', long, default_value_t = 5)]
    pub top_k: usize,

    /// Filter by historical period (e.g. "抗日", "土地革命", "解放战争")
    #[arg(short, long)]
    pub period: Option<String>,

    /// Filter by volume (e.g. "第一卷", "第二卷")
    #[arg(long)]
    pub volume: Option<String>,

    /// Filter by category (e.g. "军事", "哲学", "党建")
    #[arg(long)]
    pub category: Option<String>,

    /// Minimum score threshold
    #[arg(long, default_value_t = 0.0)]
    pub min_score: f32,

    /// Disable Cohere rerank (keep fused RRF order truncated to top_k)
    #[arg(long)]
    pub no_rerank: bool,

    /// Cohere rerank model (default: rerank-v3.5)
    #[arg(long, env = "COHERE_RERANK_MODEL")]
    pub rerank_model: Option<String>,

    /// Optional knowledge-graph snapshot (`ingest-graph` output). Missing file is a no-op.
    #[arg(long, default_value = "data/graph_store.bin")]
    pub graph_file: PathBuf,

    #[command(flatten)]
    pub embedder: EmbedderArgs,
}

/// Display statistics and health metrics of the vector store
#[derive(Args, Debug, Clone)]
pub struct StatsArgs {
    /// Path to index snapshot file
    #[arg(short, long, default_value = "data/vector_store.bin")]
    pub index_file: PathBuf,

    #[command(flatten)]
    pub embedder: EmbedderArgs,
}

/// Initialize sample historical corpus documents for testing
#[derive(Args, Debug, Clone)]
pub struct InitSamplesArgs {
    /// Target directory to create sample files
    #[arg(short, long, default_value = "corpus")]
    pub target_dir: PathBuf,
}

/// Ask the Dialectical Reasoning Agent (Mao Agent) with historical literature grounding
#[derive(Args, Debug, Clone)]
pub struct AskArgs {
    /// Question or consultation topic
    pub question: String,

    /// Path to index snapshot file
    #[arg(short, long, default_value = "data/vector_store.bin")]
    pub index_file: PathBuf,

    /// Path to Tantivy full-text index directory
    #[arg(long, default_value = "data/tantivy_index")]
    pub tantivy_dir: PathBuf,

    /// Top-K context chunks to retrieve
    #[arg(short = 'k', long, default_value_t = 3)]
    pub top_k: usize,

    /// Filter by historical period
    #[arg(short, long)]
    pub period: Option<String>,

    /// LLM API base URL (OpenAI compatible). Cohere: https://api.cohere.ai/compatibility/v1
    #[arg(long, env = "COHERE_BASE_URL", default_value = COHERE_COMPAT_BASE_URL)]
    pub base_url: String,

    /// LLM API key (`COHERE_API_KEY`, `--api-key`, or `config.toml` `[cohere].api_key`). Never commit this value.
    #[arg(long, env = "COHERE_API_KEY")]
    pub api_key: Option<String>,

    /// Chat model id (Cohere free-tier default: command-r7b-12-2024)
    #[arg(long, default_value = COHERE_CHAT_MODEL)]
    pub model: String,

    /// Disable Cohere rerank after hybrid fusion
    #[arg(long)]
    pub no_rerank: bool,

    /// Cohere rerank model (default: rerank-v3.5)
    #[arg(long, env = "COHERE_RERANK_MODEL")]
    pub rerank_model: Option<String>,

    /// Optional knowledge-graph snapshot. Missing file is a no-op.
    #[arg(long, default_value = "data/graph_store.bin")]
    pub graph_file: PathBuf,

    #[command(flatten)]
    pub embedder: EmbedderArgs,
}

/// Serve the Mao Agent as a high-performance HTTP API (Axum + Tokio)
#[derive(Args, Debug, Clone)]
pub struct ServeArgs {
    /// Bind address, e.g. 127.0.0.1:3000 or 0.0.0.0:8080
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    pub bind: String,

    /// Path to vector index snapshot file
    #[arg(short, long, default_value = "data/vector_store.bin")]
    pub index_file: PathBuf,

    /// Path to Tantivy full-text index directory
    #[arg(long, default_value = "data/tantivy_index")]
    pub tantivy_dir: PathBuf,

    /// Optional knowledge-graph snapshot. Missing file is a no-op.
    #[arg(long, default_value = "data/graph_store.bin")]
    pub graph_file: PathBuf,

    /// LLM API base URL (OpenAI compatible)
    #[arg(long, env = "COHERE_BASE_URL", default_value = COHERE_COMPAT_BASE_URL)]
    pub base_url: String,

    /// LLM API key (`COHERE_API_KEY` / `config.toml` `[cohere].api_key`)
    #[arg(long, env = "COHERE_API_KEY")]
    pub api_key: Option<String>,

    /// Chat model id
    #[arg(long, default_value = COHERE_CHAT_MODEL)]
    pub model: String,

    /// Disable Cohere rerank for hybrid search endpoints
    #[arg(long)]
    pub no_rerank: bool,

    /// Cohere rerank model (default: rerank-v3.5)
    #[arg(long, env = "COHERE_RERANK_MODEL")]
    pub rerank_model: Option<String>,

    /// Comma-separated CORS origin allowlist (default: localhost ports 3000/5173/8080).
    /// Env: `MAO_CORS_ORIGINS`. Overrides `[server].cors_origins` in config.toml.
    #[arg(long, env = "MAO_CORS_ORIGINS")]
    pub cors_origins: Option<String>,

    /// Shared API bearer token. When set, protected routes require `Authorization: Bearer …`.
    /// Env: `MAO_API_TOKEN`. Overrides `[server].api_token` in config.toml.
    /// Loopback without a token stays open for local dev (ADR 0005).
    #[arg(long, env = "MAO_API_TOKEN")]
    pub api_token: Option<String>,

    /// Max concurrent `/api/v1/ask` + `/ask/stream` requests (default 32). Env: `MAO_MAX_CONCURRENT_ASKS`.
    #[arg(long, env = "MAO_MAX_CONCURRENT_ASKS", default_value_t = 32)]
    pub max_concurrent_asks: usize,

    #[command(flatten)]
    pub embedder: EmbedderArgs,
}

/// Offline retrieval evaluation over gold queries (Recall/MRR/NDCG@k)
#[derive(Args, Debug, Clone)]
pub struct EvalRetrievalArgs {
    /// Path to gold queries NDJSON file
    #[arg(long, default_value = "evals/retrieval/queries.jsonl")]
    pub queries_file: PathBuf,

    /// Cutoff k for Recall/MRR/NDCG@k
    #[arg(long, default_value_t = 5)]
    pub k: usize,

    /// Retrieval mode: hybrid, vector, or bm25
    #[arg(long, default_value = "hybrid")]
    pub mode: String,

    /// Disable Cohere rerank (baseline offline default)
    #[arg(long)]
    pub no_rerank: bool,

    /// Force brute-force vector scan (disable HNSW ANN for recall comparison)
    #[arg(long)]
    pub force_brute: bool,

    /// Emit per-query NDJSON + summary object instead of a table
    #[arg(long)]
    pub json: bool,

    #[command(flatten)]
    pub embedder: EmbedderArgs,

    /// Path to vector index snapshot file
    #[arg(short, long, default_value = "data/vector_store.bin")]
    pub index_file: PathBuf,

    /// Path to Tantivy full-text index directory
    #[arg(long, default_value = "data/tantivy_index")]
    pub tantivy_dir: PathBuf,
}

/// Compile JSON graph (`build_knowledge_graph.py`) into a bincode snapshot.
#[derive(Args, Debug, Clone)]
pub struct IngestGraphArgs {
    /// JSON graph from the extractor
    #[arg(short, long, default_value = "data/graph_store.json")]
    pub input: PathBuf,

    /// Bincode snapshot written atomically
    #[arg(short, long, default_value = "data/graph_store.bin")]
    pub output: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Ingest a corpus directory of Markdown documents into the vector database
    Ingest(IngestArgs),

    /// Compile a JSON knowledge graph into a bincode snapshot
    IngestGraph(IngestGraphArgs),

    /// Search the corpus using Hybrid (Vector + BM25), Vector-only, or BM25-only retrieval
    Search(SearchArgs),

    /// Display statistics and health metrics of the vector store
    Stats(StatsArgs),

    /// Initialize sample historical corpus documents for testing
    InitSamples(InitSamplesArgs),

    /// Ask the Dialectical Reasoning Agent (Mao Agent) with historical literature grounding
    Ask(AskArgs),

    /// Serve the Mao Agent as a high-performance HTTP API (Axum + Tokio)
    Serve(ServeArgs),

    /// Evaluate retrieval quality (Recall/MRR/NDCG@k) against gold queries
    EvalRetrieval(EvalRetrievalArgs),

    /// Run as a Model Context Protocol (MCP) server over standard I/O (stdio)
    Mcp(McpArgs),
}

/// Run as a Model Context Protocol (MCP) server over standard I/O (stdio)
#[derive(Args, Debug, Clone)]
pub struct McpArgs {
    /// Path to vector index snapshot file
    #[arg(short, long, default_value = "data/vector_store.bin")]
    pub index_file: PathBuf,

    /// Path to Tantivy full-text index directory
    #[arg(long, default_value = "data/tantivy_index")]
    pub tantivy_dir: PathBuf,

    /// Optional knowledge-graph snapshot. Missing file is a no-op.
    #[arg(long, default_value = "data/graph_store.bin")]
    pub graph_file: PathBuf,

    /// LLM API base URL (OpenAI compatible)
    #[arg(long, env = "COHERE_BASE_URL", default_value = COHERE_COMPAT_BASE_URL)]
    pub base_url: String,

    /// LLM API key (`COHERE_API_KEY` / `config.toml` `[cohere].api_key`)
    #[arg(long, env = "COHERE_API_KEY")]
    pub api_key: Option<String>,

    /// Chat model id
    #[arg(long, default_value = COHERE_CHAT_MODEL)]
    pub model: String,

    /// Disable Cohere rerank for hybrid search
    #[arg(long)]
    pub no_rerank: bool,

    /// Cohere rerank model (default: rerank-v3.5)
    #[arg(long, env = "COHERE_RERANK_MODEL")]
    pub rerank_model: Option<String>,

    #[command(flatten)]
    pub embedder: EmbedderArgs,
}
