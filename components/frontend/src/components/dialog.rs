//! Node dialog components.

use gloo_net::http::Request;
use serde::Deserialize;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlInputElement};
use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{
    Node, NodeData, SelectorData, StillPreviewData, StillSamplerData, UploadStatus, VideoInputData,
    ViewerData,
};

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
        NodeData::Viewer(data) => render_viewer_dialog(node, data, state),
        NodeData::Selector(data) => render_selector_dialog(node, data, state),
        NodeData::StillPreview(data) => render_still_preview_dialog(node, data, state),
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
    data: &StillSamplerData,
    state: &AppStateContext,
) -> Html {
    let node_id = node.id;
    let interval = data.interval_seconds;
    let current_stills = data.extracted_stills.len();
    let connected_video = find_connected_video(node, state);
    let video_duration = connected_video.as_ref().and_then(|v| v.duration_seconds);

    let state_update = state.clone();
    let on_interval_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
        if let Ok(new_interval) = input.value().parse::<u32>() {
            let clamped = new_interval.clamp(1, 300);
            let mut new_data = StillSamplerData {
                interval_seconds: clamped,
                ..Default::default()
            };
            // Regenerate stills if we have a connected video
            if let Some(duration) = video_duration {
                new_data.generate_stills(duration);
            }
            state_update.dispatch(AppAction::UpdateNodeData(
                node_id,
                NodeData::StillSampler(new_data),
            ));
        }
    });

    html! {
        <div class="dialog-body">
            <h3>{"Still Sampler"}</h3>
            <div class="dialog-row">
                <label>{"Sample Interval (seconds):"}</label>
                <input
                    type="number"
                    min="1"
                    max="300"
                    value={interval.to_string()}
                    onchange={on_interval_change}
                />
            </div>
            <div class="dialog-row">
                <label>{"Stills extracted:"}</label>
                <span>{current_stills}</span>
            </div>
            if video_duration.is_none() {
                <div class="dialog-row hint">
                    {"Connect a video input to extract stills"}
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

fn render_viewer_dialog(node: &Node, _data: &ViewerData, state: &AppStateContext) -> Html {
    let connected_video = find_connected_video(node, state);

    match connected_video {
        Some(video) if video.file_id.is_some() => {
            let file_id = video.file_id.unwrap();
            let video_name = video.file_name.unwrap_or_else(|| "Unknown".to_string());
            let duration = video
                .duration_seconds
                .map(format_duration)
                .unwrap_or_else(|| "Unknown".to_string());
            let stream_url = format!("/api/v1/videos/{}/stream", file_id);

            html! {
                <div class="dialog-body viewer-dialog">
                    <h3>{"Viewer"}</h3>
                    <div class="dialog-row">
                        <label>{"Video:"}</label>
                        <span>{video_name}</span>
                    </div>
                    <div class="dialog-row">
                        <label>{"Duration:"}</label>
                        <span>{duration}</span>
                    </div>
                    <div class="video-player">
                        <video controls=true width="100%">
                            <source src={stream_url} type="video/mp4" />
                            {"Your browser does not support video playback."}
                        </video>
                    </div>
                </div>
            }
        }
        _ => {
            html! {
                <div class="dialog-body">
                    <h3>{"Viewer"}</h3>
                    <div class="dialog-row hint">
                        {"Connect a video input to view video"}
                    </div>
                </div>
            }
        }
    }
}

fn format_duration(seconds: f64) -> String {
    let mins = (seconds / 60.0).floor() as u32;
    let secs = (seconds % 60.0).floor() as u32;
    format!("{}:{:02}", mins, secs)
}

fn render_selector_dialog(node: &Node, data: &SelectorData, state: &AppStateContext) -> Html {
    let node_id = node.id;
    let current_index = data.selected_index;
    let stills_count = find_stills_count(node, state);

    let state_update = state.clone();
    let on_index_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
        if let Ok(idx) = input.value().parse::<usize>() {
            state_update.dispatch(AppAction::UpdateNodeData(
                node_id,
                NodeData::Selector(SelectorData {
                    selected_index: idx,
                }),
            ));
        }
    });

    html! {
        <div class="dialog-body">
            <h3>{"Selector"}</h3>
            <div class="dialog-row">
                <label>{"Still Count:"}</label>
                <span>{if stills_count > 0 { stills_count.to_string() } else { "No input".to_string() }}</span>
            </div>
            <div class="dialog-row">
                <label>{"Selected Index:"}</label>
                <input
                    type="number"
                    min="0"
                    max={stills_count.saturating_sub(1).to_string()}
                    value={current_index.to_string()}
                    onchange={on_index_change}
                    disabled={stills_count == 0}
                />
            </div>
            if stills_count > 0 && current_index < stills_count {
                <div class="dialog-row hint">
                    {format!("Selecting still {} of {}", current_index + 1, stills_count)}
                </div>
            }
        </div>
    }
}

