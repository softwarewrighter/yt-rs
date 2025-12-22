#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$ROOT_DIR/components"

echo "Building all components..."

# Build shared first (dependency for others)
echo "Building shared..."
(cd "$COMPONENTS_DIR/shared" && cargo build)

# Build CLI
echo "Building cli..."
(cd "$COMPONENTS_DIR/cli" && cargo build)

# Build frontend (WASM)
echo "Building frontend..."
(cd "$COMPONENTS_DIR/frontend" && trunk build)

echo "All components built successfully."
