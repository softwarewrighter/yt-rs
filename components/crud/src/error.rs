//! Error types for CRUD operations.

use thiserror::Error;

/// Errors from CRUD operations.
#[derive(Debug, Error)]
pub enum CrudError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<serde_json::Error> for CrudError {
    fn from(e: serde_json::Error) -> Self {
        CrudError::Serialization(e.to_string())
    }
}
