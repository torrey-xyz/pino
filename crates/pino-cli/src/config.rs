//! Configuration management for Pino CLI

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PinoConfig {
    pub version: String,
    pub build: BuildConfig,
    pub deploy: DeployConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    pub target: String,
    pub optimization: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployConfig {
    pub cluster: String,
    pub program_id: Option<String>,
}

impl Default for PinoConfig {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            build: BuildConfig {
                target: "bpf-unknown-unknown".to_string(),
                optimization: "release".to_string(),
            },
            deploy: DeployConfig {
                cluster: "devnet".to_string(),
                program_id: None,
            },
        }
    }
}

impl PinoConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: PinoConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
} 