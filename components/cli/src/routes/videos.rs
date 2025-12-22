//! Video upload and management routes.

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

/// Video metadata response.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoMeta {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}

/// Creates video routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/videos/upload", post(upload_video))
}

async fn upload_video(State(state): State<AppState>, mut multipart: Multipart) -> Result<Json<VideoMeta>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.file_name().map(|s| s.to_string()).unwrap_or_else(|| "video.mp4".to_string());
        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        let id = Uuid::new_v4();
        let path = state.save_video(id, &name, &data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(Json(VideoMeta {
            id,
            name,
            path,
            size_bytes: data.len() as u64,
        }));
    }
    Err(StatusCode::BAD_REQUEST)
}
