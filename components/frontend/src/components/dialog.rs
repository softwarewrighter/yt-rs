//! Node dialog rendering using composable UI components.

use gloo_net::http::Request;
use serde::Deserialize;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlInputElement};
use yew::prelude::*;

use super::ui::{DialogBody, DialogRow, HintRow, NumberInput};
use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{
    GenerateDialogData, GenerationStatus, Node, NodeData, SelectorData, StillSamplerData,
    UploadStatus, VideoInputData,
};

#[derive(Debug, Clone, Deserialize)]
struct VideoMeta {
    id: Uuid,
    name: String,
    path: String,
    duration_seconds: Option<f64>,
}

/// Renders the node dialog overlay if one is open.
pub fn render_dialog(state: &AppStateContext) -> Html {
    let Some(node_id) = state.open_dialog else {
        return html! {};
    };
    let Some(node) = state.nodes.get(&node_id).cloned() else {
        return html! {};
    };
    let state_close = state.clone();
    let on_close = Callback::from(move |_| state_close.dispatch(AppAction::CloseDialog));

    html! {
        <div class="dialog-overlay" onclick={on_close.clone()}>
            <div class="dialog" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <DialogContent node={node} state={state.clone()} />
                <button class="dialog-close" onclick={on_close}>{"Close"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DialogContentProps {
    node: Node,
    state: AppStateContext,
}

#[function_component(DialogContent)]
fn dialog_content(props: &DialogContentProps) -> Html {
    match &props.node.data {
        NodeData::VideoInput(d) => {
            html! { <VideoInputDlg id={props.node.id} data={d.clone()} state={props.state.clone()} /> }
        }
        NodeData::StillSampler(d) => {
            html! { <StillSamplerDlg node={props.node.clone()} data={d.clone()} state={props.state.clone()} /> }
        }
        NodeData::Viewer(_) => render_viewer(&props.node, &props.state),
        NodeData::Selector(d) => {
            html! { <SelectorDlg node={props.node.clone()} data={d.clone()} state={props.state.clone()} /> }
        }
        NodeData::StillPreview(_) => render_still_preview(&props.node, &props.state),
        NodeData::GenerateDialog(d) => render_generate(&props.node, d, &props.state),
        NodeData::TextView(_) => render_text_view(&props.node, &props.state),
    }
}

// === VideoInput Dialog ===

#[derive(Properties, PartialEq)]
struct VideoInputDlgProps {
    id: Uuid,
    data: VideoInputData,
    state: AppStateContext,
}

#[function_component(VideoInputDlg)]
fn video_input_dlg(props: &VideoInputDlgProps) -> Html {
    let id = props.id;
    let d = &props.data;
    let has_video = d.file_name.is_some();
    let is_uploading = matches!(d.upload_status, UploadStatus::Uploading { .. });

    let on_file = {
        let state = props.state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            if let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                start_upload(id, file, state.clone());
            }
        })
    };
    let on_del = {
        let state = props.state.clone();
        Callback::from(move |_| {
            state.dispatch(AppAction::UpdateNodeData(
                id,
                NodeData::VideoInput(VideoInputData::default()),
            ))
        })
    };

    html! {
        <DialogBody title="Video Input">
            <DialogRow label="Video:"><span>{d.file_name.clone().unwrap_or("None".into())}</span></DialogRow>
            if has_video { <DialogRow label="Duration:"><span>{d.duration_seconds.map(|s| format!("{:.1}s", s)).unwrap_or_default()}</span></DialogRow> }
            if is_uploading { <DialogRow label=""><span class="uploading">{"Uploading..."}</span></DialogRow> }
            <div class="dialog-actions">
                <button onclick={on_del} disabled={!has_video || is_uploading}>{"Delete Video"}</button>
                <label class="file-button">{if is_uploading { "Uploading..." } else { "Load Video..." }}
                    <input type="file" accept="video/*" onchange={on_file} disabled={is_uploading} style="display: none;" />
                </label>
            </div>
        </DialogBody>
    }
}

fn start_upload(id: Uuid, file: web_sys::File, state: AppStateContext) {
    let name = file.name();
    state.dispatch(AppAction::UpdateNodeData(
        id,
        NodeData::VideoInput(VideoInputData {
            file_id: None,
            file_name: Some(name),
            file_path: None,
            duration_seconds: None,
            upload_status: UploadStatus::Uploading { progress: 0.0 },
        }),
    ));
    spawn_local(async move {
        match upload_video(file).await {
            Ok(m) => state.dispatch(AppAction::UpdateNodeData(
                id,
                NodeData::VideoInput(VideoInputData {
                    file_id: Some(m.id),
                    file_name: Some(m.name),
                    file_path: Some(m.path),
                    duration_seconds: m.duration_seconds,
                    upload_status: UploadStatus::Complete,
                }),
            )),
            Err(e) => state.dispatch(AppAction::UpdateNodeData(
                id,
                NodeData::VideoInput(VideoInputData {
                    file_id: None,
                    file_name: None,
                    file_path: None,
                    duration_seconds: None,
                    upload_status: UploadStatus::Error(e),
                }),
            )),
        }
    });
}

