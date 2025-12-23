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
use serde::Serialize;
use yt_rs_shared::{
    GenerateDialogData, GeneratedDialog, GenerationStatus, Node, NodeData, SelectorData,
    StillSamplerData, UploadStatus, VideoInputData,
};

#[derive(Debug, Clone, Deserialize)]
struct VideoMeta {
    id: Uuid,
    name: String,
    path: String,
    duration_seconds: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct GenerateRequest {
    video_id: Uuid,
    timestamps: Vec<f64>,
    context: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GenerateResponse {
    dialog: GeneratedDialog,
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
    let gen_info = find_generation_info(node, state);
    let is_idle = matches!(d.generation_status, GenerationStatus::Idle);
    let can_generate = cnt > 0 && is_idle && gen_info.is_some();
    let status = match &d.generation_status {
        GenerationStatus::Idle => "Ready".into(),
        GenerationStatus::Generating {
            current_still,
            total_stills,
        } => format!("Processing {}/{}...", current_still, total_stills),
        GenerationStatus::Complete => "Complete".into(),
        GenerationStatus::Error(e) => format!("Error: {}", e),
    };

    let node_id = node.id;
    let context_text = d.context_text.clone();

    let on_context_change = {
        let state = state.clone();
        let data = d.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let mut new_data = data.clone();
            new_data.context_text = input.value();
            state.dispatch(AppAction::UpdateNodeData(
                node_id,
                NodeData::GenerateDialog(new_data),
            ));
        })
    };

    let on_generate = {
        let state = state.clone();
        let ctx = context_text.clone();
        Callback::from(move |_| {
            if let Some((video_id, timestamps)) = find_generation_info_static(&state, node_id) {
                start_generation(state.clone(), node_id, video_id, timestamps, ctx.clone());
            }
        })
    };

    html! {
        <DialogBody title="Generate Dialog">
            <DialogRow label="Input stills:"><span>{if cnt > 0 { cnt.to_string() } else { "No input".into() }}</span></DialogRow>
            <DialogRow label="Status:"><span>{status}</span></DialogRow>
            <DialogRow label="Context:">
                <textarea
                    class="context-input"
                    placeholder="Describe your video content (project, topic, etc.)"
                    value={context_text}
                    oninput={on_context_change}
                    rows="3"
                />
            </DialogRow>
            if cnt == 0 { <HintRow message="Connect a Still Sampler or Selector" /> }
            if d.generated_dialog.is_some() { {render_dialog_preview(d)} }
            <div class="dialog-actions">
                <button disabled={!can_generate} onclick={on_generate}>{"Generate"}</button>
            </div>
        </DialogBody>
    }
}

fn render_dialog_preview(d: &GenerateDialogData) -> Html {
    if let Some(ref dlg) = d.generated_dialog {
        let clip_count = dlg.clips.len();
        let interesting = dlg.clips.iter().filter(|c| c.is_interesting).count();
        html! {
            <div class="generation-preview">
                <DialogRow label="Clips:"><span>{format!("{} ({} interesting)", clip_count, interesting)}</span></DialogRow>
                if !dlg.prolog.is_empty() {
                    <DialogRow label="Prolog:"><span class="preview-text">{&dlg.prolog}</span></DialogRow>
                }
            </div>
        }
    } else {
        html! {}
    }
}

fn find_generation_info(node: &Node, state: &AppStateContext) -> Option<(Uuid, Vec<f64>)> {
    find_generation_info_static(state, node.id)
}

fn find_generation_info_static(state: &AppStateContext, node_id: Uuid) -> Option<(Uuid, Vec<f64>)> {
    let node = state.nodes.get(&node_id)?;
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    let upstream = state.nodes.get(&conn.from_node)?;

    // Get stills from upstream
    let stills = match &upstream.data {
        NodeData::StillSampler(d) => d.extracted_stills.clone(),
        NodeData::Selector(_) => find_stills_recursive(state, upstream)?,
        _ => return None,
    };

    // Get video_id by traversing up to VideoInput
    let video_id = find_video_id_recursive(state, upstream)?;

    let timestamps: Vec<f64> = stills.iter().map(|s| s.timestamp_seconds).collect();
    Some((video_id, timestamps))
}

fn find_stills_recursive(state: &AppStateContext, node: &Node) -> Option<Vec<yt_rs_shared::Still>> {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    let up = state.nodes.get(&conn.from_node)?;
    match &up.data {
        NodeData::StillSampler(d) => Some(d.extracted_stills.clone()),
        NodeData::Selector(_) => find_stills_recursive(state, up),
        _ => None,
    }
}

fn find_video_id_recursive(state: &AppStateContext, node: &Node) -> Option<Uuid> {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    let up = state.nodes.get(&conn.from_node)?;
    match &up.data {
        NodeData::VideoInput(d) => d.file_id,
        _ => find_video_id_recursive(state, up),
    }
}

