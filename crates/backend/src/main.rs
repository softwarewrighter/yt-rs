//! yt-rs backend server.
//!
//! A REST API server for the node editor that serves:
//! - Static WASM frontend files
//! - Project CRUD endpoints
//! - File upload/download
//! - Video processing jobs

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod state;

use routes::create_router;
use state::AppState;

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
    /// Port to listen on.
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// Directory to store uploaded files and project data.
    #[arg(short, long, default_value = "./data")]
    pub data_dir: PathBuf,

    /// Directory containing static frontend files.
    #[arg(short, long, default_value = "./dist")]
    pub static_dir: PathBuf,
}

const fn version_string() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "\n",
        "Copyright (c) 2024 Software Wrighter\n",
        "License: MIT\n",
        "Repository: https://github.com/softwarewrighter/yt-rs\n",
        "Build Host: ", env!("BUILD_HOST"), "\n",
        "Build Commit: ", env!("BUILD_COMMIT"), "\n",
        "Build Time: ", env!("BUILD_TIME")
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yt_rs_backend=debug,tower_http=debug".into()),
        )
        .init();

    let args = Args::parse();

    // Ensure data directory exists
    std::fs::create_dir_all(&args.data_dir)?;

    tracing::info!("Starting yt-rs server on port {}", args.port);
    tracing::info!("Data directory: {:?}", args.data_dir);
    tracing::info!("Static files: {:?}", args.static_dir);

    let state = AppState::new(args.data_dir.clone());

    // Build router
    let app = create_router(state)
        .nest_service("/", ServeDir::new(&args.static_dir))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
