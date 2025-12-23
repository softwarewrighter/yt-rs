//! Server setup for yt-rs backend.

mod config;
mod runner;
mod shutdown;

pub use config::ServerConfig;
pub use runner::run;
pub use shutdown::ShutdownSignal;
