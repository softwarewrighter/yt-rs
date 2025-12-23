# Physical Design

This document defines the crate structure, dependency graph, and coupling rules for yt-rs. Changes to dependencies should be reviewed against this design.

## Dependency Principles

1. **Unidirectional dependencies**: Lower layers never depend on higher layers
2. **CLI is thin**: CLI only parses args and delegates to server
3. **Server orchestrates**: Server composes routers from rest and uses crud for data
4. **Shared types flow down**: Models/shared only contain data types, no behavior
5. **Utilities are independent**: ffmpeg, agent have no dependencies on domain code

## Layer Architecture

```
                    ┌─────────────┐
                    │    CLI      │  (arg parsing only)
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │   Server    │  (orchestration, startup)
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
       ┌──────▼──────┐ ┌───▼───┐ ┌──────▼──────┐
       │    Rest     │ │ Crud  │ │   Agent     │
       │  (routes)   │ │(data) │ │ (vision AI) │
       └──────┬──────┘ └───┬───┘ └──────┬──────┘
              │            │            │
              └────────────┼────────────┘
                           │
                    ┌──────▼──────┐
                    │   Shared    │  (re-exports types)
                    └──────┬──────┘
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
┌──────▼──────┐    ┌───────▼───────┐   ┌──────▼──────┐
│   Nodes     │    │   Project     │   │   FFmpeg    │
│  (types)    │    │  (types)      │   │ (utility)   │
└─────────────┘    └───────────────┘   └─────────────┘
```

## Crate Dependency Graph

### Current State (with issues noted)

```
cli
├── yt-rs-shared
├── yt-rs-ffmpeg
├── [routes module - should move to rest]
└── [state module - should move to server or app]

server
├── axum, tokio, tower-http
└── (no domain deps - correct)

rest
├── yt-rs-server
└── axum, uuid

crud
├── yt-rs-shared
└── async-trait, tokio

agent
├── yt-rs-ffmpeg
└── base64, reqwest, serde

shared
├── yt-rs-nodes
└── yt-rs-project

nodes
└── serde, uuid

project
├── yt-rs-nodes
└── serde, chrono, uuid

ffmpeg
└── (external deps only)
```

### Target State

```
cli                          # THIN: arg parsing only
└── yt-rs-server            # delegates to server

server                       # ORCHESTRATION
├── yt-rs-rest              # composes routes
├── yt-rs-crud              # data access
├── yt-rs-agent             # AI integration
├── yt-rs-ffmpeg            # video processing
├── axum, tokio             # infrastructure
└── (NO direct yt-rs-shared - goes through rest/crud)

rest                         # ROUTE HANDLERS
├── yt-rs-shared            # for request/response types
├── yt-rs-crud              # for data access in handlers
└── axum

crud                         # DATA ACCESS
├── yt-rs-shared            # domain types
└── tokio, serde

agent                        # AI INTEGRATION
├── yt-rs-ffmpeg            # for image extraction
└── reqwest                 # for Ollama API

shared                       # TYPE RE-EXPORTS (no behavior)
├── yt-rs-nodes
└── yt-rs-project

nodes                        # DOMAIN TYPES
└── serde, uuid

project                      # PROJECT TYPES
├── yt-rs-nodes
└── serde, uuid

ffmpeg                       # UTILITY (no domain deps)
└── tokio
```

## Coupling Rules

### What SHOULD couple

| Crate | May depend on |
|-------|---------------|
| cli | server only |
| server | rest, crud, agent, ffmpeg, shared |
| rest | shared, crud |
| crud | shared |
| agent | ffmpeg only |
| shared | nodes, project |
| nodes | (none) |
| project | nodes |
| ffmpeg | (none) |

### What SHOULD NOT couple

| Crate | Must NOT depend on |
|-------|-------------------|
| cli | crud, rest, shared, nodes, project |
| nodes | project, shared, crud, rest |
| project | shared, crud, rest |
| ffmpeg | shared, crud, rest, nodes, project |
| shared | crud, rest, server |

## Module Limits (per sw-checklist)

| Metric | Limit | Warning |
|--------|-------|---------|
| Functions per module | 7 max | 5 warning |
| Lines per function | 50 max | 25 warning |
| Modules per crate | 7 max | N/A |
| Lines per file | 350 max | N/A |

## File Responsibilities

### CLI (components/cli)

```
src/
├── main.rs      # parse args, call server::run()
└── lib.rs       # re-export for tests
```

CLI should NOT contain:
- Route handlers (move to rest)
- State management (move to server or app)
- Configuration loading (move to server)

### Server (components/server)

```
src/
├── lib.rs       # pub use exports
├── config.rs    # ServerConfig struct
├── runner.rs    # run() function
├── shutdown.rs  # ShutdownSignal
└── app.rs       # AppState, compose router (NEW)
```

Server SHOULD contain:
- AppState struct and initialization
- Router composition from rest routes
- Configuration management

### Rest (components/rest)

```
src/
├── lib.rs       # pub use exports
├── router.rs    # create_router()
├── health.rs    # health routes
├── shutdown.rs  # shutdown routes
├── projects.rs  # project CRUD routes (MOVE from cli)
└── videos.rs    # video routes (MOVE from cli)
```

### Crud (components/crud)

```
src/
├── lib.rs       # pub use exports
├── store.rs     # FileStore struct
├── project.rs   # ProjectStore trait
├── video.rs     # VideoStore trait
├── error.rs     # CrudError
└── store/       # implementations
    ├── project_impl.rs
    └── video_impl.rs
```

## Frontend Structure (components/frontend)

Current: Single crate with many modules (exceeds 7 module limit)

Target: Split into multiple crates

```
frontend/               # Main entry, app shell
├── frontend-state/    # State management
├── frontend-nodes/    # Node components
├── frontend-dialogs/  # Dialog components
├── frontend-canvas/   # Canvas component
└── frontend-ui/       # Shared UI components
```

## Migration Path

### Phase 1: Server takes AppState

1. Move AppState from cli/state.rs to server/app.rs
2. Server exposes create_app() that builds router with state
3. CLI calls server::create_app() then server::run()

### Phase 2: Routes move to Rest

1. Move cli/routes/projects.rs to rest/projects.rs
2. Move cli/routes/videos.rs to rest/videos.rs
3. Rest creates router with all domain routes
4. Server composes rest router

### Phase 3: Frontend crate split

1. Extract frontend-state crate
2. Extract frontend-canvas crate
3. Extract frontend-nodes crate
4. Extract frontend-dialogs crate
5. Main frontend composes all

## Validation

Run these checks to validate physical design:

```bash
# Check dependency graph
cargo tree -p yt-rs-cli --no-dedupe | grep yt-rs

# Verify no cycles
cargo tree --duplicates

# Check tech debt
./scripts/check-tech-debt.sh
```

## Anti-patterns to Avoid

1. **God crate**: One crate that depends on everything
2. **Circular deps**: A depends on B depends on A
3. **Leaky abstractions**: Handler knows storage details
4. **Type leakage**: Internal types in public APIs
5. **Tight coupling**: Changing one crate breaks many others
