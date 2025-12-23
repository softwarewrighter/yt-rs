//! Dialog generation endpoints using Ollama vision model.

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use uuid::Uuid;
use yt_rs_shared::{ClipDialog, GeneratedDialog};

use super::ollama::OllamaClient;
use crate::state::{self, AppState};

/// Request to generate dialog from video stills.
#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub video_id: Uuid,
    pub timestamps: Vec<f64>,
    /// Optional context about the video to improve generation.
    #[serde(default)]
    pub context: String,
}

/// Response with generated dialog.
#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub dialog: GeneratedDialog,
}

/// Creates generation routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/generate/dialog", post(generate_dialog))
}

async fn generate_dialog(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, StatusCode> {
    let config = &state.config().generate_dialog;
    info!(
        "=== Starting dialog generation for video {} ===",
        req.video_id
    );
    info!(
        "Config: host={}:{}, model={}, system_prompt={:?}",
        config.ollama_host,
        config.ollama_port,
        config.vision_model,
        config
            .system_prompt
            .as_deref()
            .map(|s| &s[..s.len().min(50)])
    );
    info!("Processing {} timestamps", req.timestamps.len());
    if !req.context.is_empty() {
        info!("Context provided: {} chars", req.context.len());
        debug!("Context: {}", req.context);
    }

    let client = OllamaClient::new(config);
    let mut clips = Vec::new();
    let context_ref = if req.context.is_empty() {
        None
    } else {
        Some(req.context.as_str())
    };

    for (i, timestamp) in req.timestamps.iter().enumerate() {
        info!(
            "--- Processing clip {}/{} at {:.1}s ---",
            i + 1,
            req.timestamps.len(),
            timestamp
        );
        let clip = generate_clip(&state, &client, req.video_id, *timestamp, context_ref).await?;
        info!(
            "Clip result: {} chars, interesting={}",
            clip.dialog_text.len(),
            clip.is_interesting
        );
        clips.push(clip);
    }

    info!("--- Generating prolog ---");
    let prolog = match generate_prolog(&client, &clips).await {
        Ok(p) => {
            info!("Prolog: {} chars", p.len());
            p
        }
        Err(e) => {
            error!("Prolog generation failed: {}", e);
            String::new()
        }
    };

    info!("--- Generating epilog ---");
    let epilog = match generate_epilog(&client).await {
        Ok(e) => {
            info!("Epilog: {} chars", e.len());
            e
        }
        Err(e) => {
            error!("Epilog generation failed: {}", e);
            String::new()
        }
    };

    info!("--- Generating YouTube description ---");
    let youtube_description = match generate_youtube_description(&client, &clips).await {
        Ok(d) => {
            info!("YouTube description: {} chars", d.len());
            d
        }
        Err(e) => {
            error!("YouTube description generation failed: {}", e);
            String::new()
        }
    };

    let dialog = GeneratedDialog {
        prolog,
        clips,
        epilog,
        youtube_description,
    };

    info!("=== Dialog generation complete ===");
    Ok(Json(GenerateResponse { dialog }))
}

async fn generate_clip(
    state: &AppState,
    client: &OllamaClient,
    video_id: Uuid,
    timestamp: f64,
    context: Option<&str>,
) -> Result<ClipDialog, StatusCode> {
    let still_path = state::get_or_create_still(state, video_id, timestamp)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let image_data = tokio::fs::read(&still_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let prompt = match context {
        Some(ctx) => format!(
            "Context about this video: {}\n\n\
            Describe what is happening in this video frame. \
            Focus on any interesting action, subjects, or noteworthy elements \
            that relate to the context above. Be concise but descriptive.",
            ctx
        ),
        None => "Describe what is happening in this video frame. \
            Focus on any interesting action, subjects, or noteworthy elements. \
            Be concise but descriptive."
            .to_string(),
    };

    let response = client
        .analyze_image(&image_data, &prompt)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let is_interesting = response.len() > 50;

    Ok(ClipDialog {
        timestamp_seconds: timestamp,
        dialog_text: response,
        is_interesting,
    })
}

async fn generate_prolog(client: &OllamaClient, clips: &[ClipDialog]) -> Result<String, String> {
    if clips.is_empty() {
        return Ok(String::new());
    }

    let context: String = clips
        .iter()
        .take(3)
        .map(|c| format!("At {:.0}s: {}", c.timestamp_seconds, c.dialog_text))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "Based on these video descriptions, write a brief engaging introduction:\n{}\n\n\
        Write 1-2 sentences to introduce this video.",
        context
    );

    // Use empty image for text-only generation
    client.analyze_image(&[], &prompt).await
}

async fn generate_epilog(client: &OllamaClient) -> Result<String, String> {
    let prompt = "Write a brief closing statement for a video, \
        encouraging viewers to like and subscribe. Keep it under 2 sentences.";
    client.analyze_image(&[], prompt).await
}

async fn generate_youtube_description(
    client: &OllamaClient,
    clips: &[ClipDialog],
) -> Result<String, String> {
    if clips.is_empty() {
        return Ok(String::new());
    }

    let context: String = clips
        .iter()
        .filter(|c| c.is_interesting)
        .take(5)
        .map(|c| format!("- {}", c.dialog_text))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "Based on these video highlights, write a YouTube video description:\n{}\n\n\
        Include:\n\
        - Opening hook (1 sentence)\n\
        - What viewers will learn/see\n\
        - Relevant hashtags\n\
        Keep it under 200 words.",
        context
    );

    client.analyze_image(&[], &prompt).await
}
