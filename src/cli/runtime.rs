//! CLI runtime helpers: config key resolution, embedder/reranker construction, store loading.
use super::EmbedderArgs;
use crate::config::{ProjectConfig, nonempty_key};
use crate::model::{HistoricalPeriod, VectorFilter};
use crate::vector::embedder::{Embedder, EmbedderSelection, resolve_embedder};
use crate::vector::store::VectorStore;
use std::path::Path;
use std::sync::Arc;

pub fn config_cohere_api_key() -> Option<String> {
    ProjectConfig::try_load_default()?
        .cohere_api_key()
        .map(str::to_string)
}

fn config_gemini_api_key() -> Option<String> {
    ProjectConfig::try_load_default()?
        .gemini_api_key()
        .map(str::to_string)
}

fn config_gemini_model() -> Option<String> {
    ProjectConfig::try_load_default()?
        .gemini_model()
        .map(str::to_string)
}

fn config_gemini_dimension() -> Option<usize> {
    ProjectConfig::try_load_default()?.gemini_dimension()
}

fn config_siliconflow_api_key() -> Option<String> {
    ProjectConfig::try_load_default()?
        .siliconflow_api_key()
        .map(str::to_string)
}

fn config_siliconflow_base_url() -> Option<String> {
    ProjectConfig::try_load_default()?
        .siliconflow_base_url()
        .map(str::to_string)
}

fn config_siliconflow_model() -> Option<String> {
    ProjectConfig::try_load_default()?
        .siliconflow_model()
        .map(str::to_string)
}

fn config_siliconflow_dimension() -> Option<usize> {
    ProjectConfig::try_load_default()?.siliconflow_dimension()
}

