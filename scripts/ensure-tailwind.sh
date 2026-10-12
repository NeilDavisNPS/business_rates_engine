#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
BIN_DIR="$ROOT_DIR/bin"
TAILWIND_BIN="$BIN_DIR/tailwindcss"
TAILWIND_VERSION="v4.3.3"
TAILWIND_SHA256="dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a"

mkdir -p "$BIN_DIR"

verify_checksum() {
    local target="$1"
    if [ ! -f "$target" ]; then
        return 1
    fi
    if command -v sha256sum >/dev/null 2>&1; then
        local sum
        sum="$(sha256sum "$target" | awk '{print $1}')"
        [ "$sum" = "$TAILWIND_SHA256" ]
    elif command -v shasum >/dev/null 2>&1; then
        local sum
        sum="$(shasum -a 256 "$target" | awk '{print $1}')"
        [ "$sum" = "$TAILWIND_SHA256" ]
    else
        [ -s "$target" ] && [ -x "$target" ]
    fi
}

if [ -f "$TAILWIND_BIN" ]; then
    if verify_checksum "$TAILWIND_BIN"; then
        exit 0
    else
        echo "Corrupt or outdated tailwindcss binary detected at $TAILWIND_BIN. Re-downloading..."
        rm -f "$TAILWIND_BIN"
    fi
fi

echo "Downloading standalone Tailwind CLI ($TAILWIND_VERSION)..."
TEMP_FILE="$BIN_DIR/tailwindcss.tmp.$$"
DOWNLOAD_URL="https://github.com/tailwindlabs/tailwindcss/releases/download/${TAILWIND_VERSION}/tailwindcss-linux-x64"

if ! curl -fSL "$DOWNLOAD_URL" -o "$TEMP_FILE"; then
    echo "Error: Failed to download Tailwind CLI from $DOWNLOAD_URL" >&2
    rm -f "$TEMP_FILE"
    exit 1
fi

if ! verify_checksum "$TEMP_FILE"; then
    echo "Error: Checksum verification failed for $TEMP_FILE" >&2
    rm -f "$TEMP_FILE"
    exit 1
fi

chmod +x "$TEMP_FILE"
mv "$TEMP_FILE" "$TAILWIND_BIN"
echo "Tailwind CLI verified and installed at $TAILWIND_BIN"