fn find_stills_count(node: &Node, state: &AppStateContext) -> usize {
    let input_conn = match node.inputs.first() {
        Some(c) => c,
        None => return 0,
    };
    let connection = match state
        .connections
        .values()
        .find(|c| c.to_node == node.id && c.to_connector == input_conn.id)
    {
        Some(c) => c,
        None => return 0,
    };
    let source_node = match state.nodes.get(&connection.from_node) {
        Some(n) => n,
        None => return 0,
    };
    match &source_node.data {
        NodeData::StillSampler(data) => data.extracted_stills.len(),
        NodeData::Selector(_) => find_stills_count(source_node, state),
        _ => 0,
    }
}

fn render_still_preview_dialog(
    node: &Node,
    _data: &StillPreviewData,
    state: &AppStateContext,
) -> Html {
    let still_info = find_still_info(node, state);

    match still_info {
        Some((video_id, timestamp, video_name)) => {
            let image_url = format!("/api/v1/stills/{}/{:.2}", video_id, timestamp);

            html! {
                <div class="dialog-body still-preview-dialog">
                    <h3>{"Still Preview"}</h3>
                    <div class="dialog-row">
                        <label>{"Source:"}</label>
                        <span>{video_name}</span>
                    </div>
                    <div class="dialog-row">
                        <label>{"Timestamp:"}</label>
                        <span>{format_duration(timestamp)}</span>
                    </div>
                    <div class="still-image-large">
                        <img src={image_url} alt="Still preview" />
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <div class="dialog-body">
                    <h3>{"Still Preview"}</h3>
                    <div class="dialog-row hint">
                        {"Connect a Selector to view still"}
                    </div>
                </div>
            }
        }
    }
}

fn find_still_info(node: &Node, state: &AppStateContext) -> Option<(Uuid, f64, String)> {
    // Find the connected Selector node
    let input_conn = node.inputs.first()?;
    let connection = state
        .connections
        .values()
        .find(|c| c.to_node == node.id && c.to_connector == input_conn.id)?;
    let selector_node = state.nodes.get(&connection.from_node)?;

    // Get the selected index from Selector
    let selected_index = match &selector_node.data {
        NodeData::Selector(data) => data.selected_index,
        _ => return None,
    };

    // Find the stills array from the Selector's input
    let stills = find_stills_from_selector(selector_node, state)?;
    let still = stills.get(selected_index)?;

    // Find the video source by tracing back
    let (video_id, video_name) = find_video_info_upstream(selector_node, state)?;

    Some((video_id, still.timestamp_seconds, video_name))
}

fn find_stills_from_selector(
    selector_node: &Node,
    state: &AppStateContext,
) -> Option<Vec<yt_rs_shared::Still>> {
    let input_conn = selector_node.inputs.first()?;
    let connection = state
        .connections
        .values()
        .find(|c| c.to_node == selector_node.id && c.to_connector == input_conn.id)?;
    let source_node = state.nodes.get(&connection.from_node)?;

    match &source_node.data {
        NodeData::StillSampler(data) => Some(data.extracted_stills.clone()),
        NodeData::Selector(_) => find_stills_from_selector(source_node, state),
        _ => None,
    }
}

fn find_video_info_upstream(node: &Node, state: &AppStateContext) -> Option<(Uuid, String)> {
    for input in &node.inputs {
        let connection = state
            .connections
            .values()
            .find(|c| c.to_node == node.id && c.to_connector == input.id);
        if let Some(conn) = connection {
            let upstream = state.nodes.get(&conn.from_node)?;
            match &upstream.data {
                NodeData::VideoInput(data) => {
                    let id = data.file_id?;
                    let name = data
                        .file_name
                        .clone()
                        .unwrap_or_else(|| "Unknown".to_string());
                    return Some((id, name));
                }
                _ => {
                    if let Some(info) = find_video_info_upstream(upstream, state) {
                        return Some(info);
                    }
                }
            }
        }
    }
    None
}
