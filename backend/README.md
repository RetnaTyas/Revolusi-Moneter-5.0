# Backend Server

This folder contains a small Express server that exposes cached stats for the GOAT and MEAT token contracts. The server reads on-chain data through an RPC provider and exposes JSON endpoints used by the frontend.

## Setup

1. Install dependencies at the repository root:
   ```bash
   npm install
   ```
2. Compile the contracts to generate ABI files used by the server:
   ```bash
   npx hardhat compile
   ```
   Copy updated ABI JSONs from `artifacts/contracts/` into `backend/abi/` when you modify the contracts.
3. Copy the environment template and set the required variables:
   ```bash
   cp .env.example .env
   ```
   Configure `RPC_URL`, `GOAT_ADDRESS`, `MEAT_ADDRESS` and `PORT` in `backend/.env`.

## Running

Start the server with:

```bash
npm run start:server
```

For a full overview of the project architecture and contract interactions, see the main [README](../README.md).
