//! Video upload and management routes.

use std::path::Path;

use axum::{
    Json, Router,
    extract::{Multipart, State},
    http::StatusCode,
    routing::post,
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
    pub duration_seconds: Option<f64>,
}

/// Creates video routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/videos/upload", post(upload_video))
}

async fn upload_video(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<VideoMeta>, StatusCode> {
    let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let name = field
        .file_name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "video.mp4".to_string());
    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

    let id = Uuid::new_v4();
    let path = state
        .save_video(id, &name, &data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let duration_seconds = yt_rs_ffmpeg::get_duration(Path::new(&path)).await.ok();

    Ok(Json(VideoMeta {
        id,
        name,
        path,
        size_bytes: data.len() as u64,
        duration_seconds,
    }))
}
