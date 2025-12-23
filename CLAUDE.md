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

## CRITICAL RULES (READ FIRST)

These rules are NON-NEGOTIABLE. Violating them wastes time and increases tech debt:

1. **NEVER disable lint/clippy checks** - No `#[allow(...)]`, no `// noqa`, no suppressions
2. **NEVER add dead code** - If code is unused, do not add it. Add it when needed.
3. **FIX warnings properly** - Remove unused code, fix the actual issue
4. **Pre-commit = format + fix clippy + verify .gitignore + update docs**

When I say "do NOT disable checks" I mean it literally. Fix the root cause.

## Project Overview

yt-rs is a web-based node editor for video processing workflows.

**Structure:**
```
components/
├── models/          # Graph data models (nested workspace)
│   └── crates/
│       ├── nodes/   # Node, Connection, Canvas types
│       └── project/ # Project serialization
├── shared/          # Re-exports from models
├── cli/             # Axum REST server
└── frontend/        # Yew/WASM app

scripts/
├── build-all.sh
├── check-all.sh
├── format-all.sh
└── run.sh
```

## Build Commands

```bash
# Build all components
./scripts/build-all.sh

# Check all with clippy
./scripts/check-all.sh

# Format all
./scripts/format-all.sh

# Run CLI server
./scripts/run.sh

# Build individual component
cd components/<name> && cargo build

# Run tests per component
cd components/<name> && cargo test
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

## Tech Debt Ratcheting

**Principle**: The count of `sw-checklist` failures and warnings must monotonically decrease over time. They should never go up.

**Tracking**:
- Current counts are recorded in `docs/tech-debt-baseline.md`
- A test in `components/utilities/tests/checklist_test.rs` verifies counts don't exceed baseline
- Each component tracks its own FAIL/WARN counts

**Rules**:
1. **Never increase counts** - If adding code would increase FAIL or WARN, refactor first
2. **Actively decrease every commit** - Each commit MUST reduce at least one FAIL or WARN to avoid infinite postponement
3. **Update baseline** - After reducing counts, update the baseline document and script
4. **Block merges** - PRs that increase counts should be rejected

**Goal**: All components should reach 0 FAIL and 0 WARN. Features should not increase tech debt.

**Pre-commit requirement**: Before each commit, fix at least one sw-checklist issue. This ensures steady progress toward zero debt.

**Current Baseline** (update this as counts decrease):
| Component | FAIL | WARN | Notes |
|-----------|------|------|-------|
| frontend  | 6    | 18   | Needs crate split for module count |
| cli       | 3    | 7    | state.rs still needs splitting |
| rest      | 0    | 0    | Clean |
| crud      | 0    | 2    | Minor warnings |
| agent     | 0    | 2    | Minor warnings |
| server    | 0    | 1    | Minor warnings |
| shared    | 0    | 0    | Clean |
| nodes     | 2    | 0    | Needs refactoring |
| project   | 1    | 0    | Needs refactoring |
| ffmpeg    | 1    | 4    | Needs refactoring |
| **TOTAL** | **13** | **35** | **Cannot increase** |

**Script**: Run `./scripts/check-tech-debt.sh` to verify counts don't exceed baseline.
