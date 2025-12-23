#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$ROOT_DIR/components"

# Use debug binary by default, release if specified
BUILD_TYPE="${1:-debug}"
BINARY="$COMPONENTS_DIR/cli/target/$BUILD_TYPE/yt-rs"

if [ ! -f "$BINARY" ]; then
    echo "Binary not found at $BINARY"
    echo "Run ./scripts/build-all.sh first"
    exit 1
fi

echo "Stopping yt-rs server..."
"$BINARY" stop "$@"
