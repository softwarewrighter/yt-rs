//! Ollama vision model client.

use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::config::GenerateDialogConfig;

/// Client for interacting with Ollama's vision model API.
pub struct OllamaClient {
    client: Client,
    base_url: String,
    model: String,
    system_prompt: Option<String>,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    images: Vec<String>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaClient {
    /// Creates a new Ollama client from configuration.
    pub fn new(config: &GenerateDialogConfig) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://{}:{}", config.ollama_host, config.ollama_port),
            model: config.vision_model.clone(),
            system_prompt: config.system_prompt.clone(),
        }
    }

    /// Analyzes an image using the vision model.
    pub async fn analyze_image(&self, image_data: &[u8], prompt: &str) -> Result<String, String> {
        let has_image = !image_data.is_empty();
        info!(
            "Ollama request: model={}, image_size={} bytes, has_image={}",
            self.model,
            image_data.len(),
            has_image
        );
        debug!("System prompt: {:?}", self.system_prompt);
        debug!("User prompt: {}", prompt);

        let request = self.build_request(prompt, image_data);
        let url = format!("{}/api/generate", self.base_url);
        info!("Sending request to {}", url);

        let body = self.send_request(&url, &request).await?;
        self.parse_response(&body)
    }

    fn build_request(&self, prompt: &str, image_data: &[u8]) -> OllamaRequest {
        let images = if image_data.is_empty() {
            vec![]
        } else {
            vec![base64::engine::general_purpose::STANDARD.encode(image_data)]
        };

        OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            images,
            stream: false,
            system: self.system_prompt.clone(),
        }
    }

    async fn send_request(&self, url: &str, request: &OllamaRequest) -> Result<String, String> {
        let response = self
            .client
            .post(url)
            .json(request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to connect to Ollama: {}", e);
                format!("Failed to connect to Ollama: {}", e)
            })?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!("Ollama error {}: {}", status, body);
            return Err(format!("Ollama returned error {}: {}", status, body));
        }

        response.text().await.map_err(|e| {
            error!("Failed to read response body: {}", e);
            format!("Failed to read response body: {}", e)
        })
    }

    fn parse_response(&self, body: &str) -> Result<String, String> {
        debug!("Ollama raw response: {}", body);

        let result: OllamaResponse = serde_json::from_str(body).map_err(|e| {
            error!("Failed to parse Ollama response: {} - body: {}", e, body);
            format!("Failed to parse Ollama response: {}", e)
        })?;

        info!("Ollama response: {} chars", result.response.len());
        debug!("Response text: {}", result.response);

        Ok(result.response)
    }
}
