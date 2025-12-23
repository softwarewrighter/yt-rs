//! Video storage trait.

use std::path::PathBuf;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::CrudError;

/// Trait for video file operations.
#[async_trait]
pub trait VideoStore: Send + Sync {
    /// Saves video data and returns the path.
    async fn save(&self, id: Uuid, filename: &str, data: &[u8]) -> Result<PathBuf, CrudError>;

    /// Gets the path to a video file if it exists.
    async fn get_path(&self, id: Uuid) -> Result<Option<PathBuf>, CrudError>;
}
