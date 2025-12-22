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

## Crate Structure

```
yt-rs/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── shared/             # Shared types (frontend + backend)
│   │   └── src/
│   │       ├── lib.rs
│   │       └── models/
│   │           ├── node.rs
│   │           ├── connection.rs
│   │           ├── canvas.rs
│   │           └── project.rs
│   │
│   ├── backend/            # Axum REST server
│   │   └── src/
│   │       ├── main.rs     # CLI entry point
│   │       ├── config.rs
│   │       ├── state.rs
│   │       ├── routes/
│   │       ├── handlers/
│   │       ├── services/
│   │       └── error.rs
│   │
│   └── frontend/           # Yew WASM application
│       ├── index.html      # Trunk entry
│       ├── styles/
│       └── src/
│           ├── main.rs
│           ├── app.rs
│           ├── components/
│           │   ├── canvas/
│           │   ├── nodes/
│           │   ├── connections/
│           │   └── toolbox/
│           ├── hooks/
│           ├── state/
│           ├── services/
│           └── utils/
│
├── docs/                   # Documentation
└── assets/                 # Static assets
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

## Key Components

### Frontend

| Component | Responsibility |
|-----------|----------------|
| `Workspace` | Infinite canvas with pan/zoom transforms |
| `Viewport` | Scroll management and visible area calculation |
| `Canvas` | SVG container, drop zone for nodes |
| `BaseNode` | Draggable node wrapper with connectors |
| `VideoInputNode` | File upload UI, video metadata display |
| `StillSamplerNode` | Interval input, dynamic output connectors |
| `BezierConnection` | SVG path rendering with control handles |
| `Toolbox` | Collapsible sidebar with node palette |
| `Connector` | Input/output connection points |

### Backend

| Component | Responsibility |
|-----------|----------------|
| `main.rs` | CLI parsing, server startup |
| `routes/` | API endpoint definitions |
| `handlers/` | Request handling logic |
| `services/video.rs` | FFmpeg integration |
| `services/storage.rs` | File system operations |
| `state.rs` | Application state (projects, files) |

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
