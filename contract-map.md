# Contract Map

```
[User Wallet]
    |  (native token)
    v
[MEAT] <----> [GOAT]
   |              ^
   |              |
   +-- withdraw -->|
```

* **MEAT** acts as the gateway: it accepts native coins, mints MEAT, and handles swaps in both directions. The owner may withdraw accumulated native balance.
* **GOAT** receives minted supply from MEAT and provides staking functionality. Rewards and configuration parameters are adjustable by the owner.
* **FailingGOAT** is only for testing; it implements the same interface but allows simulated transfer failures.
* **IGOAT** defines the `mintTo` function which enables MEAT to mint GOAT when necessary.

The MEAT contract relies on GOAT for minting new tokens when swapping MEAT for GOAT. Both share the same owner who can manage rates and enable or disable swapping. The table below summarises the main contracts and their roles.
| Contract | Description | Key Functions |
|---------|-------------|---------------|
| GOAT | ERC20 staking token minted by MEAT and GoatNFT burns. | `stake`, `unstake`, `claimReward`, `compoundReward`, `emergencyUnstake`, `mintTo`, `burnAndMint`, `setMEATAddress`, `setNFTAddress` |
| MEAT | ERC20 token minted with native deposits and swapped with GOAT. | `swapMEATForGOAT`, `swapGOATForMEAT`, `changeDepositRate`, `withdrawNative`, `setSwapEnabled`, `setGOATAddress` |
| GoatNFT | ERC721 goat identifier redeemable for GOAT. Metadata stored on-chain in `goatMetadata` as `GoatData` (`nfcId`, `breed`, `birthYear`, `weight`, `mintedAt`). | `mint`, `burn`, `goatValue`, `goatMetadata`, `getGoatData` |
| IGOAT | Interface for GOAT minting used by MEAT. | `mintTo` |
