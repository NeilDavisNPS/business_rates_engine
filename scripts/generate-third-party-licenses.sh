#!/usr/bin/env bash
set -e
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="$ROOT_DIR/bin:$PATH"

if ! command -v node >/dev/null 2>&1; then
    if [ -x "$ROOT_DIR/setup-node.sh" ]; then
        "$ROOT_DIR/setup-node.sh"
    fi
fi

if ! command -v node >/dev/null 2>&1; then
    echo "Error: Node.js is required to generate third-party license notices." >&2
    exit 1
fi

echo "Generating THIRD_PARTY_LICENSES.md..."
node "$ROOT_DIR/scripts/generate-licenses.js"
echo "Done!"

