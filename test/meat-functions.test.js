const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MEAT core functions", function () {
  let owner, user, meat;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();
    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy();
    await meat.waitForDeployment();
  });


  it("redeemForMeat burns tokens and emits event", async function () {
    const subtype = ethers.encodeBytes32String("GOATMEAT");
    const minted = ethers.parseEther("1");
    await meat.mintSubtype(user.address, subtype, minted);

    await expect(meat.connect(user).redeemForMeat(minted))
      .to.emit(meat, "MeatRedeemed")
      .withArgs(user.address, minted);

    expect(await meat.balanceOf(user.address)).to.equal(0n);
  });

  it("setRateHandler onlyOwner and emits", async function () {
    const RateHandler = await ethers.getContractFactory("RateHandler");
    const handler = await RateHandler.deploy();
    await handler.waitForDeployment();

    await expect(meat.connect(user).setRateHandler(handler.target)).to.be.revertedWithCustomError(
      meat,
      "OwnableUnauthorizedAccount"
    ).withArgs(user.address);

    await expect(meat.setRateHandler(handler.target))
      .to.emit(meat, "RateHandlerUpdated")
      .withArgs(ethers.ZeroAddress, handler.target);
    expect(await meat.rateHandler()).to.equal(handler.target);
  });

});
