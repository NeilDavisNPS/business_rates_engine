#!/usr/bin/env bash
set -e
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PATH="$ROOT_DIR/bin:$PATH"

if ! command -v node &> /dev/null; then
    echo "Node not found in PATH, running setup-node.sh..."
    "$ROOT_DIR/setup-node.sh"
fi

echo "Node: $(node -v), NPM: $(npm -v)"
cd "$ROOT_DIR/frontend"

if [ ! -d "node_modules" ]; then
    echo "Installing frontend dependencies..."
    npm install
fi

if [ ! -f "$ROOT_DIR/bin/tailwindcss" ]; then
    echo "Downloading standalone Tailwind CLI..."
    mkdir -p "$ROOT_DIR/bin"
    curl -sL https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 -o "$ROOT_DIR/bin/tailwindcss"
    chmod +x "$ROOT_DIR/bin/tailwindcss"
fi

echo "Compiling Tailwind CSS..."
"$ROOT_DIR/bin/tailwindcss" -i "$ROOT_DIR/styles/input.css" -o "$ROOT_DIR/frontend/src/tailwind.css" --minify

echo "Building Svelte 5 frontend with Vite..."
npm run build
echo "Frontend build complete: frontend/dist/"

