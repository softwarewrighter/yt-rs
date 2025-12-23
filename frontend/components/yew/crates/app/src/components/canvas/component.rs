//! Canvas component implementation.

use yew::prelude::*;

use super::super::nodes::render_node;
use super::callbacks::{CanvasCallbacks, create_canvas_callbacks};
use super::connections::{render_connections, render_pending_connection};
use crate::state::AppStateContext;

/// The main canvas component where nodes are rendered.
#[function_component(Canvas)]
pub fn canvas() -> Html {
    let state = use_context::<AppStateContext>().expect("AppStateContext not found");
    log::info!(
        "Canvas render: {} nodes, {} connections",
        state.nodes.len(),
        state.connections.len()
    );
    let callbacks = create_canvas_callbacks(&state);
    let on_key_down = callbacks.on_key_down.clone();

    html! {
        <div class="canvas-container" tabindex="0" onkeydown={on_key_down}>
            {render_svg(&state, &callbacks)}
        </div>
    }
}

fn render_svg(state: &AppStateContext, callbacks: &CanvasCallbacks) -> Html {
    html! {
        <svg
            class="canvas"
            viewBox={callbacks.view_box.clone()}
            onwheel={callbacks.on_wheel.clone()}
            onmousemove={callbacks.on_mouse_move.clone()}
            onmouseup={callbacks.on_mouse_up.clone()}
            onclick={callbacks.on_click.clone()}
        >
            {render_defs()}
            <g transform={callbacks.transform.clone()}>
                {render_grid()}
                <g class="connections">{render_connections(state)}</g>
                <g class="nodes">{render_nodes(state)}</g>
                {render_pending_connection(state)}
            </g>
        </svg>
    }
}

fn render_defs() -> Html {
    html! {
        <defs>
            <pattern id="grid" width="20" height="20" patternUnits="userSpaceOnUse">
                <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#2a2a3e" stroke-width="0.5"/>
            </pattern>
        </defs>
    }
}

fn render_grid() -> Html {
    html! { <rect width="10000" height="10000" x="-5000" y="-5000" fill="url(#grid)" /> }
}

fn render_nodes(state: &AppStateContext) -> Html {
    html! { <>{for state.nodes.values().map(|n| render_node(n, state))}</> }
}
