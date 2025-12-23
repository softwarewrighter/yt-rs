//! Video upload and management routes.

use axum::{
    Json, Router,
    body::Body,
    extract::{Multipart, Path as AxumPath, State},
    http::{HeaderMap, StatusCode, header},
    response::Response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::state::{self, AppState};

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
    Router::new()
        .route("/videos/upload", post(upload_video))
        .route("/videos/:id/stream", get(stream_video))
        .route("/videos/:id/thumbnail", get(get_thumbnail))
        .route("/stills/:video_id/:timestamp", get(get_still_thumbnail))
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
    let path = state::save_video(&state, id, &name, &data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let duration_seconds = yt_rs_ffmpeg::get_duration(&path).await.ok();
    let path = path.to_string_lossy().to_string();

    Ok(Json(VideoMeta {
        id,
        name,
        path,
        size_bytes: data.len() as u64,
        duration_seconds,
    }))
}

async fn stream_video(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<Uuid>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let video_path = state::get_video_path(&state, id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let file = File::open(&video_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let metadata = file
        .metadata()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_size = metadata.len();

    // Parse Range header for partial content support
    let range = headers
        .get(header::RANGE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| parse_range(s, file_size));

    match range {
        Some((start, end)) => {
            let length = end - start + 1;
            let mut file = file;
            file.seek(std::io::SeekFrom::Start(start))
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let stream = ReaderStream::new(file.take(length));
            let body = Body::from_stream(stream);

            Ok(Response::builder()
                .status(StatusCode::PARTIAL_CONTENT)
                .header(header::CONTENT_TYPE, "video/mp4")
                .header(header::CONTENT_LENGTH, length)
                .header(header::ACCEPT_RANGES, "bytes")
                .header(
                    header::CONTENT_RANGE,
                    format!("bytes {}-{}/{}", start, end, file_size),
                )
                .body(body)
                .unwrap())
        }
        None => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "video/mp4")
                .header(header::CONTENT_LENGTH, file_size)
                .header(header::ACCEPT_RANGES, "bytes")
                .body(body)
                .unwrap())
        }
    }
}

async fn get_thumbnail(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<Uuid>,
) -> Result<Response, StatusCode> {
    let path = state::get_or_create_thumbnail(&state, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    serve_image(&path).await
}

async fn get_still_thumbnail(
    State(state): State<AppState>,
    AxumPath((video_id, timestamp)): AxumPath<(Uuid, f64)>,
) -> Result<Response, StatusCode> {
    let path = state::get_or_create_still(&state, video_id, timestamp)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    serve_image(&path).await
}

async fn serve_image(path: &std::path::Path) -> Result<Response, StatusCode> {
    let file = File::open(path).await.map_err(|_| StatusCode::NOT_FOUND)?;
    let len = file
        .metadata()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .len();
    let body = Body::from_stream(ReaderStream::new(file));
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/jpeg")
        .header(header::CONTENT_LENGTH, len)
        .body(body)
        .unwrap())
}

/// Parses HTTP Range header value like "bytes=0-1023".
fn parse_range(range_header: &str, file_size: u64) -> Option<(u64, u64)> {
    let range_spec = range_header.strip_prefix("bytes=")?;
    let mut parts = range_spec.split('-');

    let start: u64 = parts.next()?.parse().ok()?;
    let end: u64 = parts
        .next()
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
        .and_then(|s| s.parse().ok())
        .unwrap_or(file_size - 1);

    if start <= end && end < file_size {
        Some((start, end))
    } else {
        None
    }
}
