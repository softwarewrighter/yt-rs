# Physical Design

This document defines the crate structure, dependency graph, and coupling rules for yt-rs. Changes to dependencies should be reviewed against this design.

## Top-Level Directory Structure

```
yt-rs/
├── frontend/     # All browser-side code (compiles to WASM)
├── backend/      # All server-side code (native binary)
├── shared/       # Code shared between frontend and backend
├── utilities/    # Context-independent utilities
├── scripts/      # Build, check, run scripts
├── docs/         # Documentation
└── data/         # Runtime data (gitignored)
```

## Dependency Principles

1. **Unidirectional dependencies**: Lower layers never depend on higher layers
2. **Frontend/backend isolation**: Frontend and backend never directly depend on each other
3. **Shared types flow down**: shared/ only contains data types and pure functions
4. **CLI is thin**: CLI only parses args and delegates to server
5. **Builders wire components**: Builders know dependencies, components don't
6. **Utilities are independent**: ffmpeg has no dependencies on domain code

## Layer Architecture

```
                         ┌─────────────┐
                         │    CLI      │  (arg parsing only)
                         └──────┬──────┘
                                │ delegates to
                         ┌──────▼──────┐
                         │   Server    │  (orchestration)
                         └──────┬──────┘
                                │ wires up
           ┌────────────────────┼────────────────────┐
           │                    │                    │
    ┌──────▼──────┐      ┌──────▼──────┐     ┌──────▼──────┐
    │    Rest     │      │  BE-State   │     │   Agent     │
    │  (routes)   │      │ (app state) │     │ (Ollama AI) │
    └──────┬──────┘      └──────┬──────┘     └──────┬──────┘
           │                    │                    │
           └────────────────────┼────────────────────┘
                                │
                         ┌──────▼──────┐
                         │    CRUD     │  (data persistence)
                         └──────┬──────┘
                                │
                         ┌──────▼──────┐
                         │   Shared    │  (re-exports types)
                         └──────┬──────┘
                                │
           ┌────────────────────┼────────────────────┐
           │                    │                    │
    ┌──────▼──────┐      ┌──────▼──────┐     ┌──────▼──────┐
    │   Nodes     │      │   Project   │     │   Config    │
    │  (types)    │      │   (types)   │     │  (loading)  │
    └─────────────┘      └─────────────┘     └─────────────┘

                    UTILITIES (no domain deps)
           ┌─────────────────────────────────────────┐
           │                FFmpeg                    │
           └─────────────────────────────────────────┘


                    FRONTEND (WASM context)
           ┌─────────────────────────────────────────┐
           │     App → FE-State → Components         │
           │              │                          │
           │              ▼                          │
           │           Shared                        │
           └─────────────────────────────────────────┘
```

## Directory Structure

```
yt-rs/
├── frontend/
│   └── components/
│       ├── app/                    # Main Yew application
│       │   └── crates/app/
│       │       ├── src/
│       │       │   ├── lib.rs
│       │       │   ├── main.rs
│       │       │   └── components/
│       │       ├── index.html
│       │       └── styles.css
│       │
│       └── state/                  # Frontend state (use_reducer)
│           └── crates/fe-state/
│
├── backend/
│   └── components/
│       ├── cli/                    # Thin CLI wrapper
│       │   └── crates/cli/
│       │       └── src/
│       │           ├── lib.rs
│       │           ├── main.rs     # Args: help, version, config
│       │           ├── run.rs      # Delegates to server
│       │           └── stop.rs     # Posts shutdown
│       │
│       ├── server/                 # Axum server orchestration
│       │   └── crates/server/
│       │       └── src/
│       │           ├── lib.rs
│       │           ├── builder.rs  # ServerBuilder
│       │           └── runner.rs
│       │
│       ├── rest/                   # Route handlers
│       │   └── crates/rest/
│       │       └── src/
│       │           ├── lib.rs
│       │           ├── health.rs
│       │           ├── videos.rs
│       │           ├── projects.rs
│       │           └── generate.rs
│       │
│       ├── state/                  # Backend AppState
│       │   └── crates/be-state/
│       │
│       ├── crud/                   # Data persistence
│       │   └── crates/crud/
│       │
│       └── agent/                  # AI/Ollama integration
│           └── crates/agent/
│
├── shared/
│   └── components/
│       ├── models/
│       │   └── crates/
│       │       ├── nodes/          # Node, Connector, NodeData
│       │       └── project/        # Project, graph resolution
│       │
│       ├── config/                 # Configuration types
│       │   └── crates/config/
│       │       └── src/
│       │           ├── lib.rs
│       │           └── builder.rs
│       │
│       └── shared/                 # Re-exports
│           └── crates/shared/
│
└── utilities/
    └── components/
        └── ffmpeg/
            └── crates/ffmpeg/
```

## Crate Dependency Graph

### Target State

