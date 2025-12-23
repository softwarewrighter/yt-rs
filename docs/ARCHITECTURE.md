# Architecture

## System Overview

yt-rs is a web-based node editor for video processing workflows. The system consists of two main components:

```
┌─────────────────────────────────────────────────────────────────┐
│                         Browser                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Yew/WASM Frontend                           │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │   │
│  │  │   Canvas    │  │   Toolbox   │  │  State Manager  │ │   │
│  │  │  (SVG)      │  │             │  │  (use_reducer)  │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────────┘ │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │   │
│  │  │   Nodes     │  │ Connections │  │   API Client    │ │   │
│  │  │             │  │  (Bezier)   │  │   (gloo-net)    │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────────┘ │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ HTTP/REST
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Axum Backend                                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │
│  │   Routes    │  │  Handlers   │  │      Services           │ │
│  │  /api/v1/*  │  │             │  │  ┌─────────────────┐   │ │
│  └─────────────┘  └─────────────┘  │  │  Video Service  │   │ │
│  ┌─────────────┐  ┌─────────────┐  │  │  (ffmpeg)       │   │ │
│  │Static Files │  │   State     │  │  └─────────────────┘   │ │
│  │  (WASM)     │  │             │  │  ┌─────────────────┐   │ │
│  └─────────────┘  └─────────────┘  │  │ Storage Service │   │ │
│                                     │  └─────────────────┘   │ │
│                                     └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      File System                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │
│  │   Videos    │  │   Stills    │  │      Projects           │ │
│  │  (uploads)  │  │ (extracted) │  │      (JSON)             │ │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Physical Design

See `docs/physical-design.md` for the authoritative crate dependency graph and coupling rules.

## Project Structure

```
yt-rs/
├── components/             # Self-contained Rust components
│   ├── models/             # Data type definitions
│   │   └── crates/
│   │       ├── nodes/      # Node, Connector, NodeData types
│   │       │   ├── src/lib.rs
│   │       │   └── tests/  # Node type tests
│   │       └── project/    # Project, graph resolution
│   │           ├── src/
│   │           │   ├── lib.rs
│   │           │   └── graph.rs
│   │           └── tests/  # Graph resolution tests
│   │
│   ├── shared/             # Re-exports for cross-component use
│   │   └── src/lib.rs      # Re-exports from nodes/project
│   │
│   ├── utilities/          # Utility crates
│   │   └── crates/
│   │       └── ffmpeg/     # FFmpeg subprocess wrapper
│   │
│   ├── cli/                # Axum REST server CLI
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs     # CLI entry (serve/stop subcommands)
│   │       ├── lib.rs
│   │       ├── state.rs    # Server state
│   │       └── routes/
│   │           ├── mod.rs
│   │           ├── health.rs
│   │           ├── projects.rs
│   │           ├── videos.rs
│   │           └── shutdown.rs
│   │
│   └── frontend/           # Yew WASM application
│       ├── Cargo.toml
│       ├── Trunk.toml
│       ├── index.html
│       ├── styles.css
│       └── src/
│           ├── main.rs
│           ├── app.rs
│           ├── state.rs    # Frontend state (use_reducer)
│           └── components/
│               ├── mod.rs
│               ├── toolbox.rs
│               ├── dialog.rs
│               ├── nodes.rs
│               └── canvas/
│                   ├── mod.rs
│                   ├── component.rs
│                   ├── callbacks.rs
│                   └── connections.rs
│
├── scripts/                # Build and run scripts
│   ├── build-all.sh        # Build all components
│   ├── check-all.sh        # Run clippy on all
│   ├── format-all.sh       # Format all
│   ├── run.sh              # Start CLI server
│   └── stop.sh             # Stop CLI server
│
├── data/                   # Runtime data (gitignored)
├── dist/                   # Built frontend (gitignored)
├── docs/                   # Documentation
└── README.md
```

## Data Flow

### Node Creation Flow
```
1. User drags node from Toolbox
2. Drop event on Canvas
3. Canvas dispatches CreateNode action
4. State reducer creates node with UUID
5. API client POSTs to /api/v1/projects/:id/nodes
6. Backend validates and persists
7. Response updates local state
```

### Connection Flow
```
1. User clicks output connector
2. PendingConnection state created
3. Mouse move updates pending endpoint
4. User drops on input connector
5. Validate connection (type compatibility)
6. API client POSTs to /api/v1/connections
7. Connection added to state
8. Bezier curve rendered
```

### Video Processing Flow
```
1. User uploads video via VideoInputNode
2. File uploaded to /api/v1/files/upload
3. File stored, metadata returned
4. Node updated with file reference
5. User connects to StillSamplerNode
6. Connection triggers extraction job
7. POST to /api/v1/processing/extract-stills
8. Backend spawns ffmpeg subprocess
9. Frontend polls job status
10. On completion, stills metadata returned
11. StillSamplerNode updates with output connectors
```

## Node Types

| Node | Purpose | Inputs | Outputs |
|------|---------|--------|---------|
| VideoInput | Upload and store video files | - | video_out |
| StillSampler | Extract stills at configurable intervals | video_in | stills_out |
| Viewer | Play uploaded video with controls | video_in | - |
| Selector | Select one still from array | stills_in | selected_out, array_out |
| StillPreview | Display selected still image | still_in | still_out |

### Node Data Structures

```rust
// Each node type has associated data
pub enum NodeData {
    VideoInput(VideoInputData),      // file_id, file_name, duration
    StillSampler(StillSamplerData),  // interval_seconds, extracted_stills
    Viewer(ViewerData),              // thumbnail_path
    Selector(SelectorData),          // selected_index
    StillPreview(StillPreviewData),  // marker type
}
```

### Connection Resolution

Nodes use **pull-based resolution** - they look up their connected sources at render time:

```rust
// Example: Finding connected video for a Viewer node
fn find_connected_video(node: &Node, state: &AppStateContext) -> Option<VideoInputData> {
    let input_conn = node.inputs.first()?;
    let connection = state.connections.values()
        .find(|c| c.to_node == node.id && c.to_connector == input_conn.id)?;
    let source_node = state.nodes.get(&connection.from_node)?;
    match &source_node.data {
        NodeData::VideoInput(data) => Some(data.clone()),
        _ => None,
    }
}
```

## Key Components

### Frontend

| Component | Responsibility |
|-----------|----------------|
| `Canvas` | SVG container with pan/zoom, node rendering |
| `nodes.rs` | Node SVG rendering, connector placement |
| `dialog.rs` | Node configuration dialogs |
| `toolbox.rs` | Collapsible sidebar with node palette |
| `connections.rs` | Bezier curve rendering |
| `callbacks.rs` | Mouse and keyboard event handlers |

### Backend

| Component | Responsibility |
|-----------|----------------|
| `main.rs` | CLI parsing (serve/stop), server startup |
| `routes/health.rs` | Health check endpoint |
| `routes/videos.rs` | Video upload, streaming, still extraction |
| `routes/projects.rs` | Project CRUD operations |
| `routes/shutdown.rs` | Graceful server shutdown |
| `state.rs` | Application state, thumbnail caching |

### API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/api/v1/health` | Health check |
| POST | `/api/v1/videos/upload` | Upload video file (max 500MB) |
| GET | `/api/v1/videos/:id/stream` | Stream video file |
| GET | `/api/v1/stills/:video_id/:timestamp` | Get still at timestamp |
| POST | `/api/v1/shutdown` | Graceful server shutdown |

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Frontend Framework | Yew 0.21 | Rust WASM UI framework |
| Browser APIs | gloo | Rust wrappers for Web APIs |
| HTTP Client | gloo-net | Fetch API wrapper |
| Build Tool | Trunk | WASM bundler |
| Backend Framework | Axum 0.7 | Async web framework |
| Runtime | Tokio | Async runtime |
| Video Processing | ffmpeg-sidecar | FFmpeg subprocess management |
| Serialization | Serde | JSON serialization |
| CLI | Clap | Command-line argument parsing |

