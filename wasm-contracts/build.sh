#!/bin/bash
set -e
DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$DIR"

for pkg in starter meat goatnft; do
  cd "$DIR/$pkg"
  cargo build --release --target wasm32-unknown-unknown
  mkdir -p "$DIR/../artifacts"
  cp target/wasm32-unknown-unknown/release/${pkg}.wasm "$DIR/../artifacts/"
  cargo schema --pkg $pkg --out-dir schema
  cd "$DIR"
done
