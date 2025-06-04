const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  const GOAT = await hre.ethers.getContractFactory("GOAT");
  const goat = await GOAT.deploy(deployer.address);
  await goat.waitForDeployment();
  console.log(`GOAT deployed to: ${goat.target}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
