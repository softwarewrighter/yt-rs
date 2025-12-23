//! Tests for Ollama client.

use yt_rs_agent::{OllamaClient, OllamaConfig};

#[test]
fn test_ollama_config_default() {
    let config = OllamaConfig::default();
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 11434);
    assert_eq!(config.model, "llama3.2-vision:11b");
    assert!(config.system_prompt.is_none());
}

#[test]
fn test_ollama_client_new() {
    let config = OllamaConfig::default();
    let client = OllamaClient::new(config);
    assert!(client.base_url().starts_with("http://"));
}

#[test]
fn test_ollama_client_base_url_format() {
    let config = OllamaConfig {
        host: "testhost".to_string(),
        port: 12345,
        ..Default::default()
    };
    let client = OllamaClient::new(config);
    assert_eq!(client.base_url(), "http://testhost:12345");
}

#[test]
fn test_ollama_client_model_name() {
    let config = OllamaConfig {
        model: "custom-model".to_string(),
        ..Default::default()
    };
    let client = OllamaClient::new(config);
    assert_eq!(client.model(), "custom-model");
}

#[test]
fn test_ollama_client_system_prompt_none() {
    let config = OllamaConfig::default();
    let client = OllamaClient::new(config);
    assert!(client.system_prompt().is_none());
}

#[test]
fn test_ollama_client_system_prompt_some() {
    let config = OllamaConfig {
        system_prompt: Some("You are a helpful assistant.".to_string()),
        ..Default::default()
    };
    let client = OllamaClient::new(config);
    assert_eq!(client.system_prompt(), Some("You are a helpful assistant."));
}

#[test]
fn test_ollama_config_custom() {
    let config = OllamaConfig {
        host: "big72".to_string(),
        port: 11434,
        model: "llama3.2-vision:11b".to_string(),
        system_prompt: Some("Analyze this image.".to_string()),
    };
    assert_eq!(config.host, "big72");
    assert_eq!(config.model, "llama3.2-vision:11b");
}
