const { expect } = require("chai");
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

    const tx = await nft.mint(user.address, 50, "id", "breed", 2022);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];
    await nft.connect(user).updateWeight(tokenId, 60);

    const expectedGoat = (60n * 10n ** 18n) / 100n;
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, 60n, expectedGoat);

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
});
