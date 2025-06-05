# Contract Map

| Contract | Description | Key Functions |
|---------|-------------|---------------|
| GOAT | ERC20 staking token minted by MEAT and GoatNFT burns. | `stake`, `unstake`, `claimReward`, `compoundReward`, `mintTo`, `burnAndMint`, `setMEATAddress`, `setNFTAddress` |
| MEAT | ERC20 token minted with native deposits and swapped with GOAT. | `swapMEATForGOAT`, `swapGOATForMEAT`, `changeDepositRate`, `withdrawNative`, `setSwapEnabled`, `setGOATAddress` |
| GoatNFT | ERC721 goat identifier redeemable for GOAT. | `mint`, `burn`, `goatValue` |
| IGOAT | Interface for GOAT minting used by MEAT. | `mintTo` |
