#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$ROOT_DIR/components"
DATA_DIR="$ROOT_DIR/data"
DIST_DIR="$ROOT_DIR/dist"

# Use debug binary by default, release if --release specified
BUILD_TYPE="debug"
if [ "$1" = "--release" ]; then
    BUILD_TYPE="release"
    shift
fi

BINARY="$COMPONENTS_DIR/cli/target/$BUILD_TYPE/yt-rs"

if [ ! -f "$BINARY" ]; then
    echo "Binary not found at $BINARY"
    echo "Run ./scripts/build-all.sh first"
    exit 1
fi

if [ ! -d "$DIST_DIR" ]; then
    echo "Frontend not built. Run ./scripts/build-all.sh first"
    exit 1
fi

# Ensure data directory exists
mkdir -p "$DATA_DIR"

# Run CLI server
echo "Starting yt-rs server..."
echo "  Binary: $BINARY"
echo "  Data dir: $DATA_DIR"
echo "  Static dir: $DIST_DIR"
"$BINARY" serve \
    --data-dir "$DATA_DIR" \
    --static-dir "$DIST_DIR" \
    "$@"
