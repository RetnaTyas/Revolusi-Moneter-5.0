const hre = require("hardhat");

async function main() {
  // No constructor arguments are required for any contract
  const GOAT = await hre.ethers.getContractFactory("GOAT");
  const goat = await GOAT.deploy();
  await goat.waitForDeployment();

  const MEAT = await hre.ethers.getContractFactory("MEAT");
  const meat = await MEAT.deploy();
  await meat.waitForDeployment();

  // setMEATAddress is no longer needed since the contracts are decoupled
  console.log(`GOAT deployed to: ${goat.target}`);
  console.log(`MEAT deployed to: ${meat.target}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
