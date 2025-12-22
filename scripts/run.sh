#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$ROOT_DIR/components"
DATA_DIR="$ROOT_DIR/data"
DIST_DIR="$ROOT_DIR/dist"

# Build frontend first
echo "Building frontend..."
(cd "$COMPONENTS_DIR/frontend" && trunk build)

# Ensure data directory exists
mkdir -p "$DATA_DIR"

# Run CLI server
echo "Starting CLI server..."
echo "  Data dir: $DATA_DIR"
echo "  Static dir: $DIST_DIR"
"$COMPONENTS_DIR/cli/target/debug/yt-rs" \
    --data-dir "$DATA_DIR" \
    --static-dir "$DIST_DIR" \
    "$@"
