#!/bin/bash
set -e

echo "Building WASM-4 cart..."
cargo build --release --target wasm32-unknown-unknown

echo "Optimizing with wasm-opt..."
wasm-opt -Oz --strip-debug --strip-producers \
    --enable-bulk-memory --enable-sign-ext \
    target/wasm32-unknown-unknown/release/cart.wasm \
    -o target/wasm32-unknown-unknown/release/cart-opt.wasm

SIZE=$(wc -c < target/wasm32-unknown-unknown/release/cart-opt.wasm)
LIMIT=65535

echo ""
echo "Cart size: $SIZE bytes"
echo "Size limit: $LIMIT bytes"

if [ $SIZE -le $LIMIT ]; then
    echo "✓ Cart is within size limit!"
    cp target/wasm32-unknown-unknown/release/cart-opt.wasm cart.wasm
    echo "✓ Optimized cart saved as cart.wasm"
else
    echo "✗ Cart exceeds size limit by $((SIZE - LIMIT)) bytes"
    exit 1
fi
