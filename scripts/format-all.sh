#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Formatting all components..."

# Format models
echo "Formatting models..."
(cd "$ROOT_DIR/shared/components/models" && cargo fmt)

# Format utilities
echo "Formatting utilities..."
(cd "$ROOT_DIR/backend/components/utilities/components/ffmpeg" && cargo fmt)

# Format shared
echo "Formatting shared..."
(cd "$ROOT_DIR/shared/components/shared" && cargo fmt)

# Format CLI
echo "Formatting cli..."
(cd "$ROOT_DIR/backend/components/cli" && cargo fmt)

# Format frontend
echo "Formatting frontend..."
(cd "$ROOT_DIR/frontend/components/yew/crates/app" && cargo fmt)

echo "All components formatted."
