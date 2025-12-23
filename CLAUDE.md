# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## BEFORE STARTING ANY WORK

**ALWAYS read README.md first** and consider ALL linked documentation before making changes:

- `docs/adding-nodes-guidelines.md` - **REQUIRED** when adding new node types (9-step checklist)
- `docs/compliance-checklist.md` - **REQUIRED** rules that cannot be violated
- `docs/physical-design.md` - **REQUIRED** crate dependencies and coupling rules
- `docs/process.md` - TDD workflow and pre-commit requirements
- `docs/ARCHITECTURE.md` - System design patterns to follow
- `docs/STATUS.md` - Current project state and recent changes

## CRITICAL RULES (NON-NEGOTIABLE)

1. **NEVER disable lint/clippy checks** - No `#[allow(...)]`, no suppressions. Fix the root cause.
2. **NEVER add dead code** - If code is unused, do not add it. Add it when needed.
3. **No functions in lib.rs or mod.rs** - These files contain only module declarations and re-exports.
4. **Pre-commit = format + clippy + tests + sw-checklist + update docs**

## Project Structure

```
frontend/
└── components/yew/           # Yew WASM application

backend/
├── components/cli/           # Thin CLI (args, run, stop)
├── components/server/        # Axum HTTP server
├── components/rest/          # REST API route handlers
├── components/crud/          # Data persistence operations
├── components/agent/         # Ollama AI client
└── components/utilities/     # FFmpeg video processing

shared/
├── components/models/        # Node and project data types (nodes, project crates)
└── components/shared/        # Re-exports for cross-component use
```

## Build Commands

```bash
# Build all components
./scripts/build-all.sh

# Check all with clippy (zero warnings required)
./scripts/check-all.sh

# Format all
./scripts/format-all.sh

# Run server (http://localhost:1400)
./scripts/run.sh

# Stop server
./scripts/stop.sh

# Check tech debt ratchet
./scripts/check-tech-debt.sh

# Build/test individual component
cd frontend/components/yew && cargo build
cd backend/components/cli && cargo test
```

## Pre-Commit Quality Process (Mandatory)

Run this sequence before every commit:

1. `./scripts/format-all.sh` - Format all code
2. `./scripts/check-all.sh` - Zero clippy warnings
3. `cargo test` in each component - All tests pass
4. `./scripts/check-tech-debt.sh` - Debt cannot increase
5. Update docs if behavior changed

## Architecture

Three-tier full-stack Rust:

```
Browser (Yew/WASM) <--REST--> Axum Backend <--> File System + Ollama
```

**Frontend State**: Yew `use_reducer` with Context for shared state (canvas, nodes, connections)

**Backend Services**:
- Video processing via ffmpeg-sidecar
- AI vision analysis via Ollama client
- File storage for uploads
- JSON file persistence for projects

**Canvas Rendering**: SVG with foreignObject for node HTML content. Coordinate transform: `canvas = (screen - pan) / zoom`

## Key Patterns

- **Node Types**: Each node has typed input/output connectors. Data flows through connections.
- **Connections**: Bezier curves between connectors, rendered in SVG
- **Data Flow**: UI action -> State reducer -> API call -> Backend persist -> State update
- **TDD**: Write failing test first, then implement, then refactor

## Code Standards

- Rust 2024 edition
- Files under 500 lines (prefer 200-300)
- Functions under 50 lines
- Use inline format args: `format!("{name}")` not `format!("{}", name)`
- Module docs use `//!`, item docs use `///`
- Maximum 3 TODOs per file, never commit FIXMEs

## Tech Debt Ratchet

FAIL/WARN counts must monotonically decrease. Run `./scripts/check-tech-debt.sh` to verify.

**Current Baseline** (from scripts/check-tech-debt.sh):
| Component | FAIL | WARN |
|-----------|------|------|
| yew-app   | 9    | 18   |
| cli       | 5    | 11   |
| rest      | 0    | 0    |
| crud      | 0    | 2    |
| agent     | 0    | 2    |
| server    | 0    | 1    |
| shared    | 0    | 0    |
| nodes     | 2    | 0    |
| project   | 1    | 0    |
| ffmpeg    | 1    | 4    |
| **TOTAL** | **18** | **38** |

New components MUST start at 0:0. Update baselines in `scripts/check-tech-debt.sh` only after reducing counts.
