# Deploying the CosmWasm Contract

This project includes a basic CosmWasm implementation of the GOAT logic under `wasm-contracts/starter`.

## Building

```bash
./wasm-contracts/build.sh
```

The script compiles the contract and places the resulting `.wasm` file under `artifacts/` and exports the JSON schema under `wasm-contracts/starter/schema`.

## Upload & Instantiate

1. Upload the wasm bytecode:
   ```bash
   wasmd tx wasm store artifacts/starter.wasm --from wallet \
     --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
     --chain-id testing-1 --node https://rpc.testnet.cosmos.network
   ```
   Save the resulting `code_id`.
2. Instantiate the contract:
   ```bash
   wasmd tx wasm instantiate <code_id> '{"meat_contract":"cosmos1..."}' \
     --from wallet --label "goat" \
     --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
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

