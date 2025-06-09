const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("RateHandler - LOD Engine v1.0", function () {
  let rateHandler;

  before(async function () {
    const RateHandlerFactory = await ethers.getContractFactory("RateHandler");
    rateHandler = await RateHandlerFactory.deploy();
    await rateHandler.waitForDeployment();

    console.log("RateHandler deployed at:", rateHandler.target);

    const commodityId = ethers.encodeBytes32String("KAMBING");

    const data = {
      nftAddress: "0x0000000000000000000000000000000000000001",
      tokenVirtualAddress: "0x0000000000000000000000000000000000000002",
      tokenProductAddress: "0x0000000000000000000000000000000000000003",
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

    const tx = await rateHandler.setCommodityRepresentation(commodityId, data);
    await tx.wait();

    console.log("✅ CommodityRepresentation set for KAMBING");
  });

  it("should return correct LOD for NFT layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");

    const lodNft = await rateHandler["getLODPerDay(bytes32,string)"](commodityId, "NFT");
    expect(lodNft).to.equal(ethers.parseUnits("44.52", 18));
  });

  it("should return correct LOD for VIRTUAL layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");

    const lodVirtual = await rateHandler["getLODPerDay(bytes32,string)"](commodityId, "VIRTUAL");
    expect(lodVirtual).to.equal(ethers.parseUnits("44.52", 18));
  });

  it("should return correct LOD for PRODUCT layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");

    const lodProduct = await rateHandler["getLODPerDay(bytes32,string)"](commodityId, "PRODUCT");
    expect(lodProduct).to.equal(ethers.parseUnits("44.52", 18));
  });

  it("should compute correct barter rate between same LOD (identity check)", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");

    const rate = await rateHandler["computeBarterRate(bytes32,string,bytes32,string)"](
      commodityId,
      "PRODUCT",
      commodityId,
      "PRODUCT"
    );

    expect(rate).to.equal(ethers.parseUnits("1", 18));
  });

  it("should revert on invalid layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");

    await expect(
      rateHandler["getLODPerDay(bytes32,string)"](commodityId, "INVALID_LAYER")
    ).to.be.revertedWith("Invalid layer");
  });
});
