const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();

  const GOAT = await hre.ethers.getContractFactory("GOAT");
  const goat = await GOAT.deploy();
  await goat.waitForDeployment();

  const MEAT = await hre.ethers.getContractFactory("MEAT");
  const meat = await MEAT.deploy();
  await meat.waitForDeployment();

  console.log(`GOAT deployed to: ${goat.target}`);
  console.log(`MEAT deployed to: ${meat.target}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
