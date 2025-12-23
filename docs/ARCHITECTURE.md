# Architecture

## Design Principles

1. **Clear frontend/backend separation** - Directory structure reflects execution context
2. **Thin CLI** - Just args parsing, delegates to server/stop commands
3. **Loose coupling** - Components interact via interfaces, not implementations
4. **Builders pattern** - Builders wire up components, know dependencies
5. **Adapters/facades** - Between high-level and low-level code

## System Overview

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
├── frontend/                    # All browser-side code
│   └── components/
│       ├── app/                 # Yew WASM application
│       │   └── crates/
│       │       └── app/
│       │           ├── src/
│       │           │   ├── lib.rs
│       │           │   ├── main.rs
│       │           │   └── components/
│       │           ├── index.html
│       │           └── styles.css
│       │
│       ├── state/               # Frontend state (use_reducer)
│       │   └── crates/
│       │       └── fe-state/
│       │
│       └── macros/              # Frontend-specific macros (future)
│           └── crates/
│               └── fe-macros/
│
├── backend/                     # All server-side code
│   └── components/
│       ├── cli/                 # Thin CLI wrapper
│       │   └── crates/
│       │       └── cli/
│       │           └── src/
│       │               ├── lib.rs
│       │               ├── main.rs  # Args only: help, version, config
│       │               ├── run.rs   # Delegates to server
│       │               └── stop.rs  # Posts shutdown to running server
│       │
│       ├── server/              # Axum server component
│       │   └── crates/
│       │       └── server/
│       │           └── src/
│       │               ├── lib.rs
│       │               ├── builder.rs  # ServerBuilder wires up routes
│       │               └── runner.rs   # Server run loop
│       │
│       ├── routes/              # Route handlers (REST layer)
│       │   └── crates/
│       │       └── rest/
│       │           └── src/
│       │               ├── lib.rs
│       │               ├── health.rs
│       │               ├── videos.rs
│       │               ├── projects.rs
│       │               └── generate.rs
│       │
│       ├── state/               # Backend state (AppState)
│       │   └── crates/
│       │       └── be-state/
│       │
│       ├── crud/                # Database/storage operations
│       │   └── crates/
│       │       └── crud/
│       │
│       └── macros/              # Backend-specific macros (future)
│           └── crates/
│               └── be-macros/
│
├── shared/                      # Code shared between frontend/backend
│   └── components/
│       ├── models/              # Data type definitions
│       │   └── crates/
│       │       ├── nodes/       # Node, Connector, NodeData types
│       │       └── project/     # Project, graph resolution
│       │
│       ├── config/              # Configuration types and loading
│       │   └── crates/
│       │       └── config/
│       │           └── src/
│       │               ├── lib.rs
│       │               └── builder.rs  # ConfigBuilder
│       │
│       └── shared/              # Re-exports for convenience
│           └── crates/
│               └── shared/
│
├── utilities/                   # Utility crates (context-independent)
│   └── components/
│       └── ffmpeg/
│           └── crates/
│               └── ffmpeg/
│
├── scripts/                     # Build and run scripts
│   ├── build-all.sh
│   ├── check-all.sh
│   ├── format-all.sh
│   ├── run.sh
│   └── stop.sh
│
├── data/                        # Runtime data (gitignored)
├── docs/                        # Documentation
└── README.md
```

## Component Responsibilities

### CLI (Thin Shell)
```
Responsibility: Parse args, delegate to run/stop
Does NOT: Know about routes, handlers, state internals
Depends on: config (for reading config file), server (to start)
```

### Config
```
Responsibility: Load and validate configuration
Provides: ConfigBuilder for constructing config from args/file/env
Used by: CLI (to build config), Server (to configure), Stop (to find port)
```

### Server
```
Responsibility: Wire up and run the HTTP server
Provides: ServerBuilder that accepts config and returns configured server
Depends on: routes, state, config
Does NOT: Know CLI args or how config was built
```

### Routes (REST Layer)
```
Responsibility: HTTP request/response handling
Provides: Route registration, request validation, response formatting
Depends on: crud (for data operations), shared models
Does NOT: Implement business logic or direct file access
```

### CRUD
```
Responsibility: Data persistence operations
Provides: Repository pattern for projects, videos, stills
Depends on: shared models, file system
Does NOT: Know about HTTP or routes
```

### State (Backend)
```
Responsibility: Runtime server state
Provides: AppState with video cache, thumbnail cache
Depends on: ffmpeg (for extraction)
Does NOT: Know about frontend state or routes
```

### State (Frontend)
```
Responsibility: UI state management
Provides: use_reducer context, actions, state
Depends on: shared models
Does NOT: Know about backend implementation
```

## Builder Pattern

Builders wire up components without tight coupling:

```rust
// CLI main.rs - thin shell
fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run => {
            let config = ConfigBuilder::new()
                .with_file(args.config_file)
                .with_verbosity(args.verbosity)
                .build();

            run::start(config);  // Delegates to server
        }
        Command::Stop => {
            let config = ConfigBuilder::new()
                .with_file(args.config_file)
                .build();

            stop::shutdown(config);  // Posts to running server
        }
    }
}

// server/builder.rs - wires up server
impl ServerBuilder {
    pub fn new(config: AppConfig) -> Self { ... }

    pub fn build(self) -> Server {
        let state = StateBuilder::new(&self.config).build();
        let routes = RouteBuilder::new()
            .with_health()
            .with_videos()
            .with_projects()
            .build();

        Server::new(routes, state)
    }
}
```

## Adapter/Facade Layers

High-level code uses facades to interact with low-level services:

```
┌─────────────────────────────────────────────┐
│  Routes (high-level HTTP handlers)          │
└──────────────────┬──────────────────────────┘
                   │ uses
                   ▼
┌─────────────────────────────────────────────┐
│  VideoFacade (business operations)          │
│  - upload_video(file) -> VideoMeta          │
│  - extract_stills(id, interval) -> [Still]  │
│  - get_thumbnail(id) -> Image               │
└──────────────────┬──────────────────────────┘
                   │ delegates to
                   ▼
┌─────────────────────────────────────────────┐
│  CRUD + FFmpeg (low-level operations)       │
└─────────────────────────────────────────────┘
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

## Node Types

| Node | Purpose | Inputs | Outputs |
|------|---------|--------|---------|
| VideoInput | Upload and store video files | - | video_out |
| StillSampler | Extract stills at configurable intervals | video_in | stills_out |
| Viewer | Play uploaded video with controls | video_in | - |
| Selector | Select one still from array | stills_in | selected_out, array_out |
| StillPreview | Display selected still image | still_in | still_out |
| GenerateDialog | Generate dialog from stills via Ollama | stills_in | text_out |
| TextView | Display generated text | text_in | - |

## API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/api/v1/health` | Health check |
| POST | `/api/v1/videos/upload` | Upload video file (max 500MB) |
| GET | `/api/v1/videos/:id/stream` | Stream video file |
| GET | `/api/v1/stills/:video_id/:timestamp` | Get still at timestamp |
| POST | `/api/v1/generate/dialog` | Generate dialog from stills |
| POST | `/api/v1/workspace/save` | Save current workspace |
| GET | `/api/v1/workspace/restore` | Restore saved workspace |
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
| Config | toml | Configuration file format |

## Security Considerations

- File uploads validated for type and size
- Uploaded files stored outside web root
- API endpoints validate project ownership
- No arbitrary file path access
- FFmpeg subprocess sandboxed
