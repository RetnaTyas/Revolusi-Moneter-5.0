const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BarterContract GOAT<->BEEF", function () {
  let owner, user, handler, meat, barter;
  const SUBTYPE_GOATMEAT = ethers.encodeBytes32String("GOATMEAT");
  const SUBTYPE_BEEFMEAT = ethers.encodeBytes32String("BEEFMEAT");

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

    const repGoat = {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: meat.target,
      tokenProductSubtype: SUBTYPE_GOATMEAT,
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: ethers.parseUnits("44.38", 18),
      lodPerDayVirtual: ethers.parseUnits("44.38", 18),
      lodPerDayProduct: ethers.parseUnits("44.38", 18),
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    };

    const repBeef = {
      nftAddress: ethers.ZeroAddress,
      tokenVirtualAddress: ethers.ZeroAddress,
      tokenProductAddress: meat.target,
      tokenProductSubtype: SUBTYPE_BEEFMEAT,
      isNftActive: true,
      isTokenVirtualActive: true,
      isTokenProductActive: true,
      lodPerDayNft: ethers.parseUnits("281.29", 18),
      lodPerDayVirtual: ethers.parseUnits("281.29", 18),
      lodPerDayProduct: ethers.parseUnits("281.29", 18),
      protein_g_per_kg: 1,
      fat_g_per_kg: 1,
      micronutrient_index_x1000: 1,
      yield_per_cycle_kg: 1,
      cycle_time_days: 1,
    };

    await handler.setCommodityRepresentation(SUBTYPE_GOATMEAT, repGoat);
    await handler.setCommodityRepresentation(SUBTYPE_BEEFMEAT, repBeef);

    await meat.mintSubtype(user.address, SUBTYPE_GOATMEAT, ethers.parseEther("10"));
  });

  it("barters GOATMEAT to BEEFMEAT correctly", async function () {
    const fromAmount = ethers.parseEther("2");
    await meat.connect(user).approve(barter.target, fromAmount);

    const rate = await handler.computeBarterRate(
      SUBTYPE_GOATMEAT,
      "PRODUCT",
      SUBTYPE_BEEFMEAT,
      "PRODUCT"
    );
    const expected = (fromAmount * rate) / 10n ** 18n;

    await expect(
      barter.connect(user).barterProductToProduct(
        SUBTYPE_GOATMEAT,
        SUBTYPE_BEEFMEAT,
        fromAmount
      )
    )
      .to.emit(barter, "BarterExecuted")
      .withArgs(user.address, SUBTYPE_GOATMEAT, fromAmount, SUBTYPE_BEEFMEAT, expected);

    expect(await meat.getBalanceOfSubtype(user.address, SUBTYPE_GOATMEAT)).to.equal(
      ethers.parseEther("8")
    );
    expect(await meat.getBalanceOfSubtype(user.address, SUBTYPE_BEEFMEAT)).to.equal(expected);
  });
});
