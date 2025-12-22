//! Node rendering components.

use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{Node, NodeData, Position, VideoInputData};

/// Renders a single node with its connectors as SVG.
pub fn render_node(node: &Node, state: &AppStateContext) -> Html {
    let node_id = node.id;
    let node_pos = node.position;
    let is_selected = state.selected_node == Some(node_id);
    let border_color = if is_selected { "#6c6cff" } else { "#3d3d4d" };

    let state_drag = state.clone();
    let on_mousedown = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
        e.prevent_default();
        let offset = Position::new(e.offset_x() as f64, e.offset_y() as f64);
        state_drag.dispatch(AppAction::StartDrag(node_id, offset));
    });

    let state_dbl = state.clone();
    let on_dblclick = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
        state_dbl.dispatch(AppAction::OpenDialog(node_id));
    });

    let node_details = render_node_details(node, state);

    html! {
        <g class="node" transform={format!("translate({}, {})", node_pos.x, node_pos.y)}>
            <rect
                width={node.size.width.to_string()}
                height={node.size.height.to_string()}
                rx="8"
                fill="#2d2d3d"
                stroke={border_color}
                stroke-width="2"
                onmousedown={on_mousedown}
                ondblclick={on_dblclick}
                style="cursor: move;"
            />
            <text x="10" y="24" fill="#fff" font-size="12" style="pointer-events: none;">{node.data.type_name()}</text>
            {node_details}
            {render_connectors(node, state)}
        </g>
    }
}

fn render_node_details(node: &Node, state: &AppStateContext) -> Html {
    match &node.data {
        NodeData::VideoInput(video) => {
            if let Some(ref name) = video.file_name {
                let duration = video
                    .duration_seconds
                    .map(|d| format!("{:.1}s", d))
                    .unwrap_or_default();
                html! {
                    <>
                        <text x="10" y="44" fill="#aaa" font-size="10" style="pointer-events: none;">{name}</text>
                        if !duration.is_empty() {
                            <text x="10" y="58" fill="#888" font-size="10" style="pointer-events: none;">{duration}</text>
                        }
                    </>
                }
            } else {
                html! {}
            }
        }
        NodeData::StillSampler(sampler) => {
            html! {
                <text x="10" y="44" fill="#aaa" font-size="10" style="pointer-events: none;">
                    {format!("{}s interval", sampler.interval_seconds)}
                </text>
            }
        }
        NodeData::Viewer(_) => {
            if let Some(video) = find_connected_video(node, state) {
                let name = video.file_name.unwrap_or_else(|| "Unknown".to_string());
                let duration = video
                    .duration_seconds
                    .map(|d| format!("{:.1}s", d))
                    .unwrap_or_default();
                html! {
                    <>
                        <text x="10" y="44" fill="#aaa" font-size="10" style="pointer-events: none;">{name}</text>
                        if !duration.is_empty() {
                            <text x="10" y="58" fill="#888" font-size="10" style="pointer-events: none;">{duration}</text>
                        }
                    </>
                }
            } else {
                html! {
                    <text x="10" y="44" fill="#666" font-size="10" style="pointer-events: none;">
                        {"No video connected"}
                    </text>
                }
            }
        }
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

fn render_connectors(node: &Node, state: &AppStateContext) -> Html {
    let node_id = node.id;

    let output = node.outputs.first().map(|conn| {
        let conn_id = conn.id;
        let state_conn = state.clone();
        let start_pos = Position::new(node.position.x + node.size.width, node.position.y + 60.0);
        let on_click = Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            state_conn.dispatch(AppAction::StartConnection(node_id, conn_id, start_pos));
        });
        html! {
            <circle cx={node.size.width.to_string()} cy="60" r="6" fill="#4a9eff" onclick={on_click} style="cursor: crosshair;" />
        }
    });

    let input = node.inputs.first().map(|conn| {
        let conn_id = conn.id;
        let state_conn = state.clone();
        let on_click = Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            state_conn.dispatch(AppAction::CompleteConnection(node_id, conn_id));
        });
        html! {
            <circle cx="0" cy="40" r="6" fill="#ff6b6b" onclick={on_click} style="cursor: crosshair;" />
        }
    });

    html! { <>{output}{input}</> }
}
