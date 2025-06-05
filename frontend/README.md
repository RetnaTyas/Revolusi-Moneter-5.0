# GOAT/MEAT Frontend

This directory contains the web interface used to interact with the GOAT and MEAT token contracts. The UI guides users through the typical lifecycle:

1. **Mint MEAT** by sending native currency to the MEAT contract.
2. **Swap MEAT ↔ GOAT** once `swapEnabled` is active.
3. **Stake GOAT** to begin earning rewards.
4. **Claim or Compound** rewards after the minimum interval.
5. **Unstake** to withdraw principal plus accrued GOAT.

## Development

Install dependencies and start a local dev server:

```bash
npm install
npm run dev
```

## Build

Create a production build with:

```bash
npm run build
```

For full contract information and backend instructions see the [root README](../README.md).
