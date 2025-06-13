#!/bin/bash
set -e
DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$DIR"

# Ensure the wasm target is installed
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
  echo "Installing wasm32-unknown-unknown target"
  rustup target add wasm32-unknown-unknown
fi

for pkg in starter meat goatnft ratehandler goatnftwrapper goatnftburnhook sapinft sapinftwrapper sapinftburnhook barterengine; do
  cd "$DIR/$pkg"
  cargo build --release --target wasm32-unknown-unknown
  mkdir -p "$DIR/../artifacts"
  cp target/wasm32-unknown-unknown/release/${pkg}.wasm "$DIR/../artifacts/"
  cargo schema --pkg $pkg --out-dir schema
  cd "$DIR"
done
