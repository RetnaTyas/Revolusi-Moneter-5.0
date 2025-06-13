const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Governance deployer:", deployer.address);

  const rateHandlerAddress = "0xYourRateHandlerContractAddress";
  const RateHandler = await ethers.getContractAt("RateHandler", rateHandlerAddress);

  // Example commodity data, adapt from lod_data.json
  const commodityId = ethers.encodeBytes32String("KAMBING");

  const data = {
    nftAddress: "0xGoatNFTAddress", // set real NFT address
    tokenVirtualAddress: "0xGOATTokenAddress", // GOAT token
    tokenProductAddress: "0xMEATTokenAddress", // MEAT token (GOATMEAT subtype)
    tokenProductSubtype: ethers.encodeBytes32String("GOATMEAT"),
    isNftActive: true,
    isTokenVirtualActive: true,
    isTokenProductActive: true,
    lodPerDayNft: ethers.parseUnits("44.52", 18),
    lodPerDayVirtual: ethers.parseUnits("44.52", 18),
    lodPerDayProduct: ethers.parseUnits("44.52", 18),
    protein_g_per_kg: 270,
    fat_g_per_kg: 200,
    micronutrient_index_x1000: 900,
    yield_per_cycle_kg: 25,
    cycle_time_days: 365,
  };

  const tx = await RateHandler.setCommodityRepresentation(commodityId, data);
  console.log("TX sent:", tx.hash);
  await tx.wait();
  console.log("\u2705 CommodityRepresentation updated for", commodityId);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
