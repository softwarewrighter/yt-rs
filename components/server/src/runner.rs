//! Server runner.

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::config::ServerConfig;
use crate::shutdown::ShutdownSignal;

/// Runs the server with the given router and configuration.
pub async fn run<F>(
    router: Router,
    config: ServerConfig,
    shutdown: ShutdownSignal,
    on_ready: F,
) -> anyhow::Result<()>
where
    F: FnOnce() + Send + 'static,
{
    let app = build_app(router, &config);
    let listener = bind_listener(&config).await?;

    on_ready();

    serve_with_shutdown(listener, app, shutdown).await
}

fn build_app(router: Router, config: &ServerConfig) -> Router {
    router
        .nest_service("/", ServeDir::new(&config.static_dir))
        .layer(cors_layer())
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

async fn bind_listener(config: &ServerConfig) -> anyhow::Result<TcpListener> {
    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on http://{}", addr);
    Ok(listener)
}

async fn serve_with_shutdown(
    listener: TcpListener,
    app: Router,
    shutdown: ShutdownSignal,
) -> anyhow::Result<()> {
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { shutdown.wait().await })
        .await?;
    tracing::info!("Server shut down gracefully");
    Ok(())
}