## Rendering Strategy

The canvas uses SVG for rendering:

```
<svg viewBox="0 0 {width} {height}">
  <g transform="translate({panX}, {panY}) scale({zoom})">
    <!-- Grid pattern -->
    <rect fill="url(#grid)" />

    <!-- Connections layer (rendered first, behind nodes) -->
    <g class="connections">
      <path d="M... C..." />  <!-- Bezier curves -->
    </g>

    <!-- Nodes layer -->
    <g class="nodes">
      <foreignObject>
        <!-- HTML node content -->
      </foreignObject>
    </g>
  </g>
</svg>
```

### Coordinate Systems

1. **Screen coordinates**: Pixel position in browser window
2. **Canvas coordinates**: Position in the infinite canvas space
3. **Transform**: `canvas = (screen - pan) / zoom`

## State Management

Uses Yew's `use_reducer` with Context for shared state:

```rust
pub struct AppState {
    pub canvas: CanvasState,
    pub nodes: HashMap<Uuid, Node>,
    pub connections: HashMap<Uuid, Connection>,
    pub selection: Selection,
    pub pending_connection: Option<PendingConnection>,
}

pub enum AppAction {
    // Canvas
    SetPan(Position),
    SetZoom(f64),

    // Nodes
    CreateNode(NodeData, Position),
    UpdateNode(Uuid, NodeData),
    DeleteNode(Uuid),
    MoveNode(Uuid, Position),

    // Connections
    StartConnection(Uuid, Uuid),
    UpdatePendingConnection(Position),
    CompleteConnection(Uuid, Uuid),
    CancelConnection,
    DeleteConnection(Uuid),

    // Selection
    Select(Uuid),
    Deselect,
}
```

## Security Considerations

- File uploads validated for type and size
- Uploaded files stored outside web root
- API endpoints validate project ownership
- No arbitrary file path access
- FFmpeg subprocess sandboxed
