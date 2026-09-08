use crate::error::{Result, VectorError};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Project config loaded from `config.toml` (cwd) or `MAO_AGENT_CONFIG`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ProjectConfig {
    #[serde(default)]
    pub cohere: CohereConfig,
    #[serde(default)]
    pub gemini: GeminiConfig,
    #[serde(default)]
    pub siliconflow: SiliconFlowConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct CohereConfig {
    pub api_key: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct GeminiConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub dimension: Option<usize>,
}

/// SiliconFlow OpenAI-compatible embedding backend (production default).
/// BAAI/bge-m3 yields 1024-dim dense vectors.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct SiliconFlowConfig {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub dimension: Option<usize>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ServerConfig {
    /// Explicit CORS origin allowlist (http://host:port). Empty/omitted → localhost defaults.
    pub cors_origins: Option<Vec<String>>,
    /// Optional shared bearer token for API auth (see ADR 0005).
    pub api_token: Option<String>,
    /// Max concurrent ask/ask-stream handlers (default 32 when unset at CLI).
    pub max_concurrent_asks: Option<usize>,
}

static DEFAULT_CONFIG: OnceLock<Option<ProjectConfig>> = OnceLock::new();

impl ProjectConfig {
    pub fn parse(toml_text: &str) -> Result<Self> {
        toml::from_str(toml_text).map_err(|e| VectorError::ConfigError(e.to_string()))
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| VectorError::ConfigError(format!("{}: {e}", path.display())))?;
        Self::parse(&text)
    }

    /// Get a reference to the globally cached default config (parsed at most once per process).
    pub fn cached_default() -> Option<&'static ProjectConfig> {
        DEFAULT_CONFIG
            .get_or_init(Self::try_load_default_uncached)
            .as_ref()
    }

    fn try_load_default_uncached() -> Option<Self> {
        let path = discover_config_path()?;
        match Self::load_from_path(&path) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                tracing::warn!("Failed to load {}: {e}", path.display());
                None
            }
        }
    }

    /// Backward-compatible interface: returns a clone of the global cache, never re-reads disk.
    pub fn try_load_default() -> Option<Self> {
        Self::cached_default().cloned()
    }

    pub fn cohere_api_key(&self) -> Option<&str> {
        nonempty_key(self.cohere.api_key.as_deref())
    }

    pub fn gemini_api_key(&self) -> Option<&str> {
        nonempty_key(self.gemini.api_key.as_deref())
    }

    pub fn gemini_model(&self) -> Option<&str> {
        nonempty_key(self.gemini.model.as_deref())
    }

    pub fn gemini_dimension(&self) -> Option<usize> {
        self.gemini.dimension
    }

    pub fn siliconflow_api_key(&self) -> Option<&str> {
        nonempty_key(self.siliconflow.api_key.as_deref())
    }

    pub fn siliconflow_base_url(&self) -> Option<&str> {
        nonempty_key(self.siliconflow.base_url.as_deref())
    }

    pub fn siliconflow_model(&self) -> Option<&str> {
        nonempty_key(self.siliconflow.model.as_deref())
    }

    pub fn siliconflow_dimension(&self) -> Option<usize> {
        self.siliconflow.dimension
    }

    pub fn cors_origins(&self) -> Option<&[String]> {
        self.server.cors_origins.as_deref()
    }

    pub fn api_token(&self) -> Option<&str> {
        nonempty_key(self.server.api_token.as_deref())
    }

    pub fn max_concurrent_asks(&self) -> Option<usize> {
        self.server.max_concurrent_asks
    }
}

pub fn discover_config_path() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("MAO_AGENT_CONFIG") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Some(path);
        }
    }
    let cwd = PathBuf::from("config.toml");
    cwd.is_file().then_some(cwd)
}

pub fn nonempty_key(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cached_default_idempotence() {
        let first = ProjectConfig::cached_default();
        let second = ProjectConfig::cached_default();
        let third = ProjectConfig::cached_default();

        match (first, second, third) {
            (Some(a), Some(b), Some(c)) => {
                assert!(
                    std::ptr::eq(a, b),
                    "cached_default must return the same reference"
                );
                assert!(
                    std::ptr::eq(b, c),
                    "cached_default must return the same reference"
                );
                // try_load_default returns a clone with identical behavior.
                let cloned = ProjectConfig::try_load_default().expect("cache Some => clone Some");
                assert_eq!(cloned.cohere_api_key(), a.cohere_api_key());
                assert_eq!(cloned.gemini_api_key(), a.gemini_api_key());
                assert_eq!(cloned.siliconflow_api_key(), a.siliconflow_api_key());
                assert_eq!(cloned.api_token(), a.api_token());
                assert_eq!(cloned.max_concurrent_asks(), a.max_concurrent_asks());
            }
            (None, None, None) => {
                assert!(ProjectConfig::try_load_default().is_none());
            }
            _ => panic!("cached_default must be consistent across calls"),
        }
    }
}
