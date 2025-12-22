# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

yt-rs is a web-based node editor for video processing workflows. It uses a Cargo workspace architecture with three crates:
- **crates/shared**: Data models shared between frontend and backend
- **crates/backend**: Axum REST server with CLI, serves static WASM files
- **crates/frontend**: Yew/WASM application (all UI logic in Rust, no JavaScript)

## Build Commands

```bash
# Build all crates
cargo build

# Build release
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Linting (zero warnings required)
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all

# Generate docs
cargo doc --open
```

## Frontend Development (Yew/WASM)

```bash
# Install trunk (WASM bundler)
cargo install trunk

# Build frontend (from crates/frontend/)
trunk build

# Development server with hot reload
trunk serve
```

## Pre-Commit Quality Process (Mandatory)

All changes must pass this sequence before committing:

1. `cargo test` - ALL tests must pass
2. `cargo clippy --all-targets --all-features -- -D warnings` - ZERO warnings
3. `cargo fmt --all` - Format all code
4. `markdown-checker -f "**/*.md"` - Validate markdown (ASCII-only)
5. `sw-checklist` - Project requirements check
6. Update docs/learnings.md if issues were found

Never use `#[allow(...)]` to suppress clippy warnings. Fix them properly.

## Architecture

Three-tier architecture with full-stack Rust:

```
Browser (Yew/WASM) <--REST--> Axum Backend <--> File System
```

**Frontend State**: Yew `use_reducer` with Context for shared state (canvas, nodes, connections)

**Backend Services**:
- Video processing via ffmpeg-sidecar
- File storage for uploads
- JSON file persistence for projects

**Canvas Rendering**: SVG with foreignObject for node HTML content. Coordinate transform: `canvas = (screen - pan) / zoom`

## Key Patterns

- **Node Types**: VideoInputNode (file upload, output connector), StillSamplerNode (interval input, dynamic outputs)
- **Connections**: Bezier curves between connectors, rendered in SVG
- **Data Flow**: UI action -> State reducer -> API call -> Backend persist -> State update

## Development Process

This project follows TDD (Red/Green/Refactor):
1. Write failing test
2. Implement minimal code to pass
3. Refactor while keeping tests green

When requested, perform a **checkpoint**: run tests, fix linting, format code, update docs, commit and push immediately.

## Documentation

- `docs/ARCHITECTURE.md` - System design and component responsibilities
- `docs/PRD.md` - Product requirements and user stories
- `docs/DESIGN.md` - Visual design, API specs, interaction flows
- `docs/PLAN.md` - Implementation phases and checklist
- `docs/STATUS.md` - Current progress and blockers
- `docs/process.md` - Development workflow details
- `docs/tools.md` - Available CLI tools in ~/.local/softwarewrighter/bin/

## Code Standards

- Rust 2024 edition
- Files under 500 lines (prefer 200-300)
- Functions under 50 lines
- Use inline format args: `format!("{name}")` not `format!("{}", name)`
- Module docs use `//!`, item docs use `///`
- Maximum 3 TODOs per file, never commit FIXMEs
