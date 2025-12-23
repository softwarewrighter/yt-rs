//! REST API routes for yt-rs backend.

mod health;
mod router;
mod shutdown;

pub use router::create_router;
pub use shutdown::ShutdownRoute;
