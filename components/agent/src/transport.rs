//! HTTP transport for Ollama API.

use crate::error::OllamaError;
use crate::request::GenerateRequest;
use crate::response::GenerateResponse;

/// Sends a generate request and returns the response text.
pub async fn send_generate(
    http: &reqwest::Client,
    url: &str,
    request: &GenerateRequest,
) -> Result<String, OllamaError> {
    let response = http.post(url).json(request).send().await?;
    parse_response(response).await
}

async fn parse_response(response: reqwest::Response) -> Result<String, OllamaError> {
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(OllamaError::Api(format!("{}: {}", status, text)));
    }
    let result: GenerateResponse = response.json().await?;
    Ok(result.response)
}
