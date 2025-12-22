//! Node rendering components.

use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{ConnectorPosition, Node, NodeData, Position, VideoInputData};

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
        NodeData::Selector(data) => {
            let stills_count = find_connected_stills_count(node, state);
            html! {
                <>
                    <text x="10" y="44" fill="#aaa" font-size="10" style="pointer-events: none;">
                        {format!("Index: {}", data.selected_index)}
                    </text>
                    <text x="10" y="58" fill="#888" font-size="10" style="pointer-events: none;">
                        {if stills_count > 0 {
                            format!("{} stills", stills_count)
                        } else {
                            "No input".to_string()
                        }}
                    </text>
                </>
            }
        }
        NodeData::StillPreview(_) => {
            if let Some((video_id, timestamp)) = find_connected_still_info(node, state) {
                let url = format!("/api/v1/stills/{}/{:.2}", video_id, timestamp);
                html! {
                    <>
                        <text x="10" y="44" fill="#888" font-size="10" style="pointer-events: none;">
                            {format!("{:.1}s", timestamp)}
                        </text>
                        <image
                            x="10"
                            y="50"
                            width="180"
                            height="100"
                            href={url}
                            style="pointer-events: none;"
                        />
                    </>
                }
            } else {
                html! {
                    <text x="10" y="44" fill="#666" font-size="10" style="pointer-events: none;">
                        {"No still connected"}
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

fn find_connected_stills_count(node: &Node, state: &AppStateContext) -> usize {
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
        NodeData::Selector(_) => {
            // Recursively find stills count from upstream
            find_connected_stills_count(source_node, state)
        }
        _ => 0,
    }
}

fn find_connected_still_info(node: &Node, state: &AppStateContext) -> Option<(uuid::Uuid, f64)> {
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
    let video_id = find_video_id_upstream(selector_node, state)?;

    Some((video_id, still.timestamp_seconds))
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

fn find_video_id_upstream(node: &Node, state: &AppStateContext) -> Option<uuid::Uuid> {
    for input in &node.inputs {
        let connection = state
            .connections
            .values()
            .find(|c| c.to_node == node.id && c.to_connector == input.id);
        if let Some(conn) = connection {
            let upstream = state.nodes.get(&conn.from_node)?;
            match &upstream.data {
                NodeData::VideoInput(data) => return data.file_id,
                _ => {
                    if let Some(id) = find_video_id_upstream(upstream, state) {
                        return Some(id);
                    }
                }
            }
        }
    }
    None
}

fn render_connectors(node: &Node, state: &AppStateContext) -> Html {
    let node_id = node.id;
    let node_width = node.size.width;
    let node_position = node.position;

    // Render all outputs
    let outputs: Html = node
        .outputs
        .iter()
        .map(|conn| {
            let conn_id = conn.id;
            let y_offset = match conn.position {
                ConnectorPosition::Right(y) => y,
                ConnectorPosition::Left(y) => y,
            };
            let state_conn = state.clone();
            let start_pos = Position::new(node_position.x + node_width, node_position.y + y_offset);
            let on_click = Callback::from(move |e: MouseEvent| {
                e.stop_propagation();
                state_conn.dispatch(AppAction::StartConnection(node_id, conn_id, start_pos));
            });
            html! {
                <circle
                    cx={node_width.to_string()}
                    cy={y_offset.to_string()}
                    r="6"
                    fill="#4a9eff"
                    onclick={on_click}
                    style="cursor: crosshair;"
                />
            }
        })
        .collect();

    // Render all inputs
    let inputs: Html = node
        .inputs
        .iter()
        .map(|conn| {
            let conn_id = conn.id;
            let y_offset = match conn.position {
                ConnectorPosition::Left(y) => y,
                ConnectorPosition::Right(y) => y,
            };
            let state_conn = state.clone();
            let on_click = Callback::from(move |e: MouseEvent| {
                e.stop_propagation();
                state_conn.dispatch(AppAction::CompleteConnection(node_id, conn_id));
            });
            html! {
                <circle
                    cx="0"
                    cy={y_offset.to_string()}
                    r="6"
                    fill="#ff6b6b"
                    onclick={on_click}
                    style="cursor: crosshair;"
                />
            }
        })
        .collect();

    html! { <>{outputs}{inputs}</> }
}