async fn upload_video(file: web_sys::File) -> Result<VideoMeta, String> {
    let form = FormData::new().map_err(|e| format!("{:?}", e))?;
    form.append_with_blob("file", &file)
        .map_err(|e| format!("{:?}", e))?;
    let resp = Request::post("/api/v1/videos/upload")
        .body(form)
        .map_err(|e| format!("{:?}", e))?
        .send()
        .await
        .map_err(|e| format!("{:?}", e))?;
    if !resp.ok() {
        return Err(format!("Upload failed: {}", resp.status()));
    }
    resp.json().await.map_err(|e| format!("{:?}", e))
}

// === StillSampler Dialog ===

#[derive(Properties, PartialEq)]
struct StillSamplerDlgProps {
    node: Node,
    data: StillSamplerData,
    state: AppStateContext,
}

#[function_component(StillSamplerDlg)]
fn still_sampler_dlg(props: &StillSamplerDlgProps) -> Html {
    let id = props.node.id;
    let d = &props.data;
    let video = find_connected_video(&props.node, &props.state);
    let dur = video.as_ref().and_then(|v| v.duration_seconds);

    let on_change = {
        let state = props.state.clone();
        Callback::from(move |v: i32| {
            let mut data = StillSamplerData {
                interval_seconds: v.clamp(1, 300) as u32,
                ..Default::default()
            };
            if let Some(d) = dur {
                data.generate_stills(d);
            }
            state.dispatch(AppAction::UpdateNodeData(id, NodeData::StillSampler(data)));
        })
    };

    html! {
        <DialogBody title="Still Sampler">
            <DialogRow label="Sample Interval (seconds):">
                <NumberInput value={d.interval_seconds as i32} min=1 max=300 on_change={on_change} />
            </DialogRow>
            <DialogRow label="Stills extracted:"><span>{d.extracted_stills.len()}</span></DialogRow>
            if dur.is_none() { <HintRow message="Connect a video input to extract stills" /> }
        </DialogBody>
    }
}

// === Selector Dialog ===

#[derive(Properties, PartialEq)]
struct SelectorDlgProps {
    node: Node,
    data: SelectorData,
    state: AppStateContext,
}

#[function_component(SelectorDlg)]
fn selector_dlg(props: &SelectorDlgProps) -> Html {
    let id = props.node.id;
    let idx = props.data.selected_index as i32;
    let cnt = find_stills_count(&props.node, &props.state);
    let max = (cnt.saturating_sub(1)) as i32;

    let on_change = {
        let state = props.state.clone();
        Callback::from(move |v: i32| {
            state.dispatch(AppAction::UpdateNodeData(
                id,
                NodeData::Selector(SelectorData {
                    selected_index: v.clamp(0, max) as usize,
                }),
            ));
        })
    };

    html! {
        <DialogBody title="Selector">
            <DialogRow label="Still Count:"><span>{if cnt > 0 { cnt.to_string() } else { "No input".into() }}</span></DialogRow>
            <DialogRow label="Selected Index:">
                <NumberInput value={idx} min=0 max={max} on_change={on_change} disabled={cnt == 0} />
            </DialogRow>
            if cnt > 0 { <HintRow message={format!("Selecting still {} of {}", idx + 1, cnt)} /> }
        </DialogBody>
    }
}

// === Simple dialogs as functions ===

fn render_viewer(node: &Node, state: &AppStateContext) -> Html {
    match find_connected_video(node, state) {
        Some(v) if v.file_id.is_some() => {
            let url = format!("/api/v1/videos/{}/stream", v.file_id.unwrap());
            html! {
                <DialogBody title="Viewer" class="viewer-dialog">
                    <DialogRow label="Video:"><span>{v.file_name.unwrap_or("Unknown".into())}</span></DialogRow>
                    <DialogRow label="Duration:"><span>{v.duration_seconds.map(fmt_dur).unwrap_or("?".into())}</span></DialogRow>
                    <div class="video-player"><video controls=true width="100%"><source src={url} type="video/mp4" /></video></div>
                </DialogBody>
            }
        }
        _ => {
            html! { <DialogBody title="Viewer"><HintRow message="Connect a video input to view video" /></DialogBody> }
        }
    }
}

