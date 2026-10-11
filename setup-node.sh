#!/usr/bin/env bash
set -e
mkdir -p bin
ln -sf ~/.local/node-v20.18.0-linux-x64/bin/node bin/node
ln -sf ~/.local/node-v20.18.0-linux-x64/bin/npm bin/npm
ln -sf ~/.local/node-v20.18.0-linux-x64/bin/npx bin/npx
export PATH="$(pwd)/bin:$PATH"
echo "Node version: $(node -v)"
echo "NPM version: $(npm -v)"

