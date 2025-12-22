//! Node dialog components.

use gloo_net::http::Request;
use serde::Deserialize;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlInputElement};
use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{Node, NodeData, UploadStatus, VideoInputData};

/// Video metadata response from backend.
#[derive(Debug, Clone, Deserialize)]
struct VideoMeta {
    id: Uuid,
    name: String,
    path: String,
    duration_seconds: Option<f64>,
}

/// Renders the node dialog if one is open.
pub fn render_dialog(state: &AppStateContext) -> Html {
    let Some(node_id) = state.open_dialog else {
        return html! {};
    };
    let Some(node) = state.nodes.get(&node_id) else {
        return html! {};
    };
    let state_close = state.clone();
    let on_close = Callback::from(move |_| state_close.dispatch(AppAction::CloseDialog));

    html! {
        <div class="dialog-overlay" onclick={on_close.clone()}>
            <div class="dialog" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                {render_dialog_content(node, state)}
                <button class="dialog-close" onclick={on_close}>{"Close"}</button>
            </div>
        </div>
    }
}

fn render_dialog_content(node: &Node, state: &AppStateContext) -> Html {
    match &node.data {
        NodeData::VideoInput(data) => render_video_input_dialog(node, data, state),
        NodeData::StillSampler(data) => render_still_sampler_dialog(node, data, state),
    }
}

fn render_video_input_dialog(node: &Node, data: &VideoInputData, state: &AppStateContext) -> Html {
    let node_id = node.id;
    let has_video = data.file_name.is_some();
    let video_name = data.file_name.clone().unwrap_or_else(|| "None".to_string());
    let duration = data
        .duration_seconds
        .map(|d| format!("{:.1}s", d))
        .unwrap_or_default();

    let is_uploading = matches!(data.upload_status, UploadStatus::Uploading { .. });

    let state_upload = state.clone();
    let on_file_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
        if let Some(files) = input.files()
            && let Some(file) = files.get(0)
        {
            let state = state_upload.clone();
            let file_name = file.name();

            // Set uploading status
            state.dispatch(AppAction::UpdateNodeData(
                node_id,
                NodeData::VideoInput(VideoInputData {
                    file_id: None,
                    file_name: Some(file_name.clone()),
                    file_path: None,
                    duration_seconds: None,
                    upload_status: UploadStatus::Uploading { progress: 0.0 },
                }),
            ));

            spawn_local(async move {
                match upload_video(file).await {
                    Ok(meta) => {
                        state.dispatch(AppAction::UpdateNodeData(
                            node_id,
                            NodeData::VideoInput(VideoInputData {
                                file_id: Some(meta.id),
                                file_name: Some(meta.name),
                                file_path: Some(meta.path),
                                duration_seconds: meta.duration_seconds,
                                upload_status: UploadStatus::Complete,
                            }),
                        ));
                    }
                    Err(err) => {
                        log::error!("Upload failed: {}", err);
                        state.dispatch(AppAction::UpdateNodeData(
                            node_id,
                            NodeData::VideoInput(VideoInputData {
                                file_id: None,
                                file_name: None,
                                file_path: None,
                                duration_seconds: None,
                                upload_status: UploadStatus::Error(err),
                            }),
                        ));
                    }
                }
            });
        }
    });

    let state_delete = state.clone();
    let on_delete = Callback::from(move |_| {
        state_delete.dispatch(AppAction::UpdateNodeData(
            node_id,
            NodeData::VideoInput(VideoInputData::default()),
        ));
    });

    html! {
        <div class="dialog-body">
            <h3>{"Video Input"}</h3>
            <div class="dialog-row">
                <label>{"Video:"}</label>
                <span>{video_name}</span>
            </div>
            if has_video {
                <div class="dialog-row">
                    <label>{"Duration:"}</label>
                    <span>{duration}</span>
                </div>
            }
            if is_uploading {
                <div class="dialog-row">
                    <span class="uploading">{"Uploading..."}</span>
                </div>
            }
            <div class="dialog-actions">
                <button onclick={on_delete} disabled={!has_video || is_uploading}>
                    {"Delete Video"}
                </button>
                <label class="file-button">
                    {if is_uploading { "Uploading..." } else { "Load Video..." }}
                    <input
                        type="file"
                        accept="video/*"
                        onchange={on_file_change}
                        disabled={is_uploading}
                        style="display: none;"
                    />
                </label>
            </div>
        </div>
    }
}

async fn upload_video(file: web_sys::File) -> Result<VideoMeta, String> {
    let form_data = FormData::new().map_err(|e| format!("FormData error: {:?}", e))?;
    form_data
        .append_with_blob("file", &file)
        .map_err(|e| format!("Append error: {:?}", e))?;

    let response = Request::post("/api/v1/videos/upload")
        .body(form_data)
        .map_err(|e| format!("Request error: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Send error: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Upload failed: {}", response.status()));
    }

    response
        .json::<VideoMeta>()
        .await
        .map_err(|e| format!("Parse error: {:?}", e))
}

fn render_still_sampler_dialog(
    node: &Node,
    data: &yt_rs_shared::StillSamplerData,
    state: &AppStateContext,
) -> Html {
    let interval = data.interval_seconds;
    let connected_video = find_connected_video(node, state);
    let still_count = connected_video.as_ref().and_then(|v| {
        v.duration_seconds
            .map(|d| (d / interval as f64).floor() as u32)
    });

    html! {
        <div class="dialog-body">
            <h3>{"Still Sampler"}</h3>
            <div class="dialog-row">
                <label>{"Sample Interval:"}</label>
                <span>{format!("{} seconds", interval)}</span>
            </div>
            if let Some(count) = still_count {
                <div class="dialog-row">
                    <label>{"Stills to extract:"}</label>
                    <span>{count}</span>
                </div>
            } else {
                <div class="dialog-row hint">
                    {"Connect a video input to see still count"}
                </div>
            }
        </div>
    }
}

fn find_connected_video(node: &Node, state: &AppStateContext) -> Option<VideoInputData> {
    let input_conn = node.inputs.first()?;
    let connection = state
        .connections
        .values()
        .find(|c| c.to_node == node.id && c.to_connector == input_conn.id)?;
    let source_node = state.nodes.get(&connection.from_node)?;
    match &source_node.data {
        NodeData::VideoInput(data) => Some(data.clone()),
        _ => None,
    }
}