fn render_still_preview(node: &Node, state: &AppStateContext) -> Html {
    if let Some((vid, ts, name)) = find_still_info(node, state) {
        let url = format!("/api/v1/stills/{}/{:.2}", vid, ts);
        html! {
            <DialogBody title="Still Preview" class="still-preview-dialog">
                <DialogRow label="Source:"><span>{name}</span></DialogRow>
                <DialogRow label="Timestamp:"><span>{fmt_dur(ts)}</span></DialogRow>
                <div class="still-image-large"><img src={url} alt="Still" /></div>
            </DialogBody>
        }
    } else {
        html! { <DialogBody title="Still Preview"><HintRow message="Connect a Selector to view still" /></DialogBody> }
    }
}

fn render_generate(node: &Node, d: &GenerateDialogData, state: &AppStateContext) -> Html {
    let cnt = find_stills_count(node, state);
    let status = match &d.generation_status {
        GenerationStatus::Idle => "Ready".into(),
        GenerationStatus::Generating {
            current_still,
            total_stills,
        } => format!("Processing {}/{}...", current_still, total_stills),
        GenerationStatus::Complete => "Complete".into(),
        GenerationStatus::Error(e) => format!("Error: {}", e),
    };
    html! {
        <DialogBody title="Generate Dialog">
            <DialogRow label="Input stills:"><span>{if cnt > 0 { cnt.to_string() } else { "No input".into() }}</span></DialogRow>
            <DialogRow label="Status:"><span>{status}</span></DialogRow>
            if cnt == 0 { <HintRow message="Connect a Still Sampler or Selector" /> }
        </DialogBody>
    }
}

fn render_text_view(node: &Node, state: &AppStateContext) -> Html {
    if let Some(dlg) = find_text_upstream(node, state) {
        html! {
            <DialogBody title="Text View" class="text-view-dialog">
                <div class="text-section"><h4>{"Prolog"}</h4><p>{&dlg.prolog}</p></div>
                <div class="text-section"><h4>{"Epilog"}</h4><p>{&dlg.epilog}</p></div>
                <div class="text-section"><h4>{"YouTube Description"}</h4><pre>{&dlg.youtube_description}</pre></div>
            </DialogBody>
        }
    } else {
        html! { <DialogBody title="Text View"><HintRow message="Connect a Generate Dialog node" /></DialogBody> }
    }
}

// === Graph helpers ===

fn find_connected_video(node: &Node, state: &AppStateContext) -> Option<VideoInputData> {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    match &state.nodes.get(&conn.from_node)?.data {
        NodeData::VideoInput(d) => Some(d.clone()),
        _ => None,
    }
}

fn find_stills_count(node: &Node, state: &AppStateContext) -> usize {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    });
    match conn
        .and_then(|c| state.nodes.get(&c.from_node))
        .map(|n| &n.data)
    {
        Some(NodeData::StillSampler(d)) => d.extracted_stills.len(),
        Some(NodeData::Selector(_)) => conn
            .and_then(|c| state.nodes.get(&c.from_node))
            .map(|n| find_stills_count(n, state))
            .unwrap_or(0),
        _ => 0,
    }
}

fn find_still_info(node: &Node, state: &AppStateContext) -> Option<(Uuid, f64, String)> {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    let sel = state.nodes.get(&conn.from_node)?;
    let idx = match &sel.data {
        NodeData::Selector(d) => d.selected_index,
        _ => return None,
    };
    let stills = find_stills_from(sel, state)?;
    let ts = stills.get(idx)?.timestamp_seconds;
    let (vid, name) = find_video_upstream(sel, state)?;
    Some((vid, ts, name))
}

fn find_stills_from(node: &Node, state: &AppStateContext) -> Option<Vec<yt_rs_shared::Still>> {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    match &state.nodes.get(&conn.from_node)?.data {
        NodeData::StillSampler(d) => Some(d.extracted_stills.clone()),
        NodeData::Selector(_) => find_stills_from(state.nodes.get(&conn.from_node)?, state),
        _ => None,
    }
}

fn find_video_upstream(node: &Node, state: &AppStateContext) -> Option<(Uuid, String)> {
    for input in &node.inputs {
        if let Some(conn) = state
            .connections
            .values()
            .find(|c| c.to_node == node.id && c.to_connector == input.id)
        {
            if let Some(up) = state.nodes.get(&conn.from_node) {
                if let NodeData::VideoInput(d) = &up.data {
                    return Some((d.file_id?, d.file_name.clone().unwrap_or("?".into())));
                }
                if let Some(r) = find_video_upstream(up, state) {
                    return Some(r);
                }
            }
        }
    }
    None
}

fn find_text_upstream(
    node: &Node,
    state: &AppStateContext,
) -> Option<yt_rs_shared::GeneratedDialog> {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    match &state.nodes.get(&conn.from_node)?.data {
        NodeData::GenerateDialog(d) => d.generated_dialog.clone(),
        _ => None,
    }
}

fn fmt_dur(s: f64) -> String {
    format!("{}:{:02}", (s / 60.0) as u32, (s % 60.0) as u32)
}
