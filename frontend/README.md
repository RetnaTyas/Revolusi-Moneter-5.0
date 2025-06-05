# Frontend

This folder holds the Next.js interface for interacting with the GOAT and MEAT contracts.

## Local Setup

1. Copy `.env.example` to `.env.local`:
   ```bash
   cp .env.example .env.local
   ```
2. Edit `.env.local` and set `NEXT_PUBLIC_GOAT_ADDRESS` and `NEXT_PUBLIC_MEAT_ADDRESS` to the deployed contract addresses.

Run the development server with `npm run dev` after installing dependencies.
