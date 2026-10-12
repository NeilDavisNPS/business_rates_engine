#!/usr/bin/env bash
set -e
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="$ROOT_DIR/bin:$PATH"

if ! command -v node >/dev/null 2>&1 || ! command -v npx >/dev/null 2>&1; then
    if [ -x "$ROOT_DIR/setup-node.sh" ]; then
        "$ROOT_DIR/setup-node.sh"
    fi
fi

if ! command -v npx >/dev/null 2>&1; then
    echo "Error: Node.js/npx is required to generate the Software Bill of Materials (SBOM)." >&2
    exit 1
fi

echo "Generating CycloneDX Software Bill of Materials (SBOM)..."
cd "$ROOT_DIR"
npx --yes @cyclonedx/cdxgen -o "$ROOT_DIR/sbom.cdx.json"
echo "SBOM successfully generated: $ROOT_DIR/sbom.cdx.json"

