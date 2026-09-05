use serde::{Deserialize, Serialize};

use crate::agent::VerificationReport;
use crate::model::{DocumentChunk, VectorStoreStats};

// ── Search ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    /// 检索查询文本，支持语义/关键词混合
    pub query: String,
    /// 返回条数，默认 5，最大 20
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    /// 检索模式: hybrid (默认) | vector | bm25
    #[serde(default = "default_mode")]
    pub mode: String,
    /// 历史时期过滤，如 "抗日" / "土地革命" / 完整 "抗日战争时期 (1937-1945)"
    pub period: Option<String>,
    /// 卷册过滤，如 "第一卷" / "第二卷"
    pub volume: Option<String>,
    /// 分类过滤，如 "军事" / "哲学" / "党建"
    pub category: Option<String>,
    /// 标签过滤（任一命中即通过）
    pub tags: Option<Vec<String>>,
    /// 起始日期 YYYY / YYYY-MM / YYYY-MM-DD
    pub start_date: Option<String>,
    /// 结束日期
    pub end_date: Option<String>,
    /// 文档 ID 精确过滤
    pub doc_id: Option<String>,
    /// 关键词全文过滤（必须出现在文本中）
    pub keyword: Option<String>,
    /// 最小相似度阈值，低于此分数的结果被过滤（仅 vector/hybrid 有效）
    #[serde(default)]
    pub min_score: Option<f32>,
}

fn default_top_k() -> usize {
    5
}
fn default_mode() -> String {
    "hybrid".to_string()
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub mode: String,
    pub elapsed_ms: u64,
    pub total: usize,
    pub results: Vec<SearchHit>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SearchHit {
    pub chunk_id: String,
    pub rank: usize,
    /// RRF 融合分（仅 hybrid 有值）
    pub rrf_score: Option<f32>,
    /// 向量余弦相似度
    pub vector_score: Option<f32>,
    /// BM25 分数
    pub bm25_score: Option<f32>,
    pub chunk: DocumentChunk,
}

// ── Ask ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    /// 现实问题/困惑描述
    pub question: String,
    /// 召回上下文块数，默认 3，最大 10
    #[serde(default = "default_ask_top_k")]
    pub top_k: Option<usize>,
    /// 历史时期过滤
    pub period: Option<String>,
    /// 卷册过滤
    pub volume: Option<String>,
    /// LLM 覆盖：OpenAI 兼容 base_url（可选，默认 COHERE_COMPAT_BASE_URL）
    pub base_url: Option<String>,
    /// LLM 覆盖：模型名（可选，默认 command-r7b-12-2024）
    pub model: Option<String>,
    /// 每次请求覆盖的 API Key（可选，优先于环境变量/config.toml；建议用 Header Authorization 替代）
    pub api_key: Option<String>,
}

fn default_ask_top_k() -> Option<usize> {
    Some(3)
}

#[derive(Debug, Serialize)]
pub struct AskResponse {
    pub question: String,
    pub content: String,
    pub retrieved_chunks: Vec<DocumentChunk>,
    pub citation_reports: Vec<VerificationReport>,
    pub is_fully_grounded: bool,
    pub elapsed_ms: u64,
}

// SSE 事件负载

#[derive(Debug, Serialize)]
pub struct SseRetrievedEvent {
    pub chunks: Vec<DocumentChunk>,
}

#[derive(Debug, Serialize)]
pub struct SseDeltaEvent {
    pub delta: String,
    /// 当前阶段标识: investigation | contradiction | synthesis | policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SseCitationEvent {
    pub reports: Vec<VerificationReport>,
    pub is_fully_grounded: bool,
}

#[derive(Debug, Serialize)]
pub struct SseDoneEvent {
    pub is_fully_grounded: bool,
    pub elapsed_ms: u64,
}

// ── Verify ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    /// 待核验引文
    pub quote: String,
    /// 声称出处篇名，如 "论持久战"
    pub claimed_title: String,
    /// 用于比对的上下文块；若为空则从全文索引中按 title 检索兜底
    #[serde(default)]
    pub context_chunks: Vec<DocumentChunk>,
    /// 最低置信度阈值，默认 0.85
    pub min_confidence: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub report: VerificationReport,
}

// ── Health / Meta ────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub index_loaded: bool,
    pub tantivy_loaded: bool,
    pub total_vectors: usize,
    pub total_documents: usize,
    pub vector_dimension: usize,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub stats: VectorStoreStats,
    pub tantivy_loaded: bool,
}
