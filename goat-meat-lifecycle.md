# GOAT & MEAT Token Lifecycle

The two tokens form a closed loop that allows value to enter the system via MEAT and be rewarded through GOAT staking.

1. **Minting MEAT**
   * Users send native currency to the MEAT contract. It mints MEAT tokens to the sender at a ratio determined by `DepositRate`.
2. **Swapping**
   * MEAT can be swapped to GOAT and vice versa through the MEAT contract when `swapEnabled` is true. The fixed conversion constant `SwapRate` maintains proportional supply.
3. **Staking GOAT**
   * GOAT holders stake tokens in `GOAT.sol` which records staking balances and timestamps. Rewards accrue linearly according to `rewardRate` and `rewardInterval`.
4. **Claiming Rewards**
   * After `minClaimInterval` stakers may claim rewards or compound them back into the stake. If they choose to exit entirely they call `unstake` to receive the principal plus reward.
5. **Returning to MEAT**
   * Unstaked GOAT can be swapped back for MEAT which may then be withdrawn from the ecosystem or used again for future interactions.

This lifecycle ensures that every stage of participation is backed by explicit contract functions and transparent token flows.
