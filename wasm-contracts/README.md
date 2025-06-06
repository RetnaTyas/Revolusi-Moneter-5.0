# Building the CosmWasm contracts

This folder contains the Rust implementations of the GOAT, MEAT and GoatNFT contracts. To compile them you need the standard Rust toolchain and the WebAssembly target.

## Prerequisites

1. Install [rustup](https://rustup.rs/) if you do not already have it.
2. Add the `wasm32-unknown-unknown` build target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Building

Run the build script from the repository root:
```bash
./wasm-contracts/build.sh
```
The script compiles all packages in this directory. The resulting `.wasm` files are placed in the top‑level `artifacts/` folder while JSON schemas are written to each package's `schema/` directory.
