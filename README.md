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
3. Copy `contracts/GOAT.sol` into your project's `contracts` folder and compile:
   ```bash
   npx hardhat compile
   ```
4. Deploy the contracts with your preferred Hardhat network configuration. A simple script might look like:
   ```javascript
   const GOAT = await ethers.getContractFactory('GOAT');
   const goat = await GOAT.deploy(ethers.ZeroAddress);
   await goat.deployed();

   const MEAT = await ethers.getContractFactory('MEAT');
   const meat = await MEAT.deploy(goat.address);
   await meat.deployed();

   await goat.setMEATAddress(meat.address);

   console.log('GOAT deployed to:', goat.address);
   console.log('MEAT deployed to:', meat.address);
   ```
   Run using `npx hardhat run scripts/deploy.js --network <network>` and specify your desired Hardhat network with the `--network` option.

## Running Tests

Hardhat tests for both contracts live in the `test/` directory. Run them with:

```bash
npx hardhat test
```
