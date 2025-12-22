//! Toolbox component with draggable node types.

use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{NodeData, Position, StillSamplerData, VideoInputData, ViewerData};

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
            {render_node_item("P", "Viewer", "Play video", create_viewer_callback(state))}
        </div>
    }
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

fn create_viewer_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_| {
        state.dispatch(AppAction::CreateNode(
            NodeData::Viewer(ViewerData::default()),
            Position::new(700.0, 100.0),
        ));
    })
}
