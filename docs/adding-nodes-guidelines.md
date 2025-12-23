# Adding New Node Types

This guide documents how to add new node types to the yt-rs node editor system.

## Overview

Adding a new node type requires changes across multiple components:

1. **Models** (`components/models/crates/nodes/`) - Data structures
2. **Shared** (`components/shared/`) - Re-exports for cross-component use
3. **Frontend** (`components/frontend/`) - UI rendering and dialogs
4. **Backend** (`components/cli/`) - API endpoints (if needed)

## Step 1: Define the Node Data Type

### Location: `components/models/crates/nodes/src/lib.rs`

### 1.1 Create the Data Struct

```rust
/// Data specific to a MyNewNode node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MyNewNodeData {
    pub some_field: String,
    pub some_value: u32,
}
```

**Guidelines:**
- Use `#[derive(Default)]` when all fields have sensible defaults
- Include `Serialize, Deserialize` for JSON persistence
- Include `PartialEq` for state comparison

### 1.2 Add to NodeData Enum

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeData {
    VideoInput(VideoInputData),
    StillSampler(StillSamplerData),
    Viewer(ViewerData),
    Selector(SelectorData),
    StillPreview(StillPreviewData),
    MyNewNode(MyNewNodeData),  // Add here
}
```

### 1.3 Add type_name() Match Arm

```rust
impl NodeData {
    pub fn type_name(&self) -> &'static str {
        match self {
            // ... existing matches
            NodeData::MyNewNode(_) => "My New Node",
        }
    }
}
```

### 1.4 Add Node Constructor

```rust
impl Node {
    /// Creates a new MyNewNode at the given position.
    pub fn new_my_new_node(position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            size: Size::new(200.0, 120.0),  // Adjust as needed
            data: NodeData::MyNewNode(MyNewNodeData::default()),
            inputs: vec![Connector::input("input_name", 60.0)],
            outputs: vec![Connector::output("output_name", 60.0)],
            z_index: 0,
        }
    }
}
```

**Connector Guidelines:**
- Input connectors appear on the left side of the node
- Output connectors appear on the right side
- The second parameter is the Y offset from the top of the node
- Use descriptive names like `video_in`, `stills_out`, `selected_out`

## Step 2: Write Tests (TDD)

### Location: `components/models/crates/nodes/tests/my_new_node_tests.rs`

```rust
//! Tests for MyNewNode node type.

use yt_rs_nodes::{ConnectorType, Node, NodeData, Position, MyNewNodeData};

#[test]
fn test_my_new_node_data_default() {
    let data = MyNewNodeData::default();
    assert_eq!(data.some_field, "");
    assert_eq!(data.some_value, 0);
}

#[test]
fn test_node_new_my_new_node_creates_node() {
    let pos = Position::new(100.0, 200.0);
    let node = Node::new_my_new_node(pos);

    assert_eq!(node.position.x, 100.0);
    assert_eq!(node.position.y, 200.0);
    assert!(matches!(node.data, NodeData::MyNewNode(_)));
}

#[test]
fn test_node_new_my_new_node_has_correct_connectors() {
    let node = Node::new_my_new_node(Position::new(0.0, 0.0));

    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.outputs.len(), 1);
}

#[test]
fn test_my_new_node_type_name() {
    let data = NodeData::MyNewNode(MyNewNodeData::default());
    assert_eq!(data.type_name(), "My New Node");
}

#[test]
fn test_my_new_node_serialization() {
    let node_data = NodeData::MyNewNode(MyNewNodeData::default());

    let json = serde_json::to_string(&node_data).expect("serialize");
    assert!(json.contains("\"type\":\"MyNewNode\""));

    let parsed: NodeData = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(parsed, NodeData::MyNewNode(_)));
}
```

## Step 3: Update Shared Exports

### Location: `components/shared/src/lib.rs`

Add the new type to the re-exports:

```rust
pub use yt_rs_nodes::{
    // ... existing exports
    MyNewNodeData,
};
```

## Step 4: Update Frontend State

### Location: `components/frontend/src/state.rs`

Add a match arm in `create_node()`:

```rust
fn create_node(&mut self, data: NodeData, pos: Position) {
    let node = match data {
        NodeData::VideoInput(_) => Node::new_video_input(pos),
        NodeData::StillSampler(_) => Node::new_still_sampler(pos),
        NodeData::Viewer(_) => Node::new_viewer(pos),
        NodeData::Selector(_) => Node::new_selector(pos),
        NodeData::StillPreview(_) => Node::new_still_preview(pos),
        NodeData::MyNewNode(_) => Node::new_my_new_node(pos),  // Add here
    };
    self.nodes.insert(node.id, node);
}
```

## Step 5: Add to Toolbox

### Location: `components/frontend/src/components/toolbox.rs`

Add a new palette item:

```rust
let my_new_node = {
    let state = state.clone();
    Callback::from(move |_| {
        let pos = Position::new(300.0, 300.0);
        state.dispatch(AppAction::CreateNode(
            NodeData::MyNewNode(MyNewNodeData::default()),
            pos,
        ));
    })
};

// In the HTML:
<div class="node-item" onclick={my_new_node}>
    <div class="node-icon">{"N"}</div>  // Choose an icon letter
    <div class="node-info">
        <div class="node-name">{"My New Node"}</div>
        <div class="node-desc">{"Description here"}</div>
    </div>
