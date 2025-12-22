//! Connection rendering functions.

use yew::prelude::*;

use crate::state::{AppAction, AppStateContext};
use yt_rs_shared::{Connection, Position};

pub(super) fn render_connections(state: &AppStateContext) -> Html {
    html! {
        <>
            {for state.connections.values().map(|conn| render_connection(conn, state))}
        </>
    }
}

fn render_connection(conn: &Connection, state: &AppStateContext) -> Html {
    let (start, end) = get_connection_endpoints(conn, state);
    let path = conn.svg_path(start, end);
    let conn_id = conn.id;
    let state = state.clone();

    let on_click = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
        state.dispatch(AppAction::DeleteConnection(conn_id));
    });

    html! {
        <path
            class="connection"
            d={path}
            fill="none"
            stroke="#4a9eff"
            stroke-width="2"
            onclick={on_click}
            style="cursor: pointer;"
        />
    }
}

fn get_connection_endpoints(conn: &Connection, state: &AppStateContext) -> (Position, Position) {
    let start = state
        .nodes
        .get(&conn.from_node)
        .map(|n| Position::new(n.position.x + n.size.width, n.position.y + 60.0))
        .unwrap_or_default();
    let end = state
        .nodes
        .get(&conn.to_node)
        .map(|n| Position::new(n.position.x, n.position.y + 40.0))
        .unwrap_or_default();
    (start, end)
}

pub(super) fn render_pending_connection(state: &AppStateContext) -> Html {
    if let Some(ref pending) = state.pending_connection {
        let path = pending.svg_path();
        html! {
            <path
                class="pending-connection"
                d={path}
                fill="none"
                stroke="#4a9eff"
                stroke-width="2"
                stroke-dasharray="5,5"
            />
        }
    } else {
        html! {}
    }
}
