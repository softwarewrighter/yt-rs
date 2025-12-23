#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Running clippy on all components..."

# Check models
echo "Checking models..."
(cd "$ROOT_DIR/shared/components/models" && cargo clippy -- -D warnings)

# Check utilities
echo "Checking utilities..."
(cd "$ROOT_DIR/backend/components/utilities/components/ffmpeg" && cargo clippy -- -D warnings)

# Check shared
echo "Checking shared..."
(cd "$ROOT_DIR/shared/components/shared" && cargo clippy -- -D warnings)

# Check CLI
echo "Checking cli..."
(cd "$ROOT_DIR/backend/components/cli" && cargo clippy -- -D warnings)

# Check frontend
echo "Checking frontend..."
(cd "$ROOT_DIR/frontend/components/yew/crates/app" && cargo clippy -- -D warnings)

echo "All components passed clippy."
