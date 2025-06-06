# Frontend Pages

This document summarises the main pages expected in the full UI and how they map to contract functions. The pages are only mocked in the template under `frontend/` but the flow mirrors the behaviour covered by the Solidity tests.

## `/dashboard`
*Overview*: landing page showing overall stats such as total supply and total staked.
- **Data**: fetched from the backend `/stats` endpoint which internally calls `totalSupply()` on both tokens and `balanceOf()` on the GOAT contract address.
- **User Inputs**: none directly, only view.
- **Contract Calls**: none from the user; stats come from the backend.

## `/goat`
*Overview*: display of the user's GOAT token balance and the option to convert MEAT to GOAT.
- **User Inputs**: amount of MEAT to swap.
- **Contract Calls**: `swapMEATForGOAT` on `MEAT.sol` after approving the amount.

## `/stake`
*Overview*: interface for staking GOAT tokens and viewing pending rewards.
- **User Inputs**: GOAT amount to stake.
- **Contract Calls**: `stake(amount)` on `GOAT.sol`.

## `/rewards`
*Overview*: claim or compound staking rewards and unstake.
- **User Inputs**:
  - Choice between `claim`, `compound` or `unstake`.
- **Contract Calls**:
  - `claimReward()` – withdraw rewards without touching the principal.
  - `compoundReward()` – reinvest the reward into `stakingBalance`.
  - `unstake()` – withdraw principal plus reward after `minClaimInterval`.

## `/burn`
*Overview*: burn a `GoatNFT` to redeem its value in GOAT tokens.
- **User Inputs**: NFT `tokenId` to burn.
- **Contract Calls**: `burn(tokenId)` on `GoatNFT` which internally mints equivalent GOAT.

These pages align with the flows tested under `test/` such as `stakingSwap.test.js` and `nftBurnMint.test.js`, ensuring UI actions correspond to on-chain behaviour.
