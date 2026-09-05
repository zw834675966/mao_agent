use crate::vector::embedder::{COHERE_CHAT_MODEL, COHERE_COMPAT_BASE_URL, COHERE_EMBED_MODEL};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Shared embedder selection for ingest / search / stats / ask.
#[derive(Args, Debug, Clone)]
pub struct EmbedderArgs {
    /// Use deterministic offline embedder instead of ONNX / remote API
    #[arg(long)]
    pub offline: bool,

    /// OpenAI-compatible embeddings base URL (Cohere: https://api.cohere.ai/compatibility/v1)
    #[arg(long, env = "EMBED_BASE_URL")]
    pub embed_base_url: Option<String>,

    /// Embeddings API key (`EMBED_API_KEY`, `COHERE_API_KEY`, or `config.toml` `[cohere].api_key`). Never commit this value.
    #[arg(long, env = "EMBED_API_KEY")]
    pub embed_api_key: Option<String>,

    /// Remote embeddings model name (Cohere embed-v4.0 is multilingual, 1536-dim)
    #[arg(long, default_value = COHERE_EMBED_MODEL)]
    pub embed_model: String,

    /// Embedding dimension. Omitted: 512 with `--offline` (local BGE); 1536 for remote/Cohere.
    #[arg(long)]
    pub embed_dim: Option<usize>,
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

    /// Batch size for embedding and indexing
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

    /// LLM API base URL (OpenAI compatible)
    #[arg(long, env = "COHERE_BASE_URL", default_value = COHERE_COMPAT_BASE_URL)]
    pub base_url: String,

    /// LLM API key (`COHERE_API_KEY` / `config.toml` `[cohere].api_key`)
    #[arg(long, env = "COHERE_API_KEY")]
    pub api_key: Option<String>,

    /// Chat model id
    #[arg(long, default_value = COHERE_CHAT_MODEL)]
    pub model: String,

    #[command(flatten)]
    pub embedder: EmbedderArgs,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Ingest a corpus directory of Markdown documents into the vector database
    Ingest(IngestArgs),

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
}
