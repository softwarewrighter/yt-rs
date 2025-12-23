//! CRUD operations for yt-rs data management.
//!
//! Provides data access abstractions through traits:
//! - `ProjectStore` - project persistence operations
//! - `VideoStore` - video file operations
//!
//! # Usage
//! ```ignore
//! use yt_rs_crud::{FileStore, ProjectStore, VideoStore};
//! let store = FileStore::new("./data");
//! ```

mod error;
mod project;
mod store;
mod video;

pub use error::CrudError;
pub use project::ProjectStore;
pub use store::FileStore;
pub use video::VideoStore;
