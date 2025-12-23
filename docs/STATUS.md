# Project Status

## Current Phase: 8 - AI Integration (In Progress)

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
Phase 8: AI Integration    [████░░░░░░░░░░░░░░░░]  20%
Phase 9: Audio Pipeline    [░░░░░░░░░░░░░░░░░░░░]   0%
Phase 10: Video Assembly   [░░░░░░░░░░░░░░░░░░░░]   0%
```

## Implemented Node Types

| Node | Purpose | Inputs | Outputs | Status |
|------|---------|--------|---------|--------|
| VideoInput | Upload video files | - | video_out | ✅ Done |
| StillSampler | Extract stills at intervals | video_in | stills_out | ✅ Done |
| Viewer | Play uploaded video | video_in | - | ✅ Done |
| Selector | Select one still from array | stills_in | selected_out, array_out | ✅ Done |
| StillPreview | Display selected still | still_in | still_out | ✅ Done |
| GenerateDialog | AI vision analysis of stills | stills_in | text_out | ✅ Done |
| TextView | Display generated text | text_in | - | ✅ Done |

## Planned Node Types

| Node | Purpose | Inputs | Outputs | Phase |
|------|---------|--------|---------|-------|
| ExtractCommentary | Whisper STT for audio | video_in | transcript_out | 9 |
| EditDialog | LLM text editing/cleanup | text_in | text_out | 9 |
| Narration | Vibe Voice TTS | text_in | audio_out | 9 |
| LipSync | Muse Talk avatar animation | avatar_in, audio_in | video_out | 10 |
| Transparency | rembg background removal | video_in | video_out | 10 |
| Composite | ffmpeg overlay compositing | base_in, overlay_in | video_out | 10 |
| Combine | Final video assembly | clips_in[] | video_out | 10 |

## Recent Updates

### 2024-12-23 - Dialog Generation & Project Restructure
- Restructured project: frontend/, backend/, shared/ top-level directories
- Added GenerateDialog node with Ollama vision model integration
- Added TextView node displaying prolog, clips, epilog, YouTube description
- Added context text input for video-specific generation hints
- Configurable system prompt in config.toml
- YouTube description auto-generated from clip analysis
- Changed default port from 3000 to 1400

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
frontend/
└── components/yew/crates/app/    # Yew WASM application

backend/
├── components/cli/               # Axum REST server
│   └── routes/                   # API endpoints (generate, videos, projects)
├── components/utilities/         # FFmpeg, rembg processing
└── components/agent/             # Ollama client

shared/
└── components/
    ├── models/                   # Node and project data types
    └── shared/                   # Re-exports for cross-component use

scripts/
├── build-all.sh                  # Build all components
├── check-all.sh                  # Run clippy on all
├── format-all.sh                 # Format all
├── run.sh                        # Run CLI server
└── stop.sh                       # Stop CLI server
```

## Remote Server Integration

The system coordinates multiple AI/ML services running on homelab servers:

| Service | Purpose | Server |
|---------|---------|--------|
| Ollama (llama3.2-vision) | Still image analysis | big72 |
| Whisper | Speech-to-text | TBD |
| Vibe Voice | Text-to-speech (cloned voice) | TBD |
| Muse Talk | Lip-sync avatar animation | TBD |

## Queue System (Planned)

To prevent GPU VRAM exhaustion and maximize throughput:
- Per-server request queues with configurable concurrency limits
- Priority scheduling for time-sensitive operations
- Multi-project support: multiple videos can process simultaneously
- Resource utilization monitoring and backpressure

## Links

- [Architecture](./ARCHITECTURE.md)
- [Adding Nodes Guide](./adding-nodes-guidelines.md)
- [PRD](./PRD.md)
- [Design](./DESIGN.md)
- [Implementation Plan](./PLAN.md)
- [Code Metrics Guide](./code-metrics.md)
