const { expect } = require("chai");
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
const { ethers } = require("hardhat");

describe("RateHandler integration", function () {
  let owner, user, handler;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();


    const RateHandler = await ethers.getContractFactory("RateHandler");
    handler = await RateHandler.deploy();
    await handler.waitForDeployment();
  });


  it("emits RateUpdated and updates timestamp", async function () {
    const before = await handler.lastUpdateTimestamp();
    expect(before).to.equal(0n);

    const tx = await handler.updateRate(150);
    const receipt = await tx.wait();
    const block = await ethers.provider.getBlock(receipt.blockNumber);

    await expect(tx)
      .to.emit(handler, "RateUpdated")
      .withArgs(150n, BigInt(block.timestamp));

    expect(await handler.lastUpdateTimestamp()).to.equal(BigInt(block.timestamp));
  });

  it("emits RateInvalidated after invalidateRate", async function () {
    await handler.updateRate(200);
    const tx = await handler.invalidateRate();
    const receipt = await tx.wait();
    const block = await ethers.provider.getBlock(receipt.blockNumber);

    await expect(tx)
      .to.emit(handler, "RateInvalidated")
      .withArgs(BigInt(block.timestamp));

    expect(await handler.dynamicRateValid()).to.be.false;
  });

  it("allows ownership transfer and restricts owner-only methods", async function () {
    await expect(handler.transferOwnership(user.address))
      .to.emit(handler, "OwnershipTransferred")
      .withArgs(owner.address, user.address);

    await expect(handler.updateRate(300)).to.be.revertedWith("Not the owner");

    await expect(handler.connect(user).updateRate(300))
      .to.emit(handler, "RateUpdated")
      .withArgs(300n, anyValue);
  });

  it("stores and reads commodity LOD per layer", async function () {
    const wheat = ethers.encodeBytes32String("WHEAT");
    const data = {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: ethers.ZeroAddress,
      tokenProductSubtype: ethers.encodeBytes32String("WHEATMEAT"),
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: 2,
      lodPerDayVirtual: 3,
      lodPerDayProduct: 4,
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    };
    await handler.setCommodityRepresentation(wheat, data);
    expect(
      await handler["getLODPerDay(bytes32,string)"](wheat, "NFT")
    ).to.equal(2n);
    expect(
      await handler["getLODPerDay(bytes32,string)"](wheat, "VIRTUAL")
    ).to.equal(3n);
    expect(
      await handler["getLODPerDay(bytes32,string)"](wheat, "PRODUCT")
    ).to.equal(4n);
  });

  it("computes barter rate for PRODUCT to PRODUCT", async function () {
    const wheat = ethers.encodeBytes32String("WHEAT");
    const rice = ethers.encodeBytes32String("RICE");
    await handler.setCommodityRepresentation(wheat, {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: ethers.ZeroAddress,
      tokenProductSubtype: ethers.encodeBytes32String("WHEATMEAT"),
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: 2,
      lodPerDayVirtual: 3,
      lodPerDayProduct: 4,
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    });
    await handler.setCommodityRepresentation(rice, {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: ethers.ZeroAddress,
      tokenProductSubtype: ethers.encodeBytes32String("RICEMEAT"),
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: 1,
      lodPerDayVirtual: 5,
      lodPerDayProduct: 10,
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    });

    const rate = await handler[
      "computeBarterRate(bytes32,string,bytes32,string)"
    ](wheat, "PRODUCT", rice, "PRODUCT");
    expect(rate).to.equal((4n * 10n ** 18n) / 10n);
  });

  it("reverts when layers are not PRODUCT", async function () {
    const barley = ethers.encodeBytes32String("BARLEY");
    await handler.setCommodityRepresentation(barley, {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: ethers.ZeroAddress,
      tokenProductSubtype: ethers.encodeBytes32String("BARLEYPROD"),
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: 2,
      lodPerDayVirtual: 4,
      lodPerDayProduct: 8,
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    });

    await expect(
      handler["computeBarterRate(bytes32,string,bytes32,string)"](
        barley,
        "NFT",
        barley,
        "VIRTUAL"
      )
    ).to.be.revertedWith("FROM layer must be PRODUCT");

    await expect(
      handler["computeBarterRate(bytes32,string,bytes32,string)"](
        barley,
        "PRODUCT",
        barley,
        "VIRTUAL"
      )
    ).to.be.revertedWith("TO layer must be PRODUCT");
  });
});

describe("RateHandler - LOD Engine v1.0", function () {
  let rateHandler;

  before(async function () {
    const RateHandlerFactory = await ethers.getContractFactory("RateHandler");
    rateHandler = await RateHandlerFactory.deploy();
    await rateHandler.waitForDeployment();

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
  });

  it("returns correct LOD for NFT layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");
    const lodNft = await rateHandler["getLODPerDay(bytes32,string)"](commodityId, "NFT");
    expect(lodNft).to.equal(ethers.parseUnits("44.52", 18));
  });

  it("returns correct LOD for VIRTUAL layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");
    const lodVirtual = await rateHandler["getLODPerDay(bytes32,string)"](commodityId, "VIRTUAL");
    expect(lodVirtual).to.equal(ethers.parseUnits("44.52", 18));
  });

  it("returns correct LOD for PRODUCT layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");
    const lodProduct = await rateHandler["getLODPerDay(bytes32,string)"](commodityId, "PRODUCT");
    expect(lodProduct).to.equal(ethers.parseUnits("44.52", 18));
  });

  it("computes correct barter rate between same LOD", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");
    const rate = await rateHandler["computeBarterRate(bytes32,string,bytes32,string)"](commodityId, "PRODUCT", commodityId, "PRODUCT");
    expect(rate).to.equal(ethers.parseUnits("1", 18));
  });

  it("reverts on invalid layer", async function () {
    const commodityId = ethers.encodeBytes32String("KAMBING");
    await expect(
      rateHandler["getLODPerDay(bytes32,string)"](commodityId, "INVALID_LAYER")
    ).to.be.revertedWith("Invalid layer");
  });
});
