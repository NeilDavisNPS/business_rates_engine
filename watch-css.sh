#!/usr/bin/env bash
set -e
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"$ROOT_DIR/scripts/ensure-tailwind.sh"

echo "Starting Tailwind watch mode (auto-rebuilding on template/source changes)..."
"$ROOT_DIR/bin/tailwindcss" -i "$ROOT_DIR/styles/input.css" -o "$ROOT_DIR/frontend/src/tailwind.css" --watch

