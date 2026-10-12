#!/usr/bin/env bash
set -e
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PATH="$ROOT_DIR/bin:$PATH"

if ! command -v node &> /dev/null; then
    echo "Node not found in PATH, running setup-node.sh..."
    "$ROOT_DIR/setup-node.sh"
fi

if ! command -v node &> /dev/null; then
    echo "Error: Node.js is required to build the frontend." >&2
    echo "Please install Node.js 18+ or 20+ (LTS) or run ./setup-node.sh" >&2
    exit 1
fi

echo "Node: $(node -v), NPM: $(npm -v)"

"$ROOT_DIR/scripts/ensure-tailwind.sh"

echo "Compiling Tailwind CSS..."
"$ROOT_DIR/bin/tailwindcss" -i "$ROOT_DIR/styles/input.css" -o "$ROOT_DIR/frontend/src/tailwind.css" --minify

cd "$ROOT_DIR/frontend"

if [ ! -d "node_modules" ]; then
    echo "Installing frontend dependencies..."
    npm install
fi

echo "Building Svelte 5 frontend with Vite..."
npm run build
echo "Frontend build complete: frontend/dist/"

