# User Journey

This document outlines a typical experience for a new participant in the GOAT/MEAT ecosystem.

1. **Acquire MEAT**
   * The user opens the web app, connects their wallet and sends a small amount of native token to the MEAT contract. The contract mints MEAT based on the current `DepositRate`.
2. **Convert to GOAT**
   * Using the swap interface they approve MEAT and call `swapMEATForGOAT` which transfers MEAT and returns GOAT at the fixed `SWAP_RATE`.
3. **Stake for Rewards**
   * With GOAT tokens in the wallet the user stakes them to start accruing rewards. The UI shows how long until claiming is allowed (`minClaimInterval`).
4. **Harvest**
   * After the interval passes they may claim the reward directly or choose to compound it back into their stake, increasing the principal.
5. **Exit**
   * When finished they unstake to withdraw the original GOAT plus any reward and can immediately swap back to MEAT. MEAT can then be traded or held.

Through this cycle users interact with both contracts only via the approved functions which keeps funds secure yet flexible.
