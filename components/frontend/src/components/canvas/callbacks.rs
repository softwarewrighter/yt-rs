//! Canvas callback creation functions.

use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::Position;

pub(super) struct CanvasCallbacks {
    pub view_box: String,
    pub transform: String,
    pub on_wheel: Callback<WheelEvent>,
    pub on_mouse_move: Callback<MouseEvent>,
    pub on_mouse_up: Callback<MouseEvent>,
    pub on_click: Callback<MouseEvent>,
    pub on_key_down: Callback<KeyboardEvent>,
}

pub(super) fn create_canvas_callbacks(state: &AppStateContext) -> CanvasCallbacks {
    let view_box = format!(
        "0 0 {} {}",
        state.canvas.viewport.width, state.canvas.viewport.height
    );
    let transform = format!(
        "translate({}, {}) scale({})",
        state.canvas.pan_offset.x, state.canvas.pan_offset.y, state.canvas.zoom
    );

    CanvasCallbacks {
        view_box,
        transform,
        on_wheel: create_wheel_callback(state),
        on_mouse_move: create_mouse_move_callback(state),
        on_mouse_up: create_mouse_up_callback(state),
        on_click: create_click_callback(state),
        on_key_down: create_key_down_callback(state),
    }
}

fn create_wheel_callback(state: &AppStateContext) -> Callback<WheelEvent> {
    let state = state.clone();
    Callback::from(move |e: WheelEvent| {
        e.prevent_default();
        let delta = e.delta_y();
        let current_zoom = state.canvas.zoom;
        let new_zoom = if delta > 0.0 {
            (current_zoom * 0.9).max(0.25)
        } else {
            (current_zoom * 1.1).min(4.0)
        };
        state.dispatch(AppAction::SetZoom(new_zoom));
    })
}

fn create_mouse_move_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |e: MouseEvent| {
        if state.dragging.is_some() {
            let pos = Position::new(e.offset_x() as f64, e.offset_y() as f64);
            let canvas_pos = state.canvas.screen_to_canvas(pos);
            state.dispatch(AppAction::UpdateDrag(canvas_pos));
        } else if e.buttons() == 4 {
            let dx = e.movement_x() as f64;
            let dy = e.movement_y() as f64;
            let new_pan = Position::new(
                state.canvas.pan_offset.x + dx,
                state.canvas.pan_offset.y + dy,
            );
            state.dispatch(AppAction::SetPan(new_pan));
        } else if state.pending_connection.is_some() {
            let pos = Position::new(e.offset_x() as f64, e.offset_y() as f64);
            let canvas_pos = state.canvas.screen_to_canvas(pos);
            state.dispatch(AppAction::UpdatePendingConnection(canvas_pos));
        }
    })
}

fn create_mouse_up_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_e: MouseEvent| {
        if state.dragging.is_some() {
            state.dispatch(AppAction::EndDrag);
        }
    })
}

fn create_click_callback(state: &AppStateContext) -> Callback<MouseEvent> {
    let state = state.clone();
    Callback::from(move |_e: MouseEvent| {
        if state.just_dragged {
            state.dispatch(AppAction::ClearJustDragged);
        } else if state.pending_connection.is_some() {
            state.dispatch(AppAction::CancelConnection);
        } else {
            state.dispatch(AppAction::SelectNode(None));
        }
    })
}

fn create_key_down_callback(state: &AppStateContext) -> Callback<KeyboardEvent> {
    let state = state.clone();
    Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
        "Escape" => state.dispatch(AppAction::CancelConnection),
        "Delete" | "Backspace" => {
            if let Some(node_id) = state.selected_node {
                state.dispatch(AppAction::DeleteNode(node_id));
            }
        }
        "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => {
            if let Some(node_id) = state.selected_node
                && let Some(node) = state.nodes.get(&node_id)
            {
                let step = if e.shift_key() { 10.0 } else { 1.0 };
                let (dx, dy) = match e.key().as_str() {
                    "ArrowUp" => (0.0, -step),
                    "ArrowDown" => (0.0, step),
                    "ArrowLeft" => (-step, 0.0),
                    "ArrowRight" => (step, 0.0),
                    _ => (0.0, 0.0),
                };
                let new_pos = Position::new(node.position.x + dx, node.position.y + dy);
                state.dispatch(AppAction::MoveNode(node_id, new_pos));
                e.prevent_default();
            }
        }
        _ => {}
    })
}