fn start_generation(
    state: AppStateContext,
    node_id: Uuid,
    video_id: Uuid,
    timestamps: Vec<f64>,
    context: String,
) {
    update_generation_status(
        &state,
        node_id,
        GenerationStatus::Generating {
            current_still: 0,
            total_stills: timestamps.len(),
        },
    );

    spawn_local(async move {
        let request = GenerateRequest {
            video_id,
            timestamps,
            context,
        };
        let result = Request::post("/api/v1/generate/dialog")
            .json(&request)
            .expect("serialize request")
            .send()
            .await;

        match result {
            Ok(resp) if resp.ok() => {
                if let Ok(data) = resp.json::<GenerateResponse>().await {
                    complete_generation(&state, node_id, data.dialog);
                }
            }
            _ => update_generation_status(
                &state,
                node_id,
                GenerationStatus::Error("Generation failed".into()),
            ),
        }
    });
}

fn update_generation_status(state: &AppStateContext, node_id: Uuid, status: GenerationStatus) {
    if let Some(node) = state.nodes.get(&node_id)
        && let NodeData::GenerateDialog(d) = &node.data
    {
        let mut new_data = d.clone();
        new_data.generation_status = status;
        state.dispatch(AppAction::UpdateNodeData(
            node_id,
            NodeData::GenerateDialog(new_data),
        ));
    }
}

fn complete_generation(state: &AppStateContext, node_id: Uuid, dialog: GeneratedDialog) {
    if let Some(node) = state.nodes.get(&node_id)
        && let NodeData::GenerateDialog(d) = &node.data
    {
        let mut new_data = d.clone();
        new_data.generation_status = GenerationStatus::Complete;
        new_data.generated_dialog = Some(dialog);
        state.dispatch(AppAction::UpdateNodeData(
            node_id,
            NodeData::GenerateDialog(new_data),
        ));
    }
}

fn render_text_view(node: &Node, state: &AppStateContext) -> Html {
    match find_generate_dialog_upstream(node, state) {
        Some((status, dialog_opt)) => match status {
            GenerationStatus::Idle => {
                html! { <DialogBody title="Text View"><HintRow message="Click Generate in the connected node" /></DialogBody> }
            }
            GenerationStatus::Generating {
                current_still,
                total_stills,
            } => {
                html! { <DialogBody title="Text View"><HintRow message={format!("Generating... {}/{}", current_still, total_stills)} /></DialogBody> }
            }
            GenerationStatus::Complete => {
                if let Some(dlg) = dialog_opt {
                    html! {
                        <DialogBody title="Text View" class="text-view-dialog">
                            if !dlg.prolog.is_empty() {
                                <div class="text-section"><h4>{"Prolog"}</h4><p>{&dlg.prolog}</p></div>
                            }
                            if !dlg.clips.is_empty() {
                                <div class="text-section">
                                    <h4>{format!("Clips ({})", dlg.clips.len())}</h4>
                                    { for dlg.clips.iter().map(|clip| html! {
                                        <div class="clip-item">
                                            <span class="clip-timestamp">{format!("[{:.1}s]", clip.timestamp_seconds)}</span>
                                            <span class={if clip.is_interesting { "clip-text interesting" } else { "clip-text" }}>
                                                {&clip.dialog_text}
                                            </span>
                                        </div>
                                    }) }
                                </div>
                            }
                            if !dlg.epilog.is_empty() {
                                <div class="text-section"><h4>{"Epilog"}</h4><p>{&dlg.epilog}</p></div>
                            }
                            if !dlg.youtube_description.is_empty() {
                                <div class="text-section"><h4>{"YouTube Description"}</h4><pre>{&dlg.youtube_description}</pre></div>
                            }
                        </DialogBody>
                    }
                } else {
                    html! { <DialogBody title="Text View"><HintRow message="Generation complete but no dialog" /></DialogBody> }
                }
            }
            GenerationStatus::Error(e) => {
                html! { <DialogBody title="Text View"><HintRow message={format!("Error: {}", e)} /></DialogBody> }
            }
        },
        None => {
            html! { <DialogBody title="Text View"><HintRow message="Connect a Generate Dialog node" /></DialogBody> }
        }
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
            && let Some(up) = state.nodes.get(&conn.from_node)
        {
            if let NodeData::VideoInput(d) = &up.data {
                return Some((d.file_id?, d.file_name.clone().unwrap_or("?".into())));
            }
            if let Some(r) = find_video_upstream(up, state) {
                return Some(r);
            }
        }
    }
    None
}

fn find_generate_dialog_upstream(
    node: &Node,
    state: &AppStateContext,
) -> Option<(GenerationStatus, Option<GeneratedDialog>)> {
    let conn = state.connections.values().find(|c| {
        c.to_node == node.id && Some(&c.to_connector) == node.inputs.first().map(|i| &i.id)
    })?;
    match &state.nodes.get(&conn.from_node)?.data {
        NodeData::GenerateDialog(d) => {
            Some((d.generation_status.clone(), d.generated_dialog.clone()))
        }
        _ => None,
    }
}

fn fmt_dur(s: f64) -> String {
    format!("{}:{:02}", (s / 60.0) as u32, (s % 60.0) as u32)
}
