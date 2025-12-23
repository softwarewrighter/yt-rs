//! VideoStore implementation for FileStore.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::CrudError;
use crate::store::FileStore;
use crate::video::VideoStore;

#[async_trait]
impl VideoStore for FileStore {
    async fn save(&self, id: Uuid, filename: &str, data: &[u8]) -> Result<PathBuf, CrudError> {
        let dir = self.base_path.join("work/videos");
        tokio::fs::create_dir_all(&dir).await?;
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("mp4");
        let path = dir.join(format!("{}.{}", id, ext));
        tokio::fs::write(&path, data).await?;
        Ok(path)
    }

    async fn get_path(&self, id: Uuid) -> Result<Option<PathBuf>, CrudError> {
        let dir = self.base_path.join("work/videos");
        for ext in ["mp4", "mov", "avi", "webm", "mkv"] {
            let path = dir.join(format!("{}.{}", id, ext));
            if path.exists() {
                return Ok(Some(path));
            }
        }
        Ok(None)
    }
}
