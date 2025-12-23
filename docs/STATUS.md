# Project Status

## Current Phase: 6 - Still Sampler (In Progress)

## Overall Progress

```
Phase 0: Documentation     [████████████████████] 100%
Phase 1: Foundation        [████████████████████] 100%
Phase 2: Canvas            [████████████████████] 100%
Phase 3: Node System       [████████████████████] 100%
Phase 4: Connections       [████████████████████] 100%
Phase 5: Backend API       [████████████████████] 100%
Phase 6: Still Sampler     [████████████████████] 100%
Phase 7: Polish            [████████░░░░░░░░░░░░]  40%
```

## Implemented Node Types

| Node | Purpose | Inputs | Outputs |
|------|---------|--------|---------|
| VideoInput | Upload video files | - | video_out |
| StillSampler | Extract stills at intervals | video_in | stills_out |
| Viewer | Play uploaded video | video_in | - |
| Selector | Select one still from array | stills_in | selected_out, array_out |
| StillPreview | Display selected still | still_in | still_out |

## Recent Updates

### 2024-12-22 - UI Polish
- Fixed double-click drag bug (nodes no longer jump on double-click)
- Enlarged dialogs (600-900px width, larger fonts)
- Added +/- buttons to number inputs for easier interaction
- Fixed keyboard delete for selected nodes (focus management)
- Connection lines can be selected and deleted with Delete key

### 2024-12-22 - Server Lifecycle
- Added graceful shutdown endpoint (POST /api/v1/shutdown)
- Added CLI subcommands: `yt-rs serve`, `yt-rs stop`
- Created stop.sh script for stopping the server
- Updated run.sh to not rebuild (use build-all.sh first)

### 2024-12-22 - Selector & StillPreview Nodes
- Selector node: Selects one still from array, passes through full array
- StillPreview node: Displays selected still with thumbnail
- Graph resolution methods for tracing node connections
- Still thumbnail endpoint (GET /api/v1/stills/:video_id/:timestamp)
- Automatic still generation when connecting video to sampler

### 2024-12-22 - Video Upload & Playback
- Video upload to backend (POST /api/v1/videos/upload) - up to 500MB
- Video metadata extraction (duration, name)
- Video streaming endpoint (GET /api/v1/videos/:id/stream)
- Viewer node plays uploaded videos

### 2024-12-22 - Project Structure Refactor
- Removed Cargo workspace in favor of standalone components
- Restructured: crates/ → components/ (shared, cli, frontend)
- Added models/ workspace for node and project types
- Added scripts/ directory (build-all.sh, check-all.sh, format-all.sh, run.sh, stop.sh)

## Scripts

| Script | Purpose |
|--------|---------|
| `./scripts/build-all.sh` | Build all components (models, utilities, shared, cli, frontend) |
| `./scripts/check-all.sh` | Run clippy on all components |
| `./scripts/format-all.sh` | Format all components with rustfmt |
| `./scripts/run.sh` | Start the server (build first!) |
| `./scripts/stop.sh` | Stop the running server gracefully |

## Next Steps

1. **AI Integration**
   - Add AI node that reads still images and extracts text/description
   - Connect to StillPreview's output connector

2. **Project Persistence**
   - Save/load project state to JSON
   - Project management UI

3. **Error Handling**
   - Better error display in UI
   - Retry logic for failed uploads

## Architecture

```
components/
├── models/          # Node and project data types
│   └── crates/
│       ├── nodes/   # Node, Connector, NodeData types
│       └── project/ # Project, graph resolution
├── shared/          # Re-exports for cross-component use
├── utilities/       # FFmpeg and other utilities
├── cli/             # Axum REST server (CLI)
│   └── routes/      # API endpoints (health, projects, videos, shutdown)
└── frontend/        # Yew WASM app
    └── components/  # UI (canvas, toolbox, dialog, nodes)

scripts/
├── build-all.sh     # Build all components
├── check-all.sh     # Run clippy on all
├── format-all.sh    # Format all
├── run.sh           # Run CLI server
└── stop.sh          # Stop CLI server
```

## Links

- [Architecture](./ARCHITECTURE.md)
- [Adding Nodes Guide](./adding-nodes-guidelines.md)
- [PRD](./PRD.md)
- [Design](./DESIGN.md)
- [Implementation Plan](./PLAN.md)
- [Code Metrics Guide](./code-metrics.md)
