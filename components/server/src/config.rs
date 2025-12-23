//! Server configuration.

use std::net::SocketAddr;
use std::path::PathBuf;

/// Configuration for the server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Port to listen on.
    pub port: u16,
    /// Directory for static files.
    pub static_dir: PathBuf,
}

impl ServerConfig {
    /// Creates a new server configuration.
    pub fn new(port: u16, static_dir: PathBuf) -> Self {
        Self { port, static_dir }
    }

    /// Returns the socket address to bind to.
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.port))
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            static_dir: PathBuf::from("./dist"),
        }
    }
}
