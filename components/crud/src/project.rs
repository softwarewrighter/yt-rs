//! Project storage trait.

use async_trait::async_trait;
use uuid::Uuid;
use yt_rs_shared::Project;

use crate::error::CrudError;

/// Trait for project persistence operations.
#[async_trait]
pub trait ProjectStore: Send + Sync {
    /// Saves a project.
    async fn save(&self, project: &Project) -> Result<(), CrudError>;

    /// Loads a project by ID.
    async fn load(&self, id: Uuid) -> Result<Option<Project>, CrudError>;

    /// Lists all projects.
    async fn list(&self) -> Result<Vec<Project>, CrudError>;

    /// Deletes a project by ID.
    async fn delete(&self, id: Uuid) -> Result<(), CrudError>;
}
