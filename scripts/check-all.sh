#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$ROOT_DIR/components"

echo "Running clippy on all components..."

for component in models utilities shared cli frontend; do
    echo "Checking $component..."
    (cd "$COMPONENTS_DIR/$component" && cargo clippy -- -D warnings)
done

echo "All components passed clippy."
