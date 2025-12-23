//! Node rendering components.

use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{ConnectorPosition, GenerationStatus, Node, NodeData, Position, VideoInputData};

// === Public API ===

/// Renders a single node with its connectors as SVG.
pub fn render_node(node: &Node, state: &AppStateContext) -> Html {
    let id = node.id;
    let pos = node.position;
    let border = if state.selected_node == Some(id) {
        "#6c6cff"
    } else {
        "#3d3d4d"
    };

    let on_down = {
        let s = state.clone();
        Callback::from(move |e: MouseEvent| handle_drag_start(&s, id, pos, &e))
    };
    let on_dbl = {
        let s = state.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            s.dispatch(AppAction::OpenDialog(id));
        })
    };

    html! {
        <g class="node" transform={format!("translate({}, {})", pos.x, pos.y)}>
            <rect width={node.size.width.to_string()} height={node.size.height.to_string()} rx="8"
                  fill="#2d2d3d" stroke={border} stroke-width="2" onmousedown={on_down} ondblclick={on_dbl} style="cursor: move;" />
            {txt(10.0, 24.0, node.data.type_name(), "#fff", 12)}
            {render_details(node, state)}
            {render_connectors(node, state)}
        </g>
    }
}

// === Node Details Rendering ===

fn render_details(node: &Node, state: &AppStateContext) -> Html {
    match &node.data {
        NodeData::VideoInput(v) => render_video_info(v),
        NodeData::StillSampler(s) => two_line(
            format!("{}s interval", s.interval_seconds),
            stills_text(s.extracted_stills.len()),
        ),
        NodeData::Viewer(_) => render_viewer_info(find_connected_video(node, state)),
        NodeData::Selector(d) => two_line(
            format!("Index: {}", d.selected_index),
            stills_text(find_stills_count(node, state)),
        ),
        NodeData::StillPreview(_) => render_still_preview(find_still_info(node, state)),
        NodeData::GenerateDialog(d) => two_line(
            stills_text(find_stills_count(node, state)),
            status_text(&d.generation_status),
        ),
        NodeData::TextView(_) => txt(
            10.0,
            44.0,
            if find_text(node, state).is_some() {
                "Text available"
            } else {
                "No text connected"
            },
            "#888",
            10,
        ),
    }
}

fn render_video_info(v: &VideoInputData) -> Html {
    v.file_name
        .as_ref()
        .map(|n| {
            two_line(
                n.clone(),
                v.duration_seconds
                    .map(|d| format!("{:.1}s", d))
                    .unwrap_or_default(),
            )
        })
        .unwrap_or_default()
}

fn render_viewer_info(v: Option<VideoInputData>) -> Html {
    v.map(|v| {
        let name = v.file_name.unwrap_or("Unknown".into());
        let dur = v.duration_seconds.map(|d| format!("{:.1}s", d)).unwrap_or_default();
        if let Some(id) = v.file_id {
            html! {
                <>
                    {txt(10.0, 44.0, &name, "#aaa", 10)}
                    {if !dur.is_empty() { txt(10.0, 58.0, dur, "#888", 10) } else { html! {} }}
                    <image x="10" y="65" width="180" height="90" href={format!("/api/v1/videos/{id}/thumbnail")} style="pointer-events: none;" />
                </>
            }
        } else {
            two_line(name, dur)
        }
    })
    .unwrap_or_else(|| txt(10.0, 44.0, "No video connected", "#666", 10))
}

fn render_still_preview(info: Option<(uuid::Uuid, f64)>) -> Html {
    info.map(|(vid, ts)| html! {
        <>{txt(10.0, 44.0, format!("{:.1}s", ts), "#888", 10)}<image x="10" y="50" width="180" height="100" href={format!("/api/v1/stills/{}/{:.2}", vid, ts)} style="pointer-events: none;" /></>
    }).unwrap_or_else(|| txt(10.0, 44.0, "No still connected", "#666", 10))
}

fn stills_text(count: usize) -> String {
    if count > 0 {
        format!("{} stills", count)
    } else {
        "No input".into()
    }
}
fn status_text(s: &GenerationStatus) -> String {
    match s {
        GenerationStatus::Idle => "Ready",
        GenerationStatus::Complete => "Complete",
        GenerationStatus::Error(_) => "Error",
        GenerationStatus::Generating {
            current_still,
            total_stills,
        } => return format!("{}/{}", current_still, total_stills),
    }
    .into()
}

// === SVG Helpers ===

fn txt<S: Into<String>>(x: f64, y: f64, text: S, fill: &str, size: u8) -> Html {
    let fill = fill.to_string();
    html! { <text x={x.to_string()} y={y.to_string()} {fill} font-size={size.to_string()} style="pointer-events: none;">{text.into()}</text> }
}

fn two_line<S: Into<String>>(line1: S, line2: String) -> Html {
    let l1 = line1.into();
    html! { <>{txt(10.0, 44.0, l1, "#aaa", 10)}{if !line2.is_empty() { txt(10.0, 58.0, line2, "#888", 10) } else { html! {} }}</> }
}

// === Connectors ===

