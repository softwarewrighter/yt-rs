//! Request building for Ollama API.

use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Serialize;

use crate::config::OllamaConfig;

/// Request body for Ollama generate API.
#[derive(Serialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub images: Vec<String>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

impl GenerateRequest {
    /// Creates a new generate request from config, prompt, and image data.
    pub fn new(config: &OllamaConfig, prompt: &str, image_data: &[u8]) -> Self {
        Self {
            model: config.model.clone(),
            prompt: prompt.to_string(),
            images: vec![STANDARD.encode(image_data)],
            stream: false,
            system: config.system_prompt.clone(),
        }
    }
}
