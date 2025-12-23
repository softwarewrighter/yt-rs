//! Response parsing for Ollama API.

use serde::Deserialize;

/// Response from Ollama generate API.
#[derive(Deserialize)]
pub struct GenerateResponse {
    pub response: String,
}
