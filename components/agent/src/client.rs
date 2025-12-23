//! Ollama client - thin adapter over lower-level modules.

use crate::config::OllamaConfig;
use crate::error::OllamaError;
use crate::request::GenerateRequest;
use crate::transport;

/// Client for communicating with Ollama vision models.
pub struct OllamaClient {
    config: OllamaConfig,
    http: reqwest::Client,
}

impl OllamaClient {
    /// Creates a new Ollama client with the given configuration.
    pub fn new(config: OllamaConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    /// Returns the base URL for the Ollama API.
    pub fn base_url(&self) -> String {
        self.config.base_url()
    }

    /// Returns the model name.
    pub fn model(&self) -> &str {
        &self.config.model
    }

    /// Returns the system prompt if configured.
    pub fn system_prompt(&self) -> Option<&str> {
        self.config.system_prompt.as_deref()
    }

    /// Analyzes an image with the given prompt.
    pub async fn analyze_image(
        &self,
        image_data: &[u8],
        prompt: &str,
    ) -> Result<String, OllamaError> {
        let request = GenerateRequest::new(&self.config, prompt, image_data);
        let url = format!("{}/api/generate", self.base_url());
        transport::send_generate(&self.http, &url, &request).await
    }

    /// Analyzes an image file with the given prompt.
    pub async fn analyze_image_file(
        &self,
        path: &std::path::Path,
        prompt: &str,
    ) -> Result<String, OllamaError> {
        let data = tokio::fs::read(path)
            .await
            .map_err(OllamaError::ImageRead)?;
        self.analyze_image(&data, prompt).await
    }
}