pub fn resolve_gemini_api_key(args: &EmbedderArgs) -> Option<String> {
    if let Some(key) = nonempty_key(args.gemini_api_key.as_deref()) {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("GEMINI_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    config_gemini_api_key()
}

pub fn resolve_embed_api_key(args: &EmbedderArgs) -> Option<String> {
    if let Some(key) = nonempty_key(args.embed_api_key.as_deref()) {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("COHERE_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    config_cohere_api_key()
}

/// SiliconFlow key chain for `--embed-provider siliconflow`.
/// Deliberately excludes `COHERE_API_KEY` / `[cohere].api_key` so a stale
/// Cohere credential can never shadow the production embedding backend.
pub fn resolve_siliconflow_api_key(args: &EmbedderArgs) -> Option<String> {
    if let Some(key) = nonempty_key(args.embed_api_key.as_deref()) {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("SILICONFLOW_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("EMBED_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    config_siliconflow_api_key()
}

pub fn resolve_chat_api_key(cli_key: Option<String>, offline: bool) -> Option<String> {
    if offline {
        return None;
    }
    if let Some(key) = nonempty_key(cli_key.as_deref()) {
        return Some(key.to_string());
    }
    config_cohere_api_key()
}

pub fn resolve_rerank_api_key(
    cli_key: Option<&str>,
    offline: bool,
    no_rerank: bool,
) -> Option<String> {
    if offline || no_rerank {
        return None;
    }
    if let Some(key) = nonempty_key(cli_key) {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("COHERE_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("EMBED_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    config_cohere_api_key()
}

pub fn make_reranker(
    offline: bool,
    no_rerank: bool,
    rerank_model: Option<String>,
    api_key_hint: Option<&str>,
) -> Option<std::sync::Arc<dyn crate::Reranker>> {
    let key = resolve_rerank_api_key(api_key_hint, offline, no_rerank)?;
    Some(std::sync::Arc::new(crate::CohereReranker::new(
        key,
        rerank_model,
        None,
    )))
}

pub fn get_embedder(
    args: &EmbedderArgs,
    cache_path: Option<&Path>,
) -> Result<Arc<dyn Embedder>, Box<dyn std::error::Error>> {
    let gemini_key = resolve_gemini_api_key(args);
    let siliconflow_key = resolve_siliconflow_api_key(args);
    // Explicit `--embed-provider` wins; otherwise auto-select by configured key
    // (SiliconFlow first as the production backend, then Gemini).
    let provider = args.embed_provider.as_deref().or_else(|| {
        crate::vector::preferred_embed_provider(siliconflow_key.is_some(), gemini_key.is_some())
    });

    // Offline never inherits a provider's config dimension: it is LOCAL_EMBEDDING_DIM
    // unless the user passed `--embed-dim`. Non-offline keeps the config fill-in.
    let explicit_dim = if args.offline {
        args.embed_dim
    } else {
        args.embed_dim.or_else(|| {
            if provider == Some("gemini") {
                config_gemini_dimension()
            } else if provider == Some("siliconflow") {
                config_siliconflow_dimension()
            } else {
                None
            }
        })
    };

    let model = if provider == Some("gemini") {
        if args.embed_model != crate::vector::COHERE_EMBED_MODEL {
            args.embed_model.clone()
        } else {
            config_gemini_model().unwrap_or_else(|| crate::vector::GEMINI_DEFAULT_MODEL.to_string())
        }
    } else if provider == Some("siliconflow") {
        if args.embed_model != crate::vector::COHERE_EMBED_MODEL {
            args.embed_model.clone()
        } else {
            config_siliconflow_model()
                .unwrap_or_else(|| crate::vector::SILICONFLOW_DEFAULT_MODEL.to_string())
        }
    } else {
        args.embed_model.clone()
    };

    // SiliconFlow must never fall through to the Cohere default base URL in
    // `resolve_embedder`: pin base URL explicitly (CLI > config > default).
    let base_url = if provider == Some("siliconflow") {
        Some(
            args.embed_base_url
                .clone()
                .filter(|s| !s.trim().is_empty())
                .or_else(config_siliconflow_base_url)
                .unwrap_or_else(|| crate::vector::SILICONFLOW_DEFAULT_BASE_URL.to_string()),
        )
    } else {
        args.embed_base_url.clone()
    };

    let api_key = if provider == Some("siliconflow") {
        siliconflow_key
    } else {
        resolve_embed_api_key(args)
    };

    let selection = EmbedderSelection {
        offline: args.offline,
        api_key,
        base_url,
        model,
        dimension: crate::vector::resolve_embed_dimension_with_provider(
            args.offline,
            provider,
            explicit_dim,
        ),
        provider: provider.map(str::to_string),
        gemini_api_key: gemini_key,
    };
    Ok(resolve_embedder(&selection, cache_path)?)
}

pub fn load_store_interactive(
    path: &Path,
    embedder: Arc<dyn Embedder>,
) -> Result<Option<VectorStore>, Box<dyn std::error::Error>> {
    match VectorStore::load_from_file(path, embedder) {
        Ok(s) => Ok(Some(s)),
        Err(crate::VectorError::IdentityMismatch {
            snapshot_model,
            snapshot_dimension,
            source_model,
            source_dimension,
        }) => {
            eprintln!(
                "❌ 向量模型不匹配：当前索引 snapshot 采用模型 `{}` ({} 维)，而检索请求配置为 `{}` ({} 维)。\n💡 提示：若需在离线模式下检索，请先运行 `cargo run -- ingest --offline` 重建离线索引；若需在线检索，请移除 `--offline` 参数。",
                snapshot_model, snapshot_dimension, source_model, source_dimension
            );
            Ok(None)
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn build_filter(
    period: Option<&str>,
    volume: Option<&str>,
    category: Option<&str>,
) -> Option<VectorFilter> {
    if period.is_none() && volume.is_none() && category.is_none() {
        return None;
    }
    let mut filter = VectorFilter::new();
    if let Some(p) = period {
        filter = filter.with_period(HistoricalPeriod::from_str_or_date(p));
    }
    if let Some(v) = volume {
        filter = filter.with_volume(v);
    }
    if let Some(c) = category {
        filter = filter.with_category(c);
    }
    Some(filter)
}
