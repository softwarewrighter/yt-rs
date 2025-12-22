//! Node dialog components.

use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{Node, NodeData};

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

fn render_video_input_dialog(
    _node: &Node,
    data: &yt_rs_shared::VideoInputData,
    _state: &AppStateContext,
) -> Html {
    let has_video = data.file_name.is_some();
    let video_name = data.file_name.clone().unwrap_or_else(|| "None".to_string());
    let duration = data
        .duration_seconds
        .map(|d| format!("{:.1}s", d))
        .unwrap_or_default();

    let on_file_change = Callback::from(|e: Event| {
        use wasm_bindgen::JsCast;
        let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
        if let Some(files) = input.files()
            && let Some(file) = files.get(0)
        {
            log::info!("Selected file: {}", file.name());
            // TODO: Upload file to backend
        }
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
            <div class="dialog-actions">
                <button disabled={!has_video}>{"Delete Video"}</button>
                <label class="file-button">
                    {"Load Video..."}
                    <input type="file" accept="video/*" onchange={on_file_change} style="display: none;" />
                </label>
            </div>
        </div>
    }
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

fn find_connected_video(
    node: &Node,
    state: &AppStateContext,
) -> Option<yt_rs_shared::VideoInputData> {
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
