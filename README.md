# yt-rs

A web-based node editor for video processing workflows, built with Rust.

![Screenshot](./images/screenshot.png?ts=1766467254000)

## Features

- **Node-based workflow editor** - Visual programming interface for video processing
- **Video upload and playback** - Upload videos up to 500MB, stream playback in Viewer node
- **Still extraction** - Sample frames at configurable intervals
- **Selector node** - Pick individual stills from extracted frames
- **Still preview** - View selected frames with timestamps
- **AI dialog generation** - Analyze stills with Ollama vision models
- **Text output** - View generated prolog, clips, epilog, and YouTube description

## Quick Start

```bash
# Build all components
./scripts/build-all.sh

# Start the server
./scripts/run.sh

# Open http://localhost:1400 in your browser

# Stop the server
./scripts/stop.sh
```

## Architecture

```
frontend/
└── components/yew/           # Yew WASM application

backend/
├── components/cli/           # Axum REST server with routes
├── components/utilities/     # FFmpeg video processing
└── components/agent/         # Ollama AI client

shared/
├── components/models/        # Node and project data types
└── components/shared/        # Re-exports for cross-component use

scripts/
├── build-all.sh              # Build all components
├── check-all.sh              # Run clippy on all
├── format-all.sh             # Format all code
├── run.sh                    # Start the server
└── stop.sh                   # Stop the server
```

## Node Types

| Node | Purpose |
|------|---------|
| VideoInput | Upload video files |
| StillSampler | Extract stills at intervals |
| Viewer | Play uploaded video |
| Selector | Select one still from array |
| StillPreview | Display selected still |
| GenerateDialog | AI vision analysis of stills |
| TextView | Display generated text output |

## Scripts

| Script | Purpose |
|--------|---------|
| `./scripts/build-all.sh` | Build all components |
| `./scripts/check-all.sh` | Run clippy on all components |
| `./scripts/format-all.sh` | Format all code |
| `./scripts/run.sh` | Start the server |
| `./scripts/stop.sh` | Stop the server |

## Documentation

### Project Documentation
- [Architecture](./docs/ARCHITECTURE.md) - System design and component overview
- [Status](./docs/STATUS.md) - Current progress and recent updates
- [PRD](./docs/PRD.md) - Product requirements document
- [Design](./docs/DESIGN.md) - UI/UX design decisions
- [Plan](./docs/PLAN.md) - Implementation plan

### Development Guides
- [Adding Nodes](./docs/adding-nodes-guidelines.md) - How to add new node types
- [Code Metrics](./docs/code-metrics.md) - Code quality guidelines
- [Compliance Checklist](./docs/compliance-checklist.md) - Quality checklist

### Process Documentation
- [Process](./docs/process.md) - Development workflow
- [Tools](./docs/tools.md) - Required tools and setup
- [AI Agent Instructions](./docs/ai_agent_instructions.md) - Guidelines for AI assistants

## Technology Stack

| Layer | Technology |
|-------|------------|
| Frontend | Yew 0.21 (Rust WASM) |
| Backend | Axum 0.7 |
| AI Integration | Ollama (llama3.2-vision) |
| Video Processing | FFmpeg |
| Build Tool | Trunk (WASM), Cargo (Rust) |

## License

MIT
