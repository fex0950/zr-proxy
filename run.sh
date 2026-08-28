#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "$0")" && pwd)"
BINARY="$SCRIPT_DIR/target/release/zr-proxy"

if [ ! -x "$BINARY" ]; then
    echo "Building zr-proxy..."
    (cd "$SCRIPT_DIR" && cargo build --release)
fi

exec "$BINARY" "$@"
