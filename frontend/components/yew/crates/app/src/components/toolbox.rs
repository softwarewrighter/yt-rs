//! Toolbox component with draggable node types.

use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{
    GenerateDialogData, NodeData, Position, Project, SelectorData, StillPreviewData,
    StillSamplerData, TextViewData, VideoInputData, ViewerData,
};

/// Response wrapper for project API.
#[derive(Debug, Deserialize)]
struct ProjectResponse {
    project: Project,
}

/// The toolbox sidebar component.
#[function_component(Toolbox)]
pub fn toolbox() -> Html {
    let collapsed = use_state(|| false);
    let state = use_context::<AppStateContext>().expect("AppStateContext not found");
    let toggle = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| collapsed.set(!*collapsed))
    };

    html! {
        <div class={if *collapsed { "toolbox collapsed" } else { "toolbox" }}>
            {render_header(*collapsed, toggle)}
            if !*collapsed { {render_palette(&state)} }
        </div>
    }
}

fn render_header(collapsed: bool, on_toggle: Callback<MouseEvent>) -> Html {
    html! {
        <div class="toolbox-header" onclick={on_toggle}>
            <span class="toggle-icon">{if collapsed { ">" } else { "<" }}</span>
            if !collapsed { <span class="title">{"Nodes"}</span> }
        </div>
    }
}

fn render_palette(state: &AppStateContext) -> Html {
    html! {
        <div class="toolbox-content">
            {render_node_item("V", "Video Input", "Load a video file", create_video_callback(state))}
            {render_node_item("S", "Still Sampler", "Extract frames", create_sampler_callback(state))}
            {render_node_item("I", "Selector", "Select a still", create_selector_callback(state))}
            {render_node_item("T", "Still Preview", "View still image", create_preview_callback(state))}
            {render_node_item("P", "Viewer", "Play video", create_viewer_callback(state))}
            {render_node_item("G", "Generate Dialog", "AI dialog generation", create_generate_callback(state))}
            {render_node_item("X", "Text View", "Display text output", create_text_view_callback(state))}
            <div class="toolbox-divider" />
            {render_workspace_buttons(state)}
        </div>
    }
}

fn render_workspace_buttons(state: &AppStateContext) -> Html {
    html! {
        <div class="workspace-buttons">
            {render_action_button("Save", "Save workspace", create_save_callback(state))}
            {render_action_button("Restore", "Load workspace", create_restore_callback(state))}
        </div>
    }
}

fn render_action_button(label: &str, title: &str, on_click: Callback<MouseEvent>) -> Html {
    let title = title.to_string();
    html! {
        <button class="action-button" {title} onclick={on_click}>{label}</button>
    }
}

fn create_save_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        let project = state.to_project("default");
        spawn_local(async move {
            let _ = Request::post("/api/v1/workspace/save")
                .json(&project)
                .expect("serialize project")
                .send()
                .await;
        });
    })
}

fn create_restore_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        let state = state.clone();
        spawn_local(async move {
            if let Ok(resp) = Request::get("/api/v1/workspace/restore").send().await
                && resp.ok()
                && let Ok(data) = resp.json::<ProjectResponse>().await
            {
                state.dispatch(AppAction::LoadProject(data.project));
            }
        });
    })
}

fn render_node_item(icon: &str, name: &str, desc: &str, on_click: Callback<MouseEvent>) -> Html {
    html! {
        <div class="node-item" onclick={on_click}>
            <div class="node-icon">{icon}</div>
            <div class="node-info">
                <div class="node-name">{name}</div>
                <div class="node-desc">{desc}</div>
            </div>
        </div>
    }
}

fn create_video_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        log::info!("Creating video node");
        state.dispatch(AppAction::CreateNode(
            NodeData::VideoInput(VideoInputData::default()),
            Position::new(100.0, 100.0),
        ));
    })
}

fn create_sampler_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        state.dispatch(AppAction::CreateNode(
            NodeData::StillSampler(StillSamplerData::default()),
            Position::new(400.0, 100.0),
        ));
    })
}

fn create_selector_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        state.dispatch(AppAction::CreateNode(
            NodeData::Selector(SelectorData::default()),
            Position::new(550.0, 100.0),
        ));
    })
}

fn create_preview_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        state.dispatch(AppAction::CreateNode(
            NodeData::StillPreview(StillPreviewData::default()),
            Position::new(750.0, 100.0),
        ));
    })
}

fn create_viewer_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        state.dispatch(AppAction::CreateNode(
            NodeData::Viewer(ViewerData::default()),
            Position::new(700.0, 100.0),
        ));
    })
}

fn create_generate_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        state.dispatch(AppAction::CreateNode(
            NodeData::GenerateDialog(GenerateDialogData::default()),
            Position::new(700.0, 250.0),
        ));
    })
}

fn create_text_view_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        state.dispatch(AppAction::CreateNode(
            NodeData::TextView(TextViewData::default()),
            Position::new(950.0, 250.0),
        ));
    })
}
