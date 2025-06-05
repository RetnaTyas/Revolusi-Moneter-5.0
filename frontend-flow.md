# Frontend Flow

The web interface exposes simple steps for interacting with GOAT and MEAT. It expects the user to connect a wallet compatible with the deployed network.

1. **Mint MEAT** – send native currency from the wallet directly to the MEAT contract via the UI. The frontend watches the `MintedWithNative` event and updates the MEAT balance.
2. **Swap MEAT for GOAT** – approve MEAT to the contract and call `swapMEATForGOAT`. The resulting GOAT is displayed once the transaction is mined.
3. **Stake GOAT** – with GOAT in hand, users can stake any amount. The UI shows pending reward through `pendingReward` and the next time rewards can be claimed.
4. **Claim or Compound** – after the claim interval the UI enables the `claimReward` and `compoundReward` buttons. Compounding reinvests the reward into the staking balance.
5. **Unstake and Swap Back** – users may unstake to withdraw principal plus reward, then swap GOAT back to MEAT using `swapGOATForMEAT`.

All state updates come from contract calls or events so balances remain consistent with on‑chain data.
