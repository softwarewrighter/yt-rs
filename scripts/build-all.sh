#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Building all components..."

# Build models first (dependency for shared)
echo "Building models..."
(cd "$ROOT_DIR/shared/components/models" && cargo build)

# Build utilities
echo "Building utilities..."
(cd "$ROOT_DIR/backend/components/utilities/components/ffmpeg" && cargo build)

# Build shared (depends on models)
echo "Building shared..."
(cd "$ROOT_DIR/shared/components/shared" && cargo build)

# Build CLI
echo "Building cli..."
(cd "$ROOT_DIR/backend/components/cli" && cargo build)

# Build frontend (WASM)
echo "Building frontend..."
(cd "$ROOT_DIR/frontend/components/yew/crates/app" && trunk build)

echo "All components built successfully."
