# GOAT and MEAT Token Contracts

This repository contains two ERC20 tokens:

- **GOAT** (Guardian of Agricultural Trade) supports staking and compounding rewards. The designated MEAT contract may mint new GOAT tokens while holders can stake their balance to earn a high annualised reward.
- **MEAT** (Market-Enabled Agricultural Token) lets users mint tokens with native currency and swap to and from GOAT, acting as the on‑ramp to the ecosystem.

## Deployment

1. Install [Hardhat](https://hardhat.org/) and initialise a project:
   ```bash
   npm install --save-dev hardhat
   npx hardhat init
   ```
2. Install the OpenZeppelin contracts used by GOAT:
   ```bash
   npm install @openzeppelin/contracts
   ```
3. Compile the included contracts:
   ```bash
   npx hardhat compile
   ```
4. Deploy the contracts with your preferred Hardhat network configuration. A simple script might look like:
   ```javascript
   const GOAT = await ethers.getContractFactory('GOAT');
   const goat = await GOAT.deploy(meatAddress);
   await goat.deployed();
   console.log('GOAT deployed to:', goat.address);
   ```
   Run using `npx hardhat run scripts/deploy.js --network <network>`.

## Running Tests

Hardhat tests for both contracts live in the `test/` directory. Run them with:

```bash
npx hardhat test
```
