//! Configuration for Ollama client.

/// Configuration for the Ollama client.
#[derive(Debug, Clone)]
pub struct OllamaConfig {
    /// Hostname of the Ollama server.
    pub host: String,
    /// Port of the Ollama server.
    pub port: u16,
    /// Vision model to use.
    pub model: String,
    /// Optional system prompt.
    pub system_prompt: Option<String>,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 11434,
            model: "llama3.2-vision:11b".to_string(),
            system_prompt: None,
        }
    }
}

impl OllamaConfig {
    /// Returns the base URL for the Ollama API.
    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
