//! Agent module for yt-rs - handles AI model communication.

mod client;
mod config;
mod error;
mod request;
mod response;
mod transport;

pub use client::OllamaClient;
pub use config::OllamaConfig;
pub use error::OllamaError;
