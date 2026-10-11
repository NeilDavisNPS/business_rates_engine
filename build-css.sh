#!/usr/bin/env bash
set -e
if [ ! -f "bin/tailwindcss" ]; then
    echo "Downloading standalone Tailwind CLI..."
    mkdir -p bin
    curl -sL https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 -o bin/tailwindcss
    chmod +x bin/tailwindcss
fi

echo "Building production Tailwind CSS..."
./bin/tailwindcss -i ./styles/input.css -o ./frontend/src/tailwind.css --minify
echo "Tailwind build complete: frontend/src/tailwind.css ($(ls -lh frontend/src/tailwind.css | awk '{print $5}'))"

