//! Tests for configuration loading.

use std::io::Write;
use tempfile::NamedTempFile;
use yt_rs_cli::config::{AppConfig, GenerateDialogConfig, WorkspaceConfig};

#[test]
fn test_config_default_values() {
    let config = AppConfig::default();

    assert_eq!(config.generate_dialog.ollama_host, "localhost");
    assert_eq!(config.generate_dialog.ollama_port, 11434);
    assert_eq!(config.generate_dialog.vision_model, "llama3.2-vision:11b");
    assert!(config.generate_dialog.system_prompt.is_none());
    assert_eq!(config.workspace.default_project_name, "default");
}

#[test]
fn test_generate_dialog_config_default() {
    let config = GenerateDialogConfig::default();

    assert_eq!(config.ollama_host, "localhost");
    assert_eq!(config.ollama_port, 11434);
    assert_eq!(config.vision_model, "llama3.2-vision:11b");
    assert!(config.system_prompt.is_none());
}

#[test]
fn test_workspace_config_default() {
    let config = WorkspaceConfig::default();

    assert_eq!(config.default_project_name, "default");
}

#[test]
fn test_config_load_missing_file_returns_default() {
    let path = std::path::PathBuf::from("/nonexistent/path/config.toml");
    let config = AppConfig::load(&path).expect("Should return default for missing file");

    assert_eq!(config.generate_dialog.ollama_host, "localhost");
}

#[test]
fn test_config_load_valid_file() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(
        temp_file,
        r#"
[generate_dialog]
ollama_host = "big72"
ollama_port = 11434
vision_model = "llama3.2-vision:11b"

[workspace]
default_project_name = "my_project"
"#
    )
    .expect("Failed to write to temp file");

    let config = AppConfig::load(&temp_file.path().to_path_buf()).expect("Should load config");

    assert_eq!(config.generate_dialog.ollama_host, "big72");
    assert_eq!(config.workspace.default_project_name, "my_project");
}

#[test]
fn test_config_load_partial_file_uses_defaults() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(
        temp_file,
        r#"
[generate_dialog]
ollama_host = "custom_host"
"#
    )
    .expect("Failed to write to temp file");

    let config = AppConfig::load(&temp_file.path().to_path_buf()).expect("Should load config");

    assert_eq!(config.generate_dialog.ollama_host, "custom_host");
    assert_eq!(config.generate_dialog.ollama_port, 11434); // default
    assert_eq!(config.workspace.default_project_name, "default"); // default
}

#[test]
fn test_config_load_with_system_prompt() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(
        temp_file,
        r#"
[generate_dialog]
ollama_host = "localhost"
system_prompt = "You are analyzing video frames."
"#
    )
    .expect("Failed to write to temp file");

    let config = AppConfig::load(&temp_file.path().to_path_buf()).expect("Should load config");

    assert_eq!(
        config.generate_dialog.system_prompt,
        Some("You are analyzing video frames.".to_string())
    );
}
