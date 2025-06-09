const { expect } = require("chai");
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
const { ethers } = require("hardhat");

describe("RateHandler integration", function () {
  let owner, user, goat, meat, nft, handler;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy(goat.target);
    await meat.waitForDeployment();
    await goat.setMEATAddress(meat.target);

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy(goat.target);
    await nft.waitForDeployment();
    await goat.setNFTAddress(nft.target);

    const RateHandler = await ethers.getContractFactory("RateHandler");
    handler = await RateHandler.deploy();
    await handler.waitForDeployment();
  });

  it("uses dynamic rate when valid", async function () {
    await meat.setRateHandler(handler.target);
    await nft.setRateHandler(handler.target);

    await handler.updateRate(100);

    const tx = await nft.mint(user.address, 500, "id", "breed", 2022);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];
    await nft.connect(user).updateWeight(tokenId, 600);

    const expectedGoat = (600n * 10n ** 18n) / 100n / 10n;
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, 600n, expectedGoat);

    await goat.connect(user).approve(meat.target, expectedGoat);
    await meat.connect(user).swapGOATForMEAT(expectedGoat);
    const expectedMeat = expectedGoat * 100n;
    expect(await meat.balanceOf(user.address)).to.equal(expectedMeat);
  });

  it("falls back to SwapConfig when invalid", async function () {
    await meat.setRateHandler(handler.target);
    await handler.updateRate(200);
    await handler.invalidateRate();

    const amount = ethers.parseEther("10");
    await meat.transfer(user.address, amount);
    await meat.connect(user).approve(meat.target, amount);

    const fallbackRate = 85n;
    const goatOut = amount / fallbackRate;
    await expect(meat.connect(user).swapMEATForGOAT(amount))
      .to.emit(meat, "SwappedMEATForGOAT")
      .withArgs(user.address, amount, goatOut);
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

  it("computes barter rate between layers", async function () {
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
    ](wheat, "NFT", rice, "PRODUCT");
    expect(rate).to.equal((2n * 10n ** 18n) / 10n);
  });

  it("computes barter rates across layer combinations", async function () {
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

    const rateNV = await handler[
      "computeBarterRate(bytes32,string,bytes32,string)"
    ](barley, "NFT", barley, "VIRTUAL");
    expect(rateNV).to.equal((2n * 10n ** 18n) / 4n);

    const rateVP = await handler[
      "computeBarterRate(bytes32,string,bytes32,string)"
    ](barley, "VIRTUAL", barley, "PRODUCT");
    expect(rateVP).to.equal((4n * 10n ** 18n) / 8n);

    const ratePP = await handler[
      "computeBarterRate(bytes32,string,bytes32,string)"
    ](barley, "PRODUCT", barley, "PRODUCT");
    expect(ratePP).to.equal(10n ** 18n);

    const rateNN = await handler[
      "computeBarterRate(bytes32,string,bytes32,string)"
    ](barley, "NFT", barley, "NFT");
    expect(rateNN).to.equal(10n ** 18n);
  });
});
