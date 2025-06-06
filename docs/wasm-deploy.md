# Deploying the CosmWasm Contract

This project includes CosmWasm implementations for all core contracts under `wasm-contracts/`:

- `starter` – GOAT token with staking logic
- `meat` – MEAT token supporting swaps and native minting
- `goatnft` – simple NFT contract whose tokens hold a GOAT value

## Building

For toolchain setup and additional details see
[../wasm-contracts/README.md](../wasm-contracts/README.md).

Install the WASM target if you have not already:

```bash
rustup target add wasm32-unknown-unknown
```

Then run the build script:

```bash
./wasm-contracts/build.sh
```

The script compiles all packages and places their `.wasm` files under `artifacts/`. JSON schemas produced by `cargo schema` are stored in each package's `schema/` directory.
If `cargo schema` is not available, install it via `cargo install cargo-run-script` before running the build script.

## Upload & Instantiate

1. Upload the wasm bytecode:
```bash
# example upload of the GOAT contract
wasmd tx wasm store artifacts/starter.wasm --from wallet \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```
Save the resulting `code_id`.
2. Instantiate the contract (example for GOAT):
```bash
wasmd tx wasm instantiate <code_id> '{"meat_contract":"cosmos1..."}' \
  --from wallet --label "goat" \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```

Instantiate `meat` and `goatnft` with similar commands by providing the desired
`goat_contract` or no parameters for the NFT.

After deploying `goatnft`, authorize the GOAT contract to burn NFTs:

```bash
wasmd tx wasm execute <nft_address> '{"set_allowed_contract":{"contract":"<goat_addr>"}}' \
  --from wallet --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```

## Query Examples

```bash
# check balance
wasmd query wasm contract-state smart <address> '{"balance":{"address":"cosmos1..."}}'

# pending reward
wasmd query wasm contract-state smart <address> '{"pending_reward":{"address":"cosmos1..."}}'
```

Ensure your `wasmd` CLI is configured with a key named `wallet` that matches the mnemonic path in `deploy-config/wasm-config.json`.

