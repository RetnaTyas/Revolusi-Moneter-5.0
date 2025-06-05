#!/bin/bash
set -e
cd "$(dirname "$0")/starter"
cargo build --release --target wasm32-unknown-unknown
mkdir -p ../../artifacts
cp target/wasm32-unknown-unknown/release/starter.wasm ../../artifacts/
cargo schema --pkg starter --out-dir schema
