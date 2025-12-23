//! Error types for Ollama client.

use thiserror::Error;

/// Errors from the Ollama client.
#[derive(Debug, Error)]
pub enum OllamaError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
    #[error("Failed to read image: {0}")]
    ImageRead(std::io::Error),
}
