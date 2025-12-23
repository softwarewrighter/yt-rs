//! Application state management using Yew's use_reducer.

use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;
use yew::prelude::*;
use yt_rs_shared::{CanvasState, Connection, Node, NodeData, PendingConnection, Position};

/// Drag operation state.
#[derive(Debug, Clone, PartialEq)]
pub struct DragState {
    pub node_id: Uuid,
    pub offset: Position,
}

/// The application state.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppState {
    pub canvas: CanvasState,
    pub nodes: HashMap<Uuid, Node>,
    pub connections: HashMap<Uuid, Connection>,
    pub selected_node: Option<Uuid>,
    pub pending_connection: Option<PendingConnection>,
    pub dragging: Option<DragState>,
    pub just_dragged: bool,
    pub open_dialog: Option<Uuid>,
}

/// Actions that can be dispatched to modify the state.
#[derive(Debug, Clone)]
pub enum AppAction {
    SetPan(Position),
    SetZoom(f64),
    CreateNode(NodeData, Position),
    MoveNode(Uuid, Position),
    UpdateNodeData(Uuid, NodeData),
    DeleteNode(Uuid),
    SelectNode(Option<Uuid>),
    StartDrag(Uuid, Position),
    UpdateDrag(Position),
    EndDrag,
    ClearJustDragged,
    OpenDialog(Uuid),
    CloseDialog,
    StartConnection(Uuid, Uuid, Position),
    UpdatePendingConnection(Position),
    CompleteConnection(Uuid, Uuid),
    CancelConnection,
    DeleteConnection(Uuid),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();
        state.apply(action);
        Rc::new(state)
    }
}

impl AppState {
    fn apply(&mut self, action: AppAction) {
        use AppAction::*;
        match action {
            SetPan(pos) => self.canvas.pan_offset = pos,
            SetZoom(zoom) => self.canvas.zoom = zoom.clamp(0.25, 4.0),
            CreateNode(data, pos) => self.create_node(data, pos),
            MoveNode(id, pos) => {
                if let Some(n) = self.nodes.get_mut(&id) {
                    n.position = pos;
                }
            }
            UpdateNodeData(id, data) => {
                if let Some(n) = self.nodes.get_mut(&id) {
                    n.data = data;
                }
            }
            DeleteNode(id) => self.delete_node(id),
            SelectNode(id) => self.selected_node = id,
            StartDrag(id, off) => {
                self.dragging = Some(DragState {
                    node_id: id,
                    offset: off,
                });
                self.selected_node = Some(id);
            }
            UpdateDrag(pos) => self.update_drag(pos),
            EndDrag => {
                self.dragging = None;
                self.just_dragged = true;
            }
            ClearJustDragged => self.just_dragged = false,
            OpenDialog(id) => {
                self.open_dialog = Some(id);
                self.dragging = None;
            }
            CloseDialog => self.open_dialog = None,
            StartConnection(n, c, p) => {
                self.pending_connection = Some(PendingConnection::new(n, c, p))
            }
            UpdatePendingConnection(pos) => {
                if let Some(p) = &mut self.pending_connection {
                    p.current_position = pos;
                }
            }
            CompleteConnection(n, c) => self.complete_connection(n, c),
            CancelConnection => self.pending_connection = None,
            DeleteConnection(id) => {
                self.connections.remove(&id);
            }
        }
    }

    fn create_node(&mut self, data: NodeData, pos: Position) {
        let node = match data {
            NodeData::VideoInput(_) => Node::new_video_input(pos),
            NodeData::StillSampler(_) => Node::new_still_sampler(pos),
            NodeData::Viewer(_) => Node::new_viewer(pos),
            NodeData::Selector(_) => Node::new_selector(pos),
            NodeData::StillPreview(_) => Node::new_still_preview(pos),
            NodeData::GenerateDialog(_) => Node::new_generate_dialog(pos),
            NodeData::TextView(_) => Node::new_text_view(pos),
        };
        self.nodes.insert(node.id, node);
    }

    fn delete_node(&mut self, id: Uuid) {
        self.nodes.remove(&id);
        self.connections
            .retain(|_, c| c.from_node != id && c.to_node != id);
        if self.selected_node == Some(id) {
            self.selected_node = None;
        }
    }

    fn update_drag(&mut self, mouse_pos: Position) {
        if let Some(ref drag) = self.dragging {
            let new_pos = Position::new(mouse_pos.x - drag.offset.x, mouse_pos.y - drag.offset.y);
            if let Some(node) = self.nodes.get_mut(&drag.node_id) {
                node.position = new_pos;
            }
        }
    }

    fn complete_connection(&mut self, to_node: Uuid, to_conn: Uuid) {
        if let Some(pending) = self.pending_connection.take() {
            let conn = Connection::new(pending.from_node, pending.from_connector, to_node, to_conn);
            self.connections.insert(conn.id, conn);

            // If connecting to a StillSampler, generate stills from the source video
            self.update_stills_on_connection(pending.from_node, to_node);
        }
    }

    fn update_stills_on_connection(&mut self, from_node: Uuid, to_node: Uuid) {
        // Get duration from source VideoInput
        let duration = self.nodes.get(&from_node).and_then(|n| match &n.data {
            NodeData::VideoInput(data) => data.duration_seconds,
            _ => None,
        });

        // If target is StillSampler, generate stills
        if let Some(duration) = duration
            && let Some(target) = self.nodes.get_mut(&to_node)
            && let NodeData::StillSampler(ref mut data) = target.data
        {
            data.generate_stills(duration);
        }
    }
}

/// Context type for the application state.
pub type AppStateContext = UseReducerHandle<AppState>;

/// Properties for the state provider.
#[derive(Properties, PartialEq)]
pub struct AppStateProviderProps {
    pub children: Children,
}

/// Provider component for the application state.
#[function_component(AppStateProvider)]
pub fn app_state_provider(props: &AppStateProviderProps) -> Html {
    let state = use_reducer(AppState::default);

    html! {
        <ContextProvider<AppStateContext> context={state}>
            {props.children.clone()}
        </ContextProvider<AppStateContext>>
    }
}
