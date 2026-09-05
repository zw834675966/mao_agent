use crate::error::{Result, VectorError};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Project config loaded from `config.toml` (cwd) or `MAO_AGENT_CONFIG`.
#[derive(Debug, Default, Deserialize)]
pub struct ProjectConfig {
    #[serde(default)]
    pub cohere: CohereConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct CohereConfig {
    pub api_key: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ServerConfig {
    /// Explicit CORS origin allowlist (http://host:port). Empty/omitted → localhost defaults.
    pub cors_origins: Option<Vec<String>>,
}

impl ProjectConfig {
    pub fn parse(toml_text: &str) -> Result<Self> {
        toml::from_str(toml_text).map_err(|e| VectorError::ConfigError(e.to_string()))
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| VectorError::ConfigError(format!("{}: {e}", path.display())))?;
        Self::parse(&text)
    }

    /// Missing file → `None`. Parse/IO errors are logged and treated as unset.
    pub fn try_load_default() -> Option<Self> {
        let path = discover_config_path()?;
        match Self::load_from_path(&path) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                tracing::warn!("Failed to load {}: {e}", path.display());
                None
            }
        }
    }

    pub fn cohere_api_key(&self) -> Option<&str> {
        nonempty_key(self.cohere.api_key.as_deref())
    }

    pub fn cors_origins(&self) -> Option<&[String]> {
        self.server.cors_origins.as_deref()
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