fn render_connectors(node: &Node, state: &AppStateContext) -> Html {
    let (id, w, pos) = (node.id, node.size.width, node.position);
    let outs: Html = node
        .outputs
        .iter()
        .map(|c| render_conn(c.id, id, w, pos, y_of(&c.position), true, state))
        .collect();
    let ins: Html = node
        .inputs
        .iter()
        .map(|c| render_conn(c.id, id, w, pos, y_of(&c.position), false, state))
        .collect();
    html! { <>{outs}{ins}</> }
}

fn render_conn(
    cid: uuid::Uuid,
    nid: uuid::Uuid,
    w: f64,
    npos: Position,
    y: f64,
    is_out: bool,
    state: &AppStateContext,
) -> Html {
    let (cx, fill) = if is_out {
        (w, "#4a9eff")
    } else {
        (0.0, "#ff6b6b")
    };
    let s = state.clone();
    let cb = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
        if is_out {
            s.dispatch(AppAction::StartConnection(
                nid,
                cid,
                Position::new(npos.x + w, npos.y + y),
            ));
        } else {
            s.dispatch(AppAction::CompleteConnection(nid, cid));
        }
    });
    html! { <circle cx={cx.to_string()} cy={y.to_string()} r="6" {fill} onclick={cb} style="cursor: crosshair;" /> }
}

fn y_of(p: &ConnectorPosition) -> f64 {
    match p {
        ConnectorPosition::Left(y) | ConnectorPosition::Right(y) => *y,
    }
}

// === Event Handlers ===

fn handle_drag_start(state: &AppStateContext, id: uuid::Uuid, npos: Position, e: &MouseEvent) {
    e.stop_propagation();
    e.prevent_default();
    focus_canvas();
    if let Some(sp) = svg_pos(e) {
        let cp = state.canvas.screen_to_canvas(sp);
        state.dispatch(AppAction::StartDrag(
            id,
            Position::new(cp.x - npos.x, cp.y - npos.y),
        ));
    }
}

fn focus_canvas() {
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.query_selector(".canvas-container").ok().flatten())
    {
        let _ = el.dyn_ref::<web_sys::HtmlElement>().map(|h| h.focus());
    }
}

fn svg_pos(e: &MouseEvent) -> Option<Position> {
    let svg = web_sys::window()?
        .document()?
        .query_selector("svg.canvas")
        .ok()??;
    let r = svg.get_bounding_client_rect();
    Some(Position::new(
        e.client_x() as f64 - r.left(),
        e.client_y() as f64 - r.top(),
    ))
}

// === Graph Traversal ===

fn find_connected_video(node: &Node, state: &AppStateContext) -> Option<VideoInputData> {
    let c = find_input_conn(node, state)?;
    match &state.nodes.get(&c.from_node)?.data {
        NodeData::VideoInput(d) => Some(d.clone()),
        _ => None,
    }
}

fn find_stills_count(node: &Node, state: &AppStateContext) -> usize {
    find_input_conn(node, state)
        .and_then(|c| state.nodes.get(&c.from_node))
        .map(|n| match &n.data {
            NodeData::StillSampler(d) => d.extracted_stills.len(),
            NodeData::Selector(_) => find_stills_count(n, state),
            _ => 0,
        })
        .unwrap_or(0)
}

fn find_still_info(node: &Node, state: &AppStateContext) -> Option<(uuid::Uuid, f64)> {
    let sel = state.nodes.get(&find_input_conn(node, state)?.from_node)?;
    let idx = match &sel.data {
        NodeData::Selector(d) => d.selected_index,
        _ => return None,
    };
    let stills = find_stills_from(sel, state)?;
    Some((
        find_video_id(sel, state)?,
        stills.get(idx)?.timestamp_seconds,
    ))
}

fn find_stills_from(node: &Node, state: &AppStateContext) -> Option<Vec<yt_rs_shared::Still>> {
    let src = state.nodes.get(&find_input_conn(node, state)?.from_node)?;
    match &src.data {
        NodeData::StillSampler(d) => Some(d.extracted_stills.clone()),
        NodeData::Selector(_) => find_stills_from(src, state),
        _ => None,
    }
}

fn find_video_id(node: &Node, state: &AppStateContext) -> Option<uuid::Uuid> {
    node.inputs.iter().find_map(|i| {
        let up = state.nodes.get(
            &state
                .connections
                .values()
                .find(|c| c.to_node == node.id && c.to_connector == i.id)?
                .from_node,
        )?;
        match &up.data {
            NodeData::VideoInput(d) => d.file_id,
            _ => find_video_id(up, state),
        }
    })
}

fn find_text(node: &Node, state: &AppStateContext) -> Option<yt_rs_shared::GeneratedDialog> {
    match &state
        .nodes
        .get(&find_input_conn(node, state)?.from_node)?
        .data
    {
        NodeData::GenerateDialog(d) => d.generated_dialog.clone(),
        _ => None,
    }
}

fn find_input_conn<'a>(
    node: &Node,
    state: &'a AppStateContext,
) -> Option<&'a yt_rs_shared::Connection> {
    let id = node.inputs.first()?.id;
    state
        .connections
        .values()
        .find(|c| c.to_node == node.id && c.to_connector == id)
}
