//! Configuration loading for the yt-rs CLI.

use serde::Deserialize;
use std::path::PathBuf;

/// Application configuration loaded from TOML file.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AppConfig {
    /// Configuration for the GenerateDialog node.
    #[serde(default)]
    pub generate_dialog: GenerateDialogConfig,

    /// Configuration for workspace management.
    #[serde(default)]
    pub workspace: WorkspaceConfig,
}

/// Configuration for the GenerateDialog node's Ollama integration.
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateDialogConfig {
    /// Ollama server hostname (default: "localhost").
    #[serde(default = "default_ollama_host")]
    pub ollama_host: String,

    /// Ollama server port (default: 11434).
    #[serde(default = "default_ollama_port")]
    pub ollama_port: u16,

    /// Vision model name (default: "llama3.2-vision:11b").
    #[serde(default = "default_vision_model")]
    pub vision_model: String,

    /// Optional custom system prompt for the vision model.
    pub system_prompt: Option<String>,
}

/// Configuration for workspace management.
#[derive(Debug, Clone, Deserialize)]
pub struct WorkspaceConfig {
    /// Default project name for save/restore (default: "default").
    #[serde(default = "default_project_name")]
    pub default_project_name: String,
}

fn default_ollama_host() -> String {
    "localhost".to_string()
}

fn default_ollama_port() -> u16 {
    11434
}

fn default_vision_model() -> String {
    "llama3.2-vision:11b".to_string()
}

fn default_project_name() -> String {
    "default".to_string()
}

impl Default for GenerateDialogConfig {
    fn default() -> Self {
        Self {
            ollama_host: default_ollama_host(),
            ollama_port: default_ollama_port(),
            vision_model: default_vision_model(),
            system_prompt: None,
        }
    }
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            default_project_name: default_project_name(),
        }
    }
}

impl AppConfig {
    /// Load configuration from a TOML file.
    ///
    /// Returns default configuration if the file doesn't exist.
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