</div>
```

## Step 6: Render Node Details

### Location: `components/frontend/src/components/nodes.rs`

Add a match arm in `render_node_details()`:

```rust
fn render_node_details(node: &Node, state: &AppStateContext) -> Html {
    match &node.data {
        // ... existing matches
        NodeData::MyNewNode(data) => {
            html! {
                <>
                    <text x="10" y="44" fill="#aaa" font-size="10" style="pointer-events: none;">
                        {format!("Field: {}", data.some_field)}
                    </text>
                    <text x="10" y="58" fill="#888" font-size="10" style="pointer-events: none;">
                        {format!("Value: {}", data.some_value)}
                    </text>
                </>
            }
        }
    }
}
```

## Step 7: Create Node Dialog

### Location: `components/frontend/src/components/dialog.rs`

### 7.1 Add to render_dialog_content()

```rust
fn render_dialog_content(node: &Node, state: &AppStateContext) -> Html {
    match &node.data {
        // ... existing matches
        NodeData::MyNewNode(data) => render_my_new_node_dialog(node, data, state),
    }
}
```

### 7.2 Implement the Dialog Function

```rust
fn render_my_new_node_dialog(
    node: &Node,
    data: &MyNewNodeData,
    state: &AppStateContext,
) -> Html {
    let node_id = node.id;
    let current_value = data.some_value;

    // Decrement button callback
    let state_dec = state.clone();
    let on_decrement = Callback::from(move |_| {
        if current_value > 0 {
            state_dec.dispatch(AppAction::UpdateNodeData(
                node_id,
                NodeData::MyNewNode(MyNewNodeData {
                    some_value: current_value - 1,
                    ..Default::default()
                }),
            ));
        }
    });

    // Increment button callback
    let state_inc = state.clone();
    let on_increment = Callback::from(move |_| {
        state_inc.dispatch(AppAction::UpdateNodeData(
            node_id,
            NodeData::MyNewNode(MyNewNodeData {
                some_value: current_value + 1,
                ..Default::default()
            }),
        ));
    });

    // Input change callback
    let state_change = state.clone();
    let on_value_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
        if let Ok(value) = input.value().parse::<u32>() {
            state_change.dispatch(AppAction::UpdateNodeData(
                node_id,
                NodeData::MyNewNode(MyNewNodeData {
                    some_value: value,
                    ..Default::default()
                }),
            ));
        }
    });

    html! {
        <div class="dialog-body">
            <h3>{"My New Node"}</h3>
            <div class="dialog-row">
                <label>{"Some Value:"}</label>
                <div class="number-input-group">
                    <button class="number-btn" onclick={on_decrement} disabled={current_value == 0}>{"-"}</button>
                    <input
                        type="number"
                        min="0"
                        value={current_value.to_string()}
                        onchange={on_value_change}
                    />
                    <button class="number-btn" onclick={on_increment}>{"+"}</button>
                </div>
            </div>
            <div class="dialog-row hint">
                {"Configuration hint or instructions"}
            </div>
        </div>
    }
}
```

## Step 8: Graph Resolution (Optional)

If your node needs to resolve data from connected upstream nodes:

### Location: `components/models/crates/project/src/graph.rs`

```rust
impl Project {
    /// Resolves data for MyNewNode from connected upstream nodes.
    pub fn resolve_my_data(&self, node_id: Uuid) -> Option<&SomeData> {
        let upstream = self.find_upstream_node(node_id, "input_name")?;
        match &upstream.data {
            NodeData::SomeSourceNode(data) => Some(&data.field),
            _ => None,
        }
    }
}
```

## Step 9: Backend API (If Needed)

If your node requires backend processing:

### Location: `components/cli/src/routes/`

Create a new route module or add to an existing one.

## Checklist

- [ ] Data struct with Default, Serialize, Deserialize, PartialEq
- [ ] NodeData enum variant added
- [ ] type_name() match arm
- [ ] Node constructor (new_my_new_node)
- [ ] Unit tests written first (TDD)
- [ ] Shared re-exports updated
- [ ] Frontend state create_node match arm
- [ ] Toolbox palette item
- [ ] Node details rendering
- [ ] Node dialog implementation
- [ ] Graph resolution methods (if needed)
- [ ] Backend API endpoints (if needed)
- [ ] Run `./scripts/format-all.sh`
- [ ] Run `./scripts/check-all.sh` (no errors or warnings)
- [ ] Run `./scripts/build-all.sh`
- [ ] Test with `./scripts/run.sh`

## Node Type Examples

### Source Nodes (No Inputs)
- **VideoInput**: Uploads video files, outputs video data

### Processing Nodes (Inputs + Outputs)
- **StillSampler**: Takes video, outputs array of stills
- **Selector**: Takes array, outputs selected item + passthrough

### Sink Nodes (No Outputs)
- **Viewer**: Displays video playback
- **StillPreview**: Displays selected still image

### Passthrough Nodes
- **Selector**: Has both primary output (selected item) and array passthrough

## Connection Rules

Current connection resolution is pull-based:
- Nodes look up their connected sources at render time
- Use helper functions like `find_connected_video()`, `find_stills_count()`
- Trace upstream through connections to find source data

## Styling Guidelines

- Dialog width: 600-900px
- Number inputs: Use `.number-input-group` with +/- buttons
- Font sizes: Headers 24px, labels 16px, hints 14px
- Node details: font-size="10" in SVG