```
cli                          # THIN: arg parsing only
├── yt-rs-config            # for loading config file
└── yt-rs-server            # delegates to server

server                       # ORCHESTRATION
├── yt-rs-rest              # composes routes
├── yt-rs-be-state          # app state
├── yt-rs-config            # configuration
└── axum, tokio             # infrastructure

rest                         # ROUTE HANDLERS
├── yt-rs-shared            # for request/response types
├── yt-rs-crud              # for data access
└── axum

be-state                     # BACKEND STATE
├── yt-rs-shared            # domain types
├── yt-rs-ffmpeg            # for extraction
└── tokio

crud                         # DATA ACCESS
├── yt-rs-shared            # domain types
└── tokio, serde

agent                        # AI INTEGRATION
├── yt-rs-ffmpeg            # for image extraction
└── reqwest                 # for Ollama API

config                       # CONFIGURATION
└── toml, serde

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

fe-state                     # FRONTEND STATE (WASM)
├── yt-rs-shared
└── yew

app                          # FRONTEND APP (WASM)
├── yt-rs-fe-state
├── yt-rs-shared
└── yew, gloo
```

## Coupling Rules

### What SHOULD couple

| Crate | May depend on |
|-------|---------------|
| cli | config, server only |
| server | rest, be-state, config |
| rest | shared, crud |
| be-state | shared, ffmpeg |
| crud | shared |
| agent | ffmpeg only |
| config | (none - types only) |
| shared | nodes, project |
| nodes | (none) |
| project | nodes |
| ffmpeg | (none) |
| fe-state | shared |
| app | fe-state, shared |

### What SHOULD NOT couple

| Crate | Must NOT depend on |
|-------|-------------------|
| cli | crud, rest, shared, nodes, project, be-state |
| server | nodes, project (use shared) |
| nodes | project, shared, crud, rest |
| project | shared, crud, rest |
| ffmpeg | shared, crud, rest, nodes, project |
| shared | crud, rest, server |
| frontend/* | backend/* |
| backend/* | frontend/* |

## Module Limits (per sw-checklist)

| Metric | Limit | Warning |
|--------|-------|---------|
| Functions per module | 7 max | 5 warning |
| Lines per function | 50 max | 25 warning |
| Modules per crate | 7 max | N/A |
| Lines per file | 500 max | N/A |

## File Responsibilities

### CLI (backend/components/cli)

```
src/
├── main.rs      # parse args, match command, delegate
├── lib.rs       # re-export for tests
├── run.rs       # build config, call server::run()
└── stop.rs      # build config, post shutdown
```

CLI should NOT contain:
- Route handlers (belong in rest)
- State management (belong in be-state)
- Configuration types (belong in config)

### Server (backend/components/server)

```
src/
├── lib.rs       # pub use exports
├── builder.rs   # ServerBuilder wires up components
└── runner.rs    # run() async function
```

### Config (shared/components/config)

```
src/
├── lib.rs       # AppConfig, GenerateDialogConfig, etc.
└── builder.rs   # ConfigBuilder with file/env/default
```

### Rest (backend/components/rest)

```
src/
├── lib.rs       # pub use, create_router()
├── health.rs    # GET /health
├── videos.rs    # video CRUD routes
├── projects.rs  # project CRUD routes
├── workspace.rs # save/restore routes
└── generate.rs  # Ollama generation routes
```

### BE-State (backend/components/state)

```
src/
├── lib.rs       # AppState struct
├── video.rs     # video cache operations
├── still.rs     # still extraction operations
└── thumbnail.rs # thumbnail operations
```

## Migration Path

### Phase 1: Restructure directories

1. Create frontend/, backend/, shared/ top-level dirs
2. Move components/frontend → frontend/components/app
3. Move components/cli → backend/components/cli
4. Move components/models → shared/components/models
5. Update all Cargo.toml paths

### Phase 2: Extract config

1. Create shared/components/config
2. Move config types from cli to config
3. Add ConfigBuilder
4. Update cli to use config crate

### Phase 3: Extract be-state

1. Create backend/components/state
2. Move AppState from cli to be-state
3. Update rest routes to use be-state

### Phase 4: Thin CLI

1. Create run.rs, stop.rs in cli
2. Move server startup to run.rs
3. CLI main.rs only parses args and delegates

## Validation

Run these checks to validate physical design:

```bash
# Check dependency graph
cargo tree -p yt-rs-cli --no-dedupe | grep yt-rs

# Verify no cycles
cargo tree --duplicates

# Check tech debt
./scripts/check-tech-debt.sh

# Verify frontend/backend isolation
cargo tree -p yt-rs-frontend | grep -E "yt-rs-(cli|server|rest|crud)"
# Should output nothing
```

## Anti-patterns to Avoid

1. **God crate**: One crate that depends on everything
2. **Circular deps**: A depends on B depends on A
3. **Leaky abstractions**: Handler knows storage details
4. **Type leakage**: Internal types in public APIs
5. **Tight coupling**: Changing one crate breaks many others
6. **Context crossing**: Frontend depending on backend or vice versa
