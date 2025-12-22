#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$ROOT_DIR/components"

# Build frontend first
echo "Building frontend..."
(cd "$COMPONENTS_DIR/frontend" && trunk build)

# Run CLI server
echo "Starting CLI server..."
(cd "$COMPONENTS_DIR/cli" && cargo run -- "$@")
