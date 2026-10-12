#!/usr/bin/env bash
set -e
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$ROOT_DIR/bin"

if command -v node >/dev/null 2>&1; then
    echo "Node is already available in PATH: $(command -v node) ($(node -v))"
    exit 0
fi

# Check for locally installed node versions (e.g. in ~/.local or ~/.nvm)
NODE_CANDIDATE=""
for cand in "$HOME/.local/node-v"*-linux-x64/bin/node "$HOME/.nvm/versions/node/"*/bin/node "$HOME/.local/bin/node"; do
    if [ -x "$cand" ]; then
        NODE_CANDIDATE="$(dirname "$cand")"
        break
    fi
done

if [ -n "$NODE_CANDIDATE" ]; then
    echo "Found local Node at $NODE_CANDIDATE, linking into $ROOT_DIR/bin..."
    ln -sf "$NODE_CANDIDATE/node" "$ROOT_DIR/bin/node"
    ln -sf "$NODE_CANDIDATE/npm" "$ROOT_DIR/bin/npm"
    ln -sf "$NODE_CANDIDATE/npx" "$ROOT_DIR/bin/npx"
    export PATH="$ROOT_DIR/bin:$PATH"
    echo "Node linked: $(node -v), NPM: $(npm -v)"
    exit 0
fi

echo "Error: Node.js was not found in PATH or in common local directories (~/.local, ~/.nvm)." >&2
echo "Please install Node.js 18+ or 20+ (LTS) to build the frontend." >&2
echo "See INSTALL_AND_RUN.md for prerequisites." >&2
exit 1

