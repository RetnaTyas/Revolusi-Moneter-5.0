const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BarterContract", function () {
  let owner, user, handler, meat, barter;
  const SUBTYPE_A = ethers.encodeBytes32String("GOATMEAT");
  const SUBTYPE_B = ethers.encodeBytes32String("DUCKMEAT");

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const RateHandler = await ethers.getContractFactory("RateHandler");
    handler = await RateHandler.deploy();
    await handler.waitForDeployment();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy();
    await meat.waitForDeployment();

    const Barter = await ethers.getContractFactory("BarterContract");
    barter = await Barter.deploy(handler.target, meat.target);
    await barter.waitForDeployment();

    await meat.setMinter(owner.address, true);
    await meat.setMinter(barter.target, true);
    await meat.setBurner(barter.target, true);

    const repA = {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: meat.target,
      tokenProductSubtype: SUBTYPE_A,
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: 2,
      lodPerDayVirtual: 2,
      lodPerDayProduct: 4,
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    };

    const repB = {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: meat.target,
      tokenProductSubtype: SUBTYPE_B,
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: 1,
      lodPerDayVirtual: 1,
      lodPerDayProduct: 2,
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    };

    await handler.setCommodityRepresentation(SUBTYPE_A, repA);
    await handler.setCommodityRepresentation(SUBTYPE_B, repB);

    await meat.mintSubtype(user.address, SUBTYPE_A, ethers.parseEther("10"));
  });

  it("barters between product subtypes", async function () {
    const fromAmount = ethers.parseEther("2");
    await meat.setSubtypeLineage(user.address, SUBTYPE_A, 1);
    await meat.connect(user).approve(barter.target, fromAmount);

    const rate = await handler.computeBarterRate(SUBTYPE_A, "PRODUCT", SUBTYPE_B, "PRODUCT");
    const expected = (fromAmount * rate) / 10n ** 18n;

    await expect(barter.connect(user).barterProductToProduct(SUBTYPE_A, SUBTYPE_B, fromAmount))
      .to.emit(barter, "BarterExecuted")
      .withArgs(user.address, SUBTYPE_A, fromAmount, SUBTYPE_B, expected);

    expect(await meat.getBalanceOfSubtype(user.address, SUBTYPE_A)).to.equal(ethers.parseEther("8"));
    expect(await meat.getBalanceOfSubtype(user.address, SUBTYPE_B)).to.equal(expected);
  });

  it("reverts when lineage not set", async function () {
    const fromAmount = ethers.parseEther("1");
    await expect(barter.connect(user).barterProductToProduct(SUBTYPE_A, SUBTYPE_B, fromAmount))
      .to.be.revertedWith("Invalid lineage");
  });
});
