#!/usr/bin/env bash
# ==============================================================================
# Business Rates Engine — Scalar API Reference Upgrade Tool
#
# Downloads and installs the standalone bundle of @scalar/api-reference from npm.
#
# Note:
# All accessibility and autofill DOM compliance (label association, unique field
# IDs/names) is decoupled and handled by frontend/public/assets/scalar-guardian.js.
# Therefore, vendor scalar.js can be upgraded cleanly without manual patching.
#
# Usage:
#   ./scripts/update-scalar.sh [version]
#
# Example:
#   ./scripts/update-scalar.sh latest
# ==============================================================================
set -euo pipefail

VERSION="${1:-latest}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="$ROOT_DIR/frontend/public/assets/scalar.js"

echo "Fetching @scalar/api-reference standalone bundle (version: $VERSION)..."
URL="https://cdn.jsdelivr.net/npm/@scalar/api-reference@${VERSION}/dist/browser/standalone.js"

curl -fSL "$URL" -o "$TARGET"
echo "Saved clean vendor bundle to $TARGET"

echo "Rebuilding frontend distribution bundle..."
"$ROOT_DIR/build-frontend.sh"

echo "Running integration tests to verify asset serving and CSP..."
cargo test --test integration_tests suite_d_api_and_parameters suite_i_ui_and_security

echo "Scalar upgrade completed and verified successfully."

