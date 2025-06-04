# GOAT Token Contract

GOAT (Guardian of Agricultural Trade) is an ERC20 token that supports staking and compounding rewards. The contract allows the designated MEAT contract to mint new tokens while holders can stake their balance to earn a high annualised reward.

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
3. Copy `GOAT.sol` into your project's `contracts` folder and compile:
   ```bash
   npx hardhat compile
   ```
4. Deploy the contract with your preferred Hardhat network configuration. A simple script might look like:
   ```javascript
   const GOAT = await ethers.getContractFactory('GOAT');
   const goat = await GOAT.deploy(meatAddress);
   await goat.deployed();
   console.log('GOAT deployed to:', goat.address);
   ```
   Run using `npx hardhat run scripts/deploy.js --network <network>`.

## Running Tests

This repository does not include tests, but Hardhat is recommended for writing them. Once your test files are created under `test/`, run:

```bash
npx hardhat test
```

or, if using another framework, run the equivalent command.
