//! File-based storage implementation.

mod project_impl;
mod video_impl;

use std::path::PathBuf;

/// File system based storage implementation.
#[derive(Clone)]
pub struct FileStore {
    pub(crate) base_path: PathBuf,
}

impl FileStore {
    /// Creates a new file store at the given base path.
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }
}
