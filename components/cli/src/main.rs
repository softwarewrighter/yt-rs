//! yt-rs backend server.
//!
//! A REST API server for the node editor that serves:
//! - Static WASM frontend files
//! - Project CRUD endpoints
//! - File upload/download
//! - Video processing jobs

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use yt_rs_cli::config::AppConfig;
use yt_rs_cli::routes::{ShutdownState, create_router};
use yt_rs_cli::state::AppState;

const AI_AGENT_INSTRUCTIONS: &str = r#"
AI CODING AGENT INSTRUCTIONS
============================

This is a Rust web application with a Yew/WASM frontend and Axum backend.

Project Structure:
  crates/shared/   - Shared data models (Node, Connection, Canvas, Project)
  crates/backend/  - Axum REST server (routes, state management)
  crates/frontend/ - Yew WASM application (components, state)

Key Commands:
  cargo build              - Build all crates
  cargo clippy -- -D warnings  - Lint (must pass with no warnings)
  cargo test               - Run all tests
  trunk build              - Build WASM frontend (from crates/frontend/)
  trunk serve              - Dev server with hot reload

Code Style:
  - Rust 2024 edition
  - Functions < 50 lines
  - Modules < 7 functions (excluding tests)
  - No #[allow(...)] - fix underlying issues
  - TDD: write tests before implementation

Architecture:
  - Frontend uses Yew's use_reducer for state management
  - SVG canvas with foreignObject for node HTML
  - Bezier curves for connections between nodes
  - REST API: /api/health, /api/projects, /api/projects/{id}
"#;

const LONG_ABOUT: &str = "\
Backend server for yt-rs node editor.

Serves the WASM frontend and provides REST API endpoints for
managing video processing projects with a node-based workflow.";

/// yt-rs node editor backend server.
#[derive(Parser, Debug)]
#[command(name = "yt-rs")]
#[command(version = version_string())]
#[command(about = "Backend server for yt-rs node editor")]
#[command(long_about = LONG_ABOUT)]
#[command(after_help = AI_AGENT_INSTRUCTIONS)]
pub struct Args {
    /// Port to connect to (for stop) or listen on (for serve).
    #[arg(short, long, default_value = "3000", global = true)]
    pub port: u16,

    /// Path to configuration file.
    #[arg(short, long, default_value = "./config.toml", global = true)]
    pub config_file: PathBuf,

    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Subcommands for the CLI.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the server (default if no subcommand given).
    Serve {
        /// Directory to store uploaded files and project data.
        #[arg(short, long, default_value = "./data")]
        data_dir: PathBuf,

        /// Directory containing static frontend files.
        #[arg(short, long, default_value = "./dist")]
        static_dir: PathBuf,
    },
    /// Stop a running server.
    Stop,
}

const fn version_string() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "\n",
        "Copyright (c) 2024 Software Wrighter\n",
        "License: MIT\n",
        "Repository: https://github.com/softwarewrighter/yt-rs\n",
        "Build Host: ",
        env!("BUILD_HOST"),
        "\n",
        "Build Commit: ",
        env!("BUILD_COMMIT"),
        "\n",
        "Build Time: ",
        env!("BUILD_TIME")
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Stop) => stop_server(args.port).await,
        Some(Command::Serve {
            data_dir,
            static_dir,
        }) => serve(args.port, args.config_file, data_dir, static_dir).await,
        None => {
            // Default to serve with default paths
            serve(
                args.port,
                args.config_file,
                PathBuf::from("./data"),
                PathBuf::from("./dist"),
            )
            .await
        }
    }
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yt_rs_backend=debug,tower_http=debug".into()),
        )
        .init();
}

fn load_config(path: &PathBuf) -> AppConfig {
    AppConfig::load(path).unwrap_or_else(|e| {
        tracing::warn!("Failed to load config from {path:?}: {e}");
        AppConfig::default()
    })
}

async fn serve(
    port: u16,
    config_file: PathBuf,
    data_dir: PathBuf,
    static_dir: PathBuf,
) -> anyhow::Result<()> {
    init_tracing();
    let config = load_config(&config_file);
    std::fs::create_dir_all(&data_dir)?;

    tracing::info!("Starting yt-rs server on port {port}");
    tracing::info!("Data directory: {data_dir:?}");
    tracing::info!("Static files: {static_dir:?}");

    let state = AppState::new(data_dir, config);
    let shutdown = ShutdownState::default();
    let shutdown_signal = shutdown.clone();

    let app = create_router(state, shutdown)
        .nest_service("/", ServeDir::new(&static_dir))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on http://{addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move { shutdown_signal.wait().await })
        .await?;

    tracing::info!("Server shut down gracefully");
    Ok(())
}

async fn stop_server(port: u16) -> anyhow::Result<()> {
    let url = format!("http://localhost:{}/api/v1/shutdown", port);
    println!("Sending shutdown request to {}...", url);

    let client = reqwest::Client::new();
    match client.post(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("Server shutdown initiated successfully.");
                Ok(())
            } else {
                anyhow::bail!("Server returned error: {}", response.status());
            }
        }
        Err(e) => {
            if e.is_connect() {
                println!("No server running on port {} (connection refused).", port);
                Ok(())
            } else {
                anyhow::bail!("Failed to connect: {}", e);
            }
        }
    }
}
