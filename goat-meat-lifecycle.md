# GOAT & MEAT Token Lifecycle

The two tokens form a closed loop that allows value to enter the system via MEAT and be rewarded through GOAT staking.

1. **Minting MEAT**
   * Users send native currency to the MEAT contract. Its `receive()` function mints MEAT to the sender using the `DepositRate`, scaled per 1000 units (default `100`, i.e. 100 MEAT per 1000 native).
   * The contract emits `MintedWithNative(user, nativeReceived, meatMinted)` recording who minted and how much native token was received.
2. **Swapping**
   * MEAT can be swapped to GOAT and vice versa through the MEAT contract when `swapEnabled` is true. The fixed conversion constant `SwapRate` maintains proportional supply.
3. **GoatNFTs**
   * A [GoatNFT](contracts/GoatNFT.sol) represents a live goat and stores its current weight in `goatValue`.
   * Owners may update the weight anytime. Before burning the NFT the weight must have been updated within the last seven days. `burn` mints GOAT automatically and emits `GoatBurned`.
4. **Staking GOAT**
   * GOAT holders stake tokens in `GOAT.sol` which records staking balances and timestamps. Rewards accrue linearly according to `rewardRate` and `rewardInterval`.
   *Calling `stake()` again resets `lastStakedTime` and discards any pending reward. Claim your reward first if you plan to restake.*
5. **Claiming Rewards**
   * After `minClaimInterval` stakers may claim rewards or compound them back into the stake. If they choose to exit entirely they call `unstake` to receive the principal plus reward.
6. **Returning to MEAT**
   * Unstaked GOAT can be swapped back for MEAT which may then be withdrawn from the ecosystem or used again for future interactions.

This lifecycle ensures that every stage of participation is backed by explicit contract functions and transparent token flows.
