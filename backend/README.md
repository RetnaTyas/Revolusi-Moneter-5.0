# Backend Server

This folder contains a small Express server that exposes cached stats for the GOAT and MEAT token contracts. The server reads on-chain data through an RPC provider and exposes JSON endpoints used by the frontend.

## Environment Variables

Copy `.env.example` to `.env` and fill the following values:

- `RPC_URL` – RPC endpoint to access the blockchain
- `GOAT_ADDRESS` – deployed GOAT contract address
- `MEAT_ADDRESS` – deployed MEAT contract address
- `PORT` – port the server should listen on

## Running

Install dependencies at the repository root and start the server with:

```bash
npm run start:server
```

For a full overview of the project architecture and contract interactions, see the main [README](../README.md).
