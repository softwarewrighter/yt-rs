#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$ROOT_DIR/components"

echo "Formatting all components..."

for component in shared cli frontend; do
    echo "Formatting $component..."
    (cd "$COMPONENTS_DIR/$component" && cargo fmt)
done

echo "All components formatted."
