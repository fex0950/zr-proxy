#!/bin/bash

BINARY="./target/release/zr-proxy"

if [ ! -f "$BINARY" ]; then
    echo "Building zr-proxy..."
    cargo build --release
fi

exec "$BINARY" "$@"
