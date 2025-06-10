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

  it("mints when sending native ETH", async function () {
    const deposit = ethers.parseEther("1");
    const expected = (deposit * 100n) / 1000n;

    await expect(user.sendTransaction({ to: meat.target, value: deposit }))
      .to.emit(meat, "MintedWithNative")
      .withArgs(user.address, deposit, expected);

    expect(await meat.balanceOf(user.address)).to.equal(expected);
  });

  it("withdrawNative onlyOwner and transfers balance", async function () {
    const deposit = ethers.parseEther("1");
    await user.sendTransaction({ to: meat.target, value: deposit });

    await expect(meat.connect(user).withdrawNative()).to.be.revertedWith(
      "Not the owner"
    );

    await expect(meat.withdrawNative())
      .to.emit(meat, "NativeWithdrawn")
      .withArgs(owner.address, deposit);

    expect(await ethers.provider.getBalance(meat.target)).to.equal(0n);
  });

  it("changeDepositRate updates rate and affects minting", async function () {
    expect(await meat.DepositRate()).to.equal(100n);

    await expect(meat.changeDepositRate(200))
      .to.emit(meat, "DepositRateChanged")
      .withArgs(100n, 200n);
    expect(await meat.DepositRate()).to.equal(200n);

    const deposit = ethers.parseEther("1");
    const expected = (deposit * 200n) / 1000n;

    await expect(user.sendTransaction({ to: meat.target, value: deposit }))
      .to.emit(meat, "MintedWithNative")
      .withArgs(user.address, deposit, expected);
    expect(await meat.balanceOf(user.address)).to.equal(expected);
  });

  it("redeemForMeat burns tokens and emits event", async function () {
    const deposit = ethers.parseEther("1");
    await user.sendTransaction({ to: meat.target, value: deposit });
    const minted = (deposit * 100n) / 1000n;

    await expect(meat.connect(user).redeemForMeat(minted))
      .to.emit(meat, "MeatRedeemed")
      .withArgs(user.address, minted);

    expect(await meat.balanceOf(user.address)).to.equal(0n);
  });

  it("setRateHandler onlyOwner and emits", async function () {
    const RateHandler = await ethers.getContractFactory("RateHandler");
    const handler = await RateHandler.deploy();
    await handler.waitForDeployment();

    await expect(meat.connect(user).setRateHandler(handler.target)).to.be.revertedWith(
      "Not the owner"
    );

    await expect(meat.setRateHandler(handler.target))
      .to.emit(meat, "RateHandlerUpdated")
      .withArgs(ethers.ZeroAddress, handler.target);
    expect(await meat.rateHandler()).to.equal(handler.target);
  });

  it("returns current DepositRate", async function () {
    const rate = await meat.DepositRate();
    expect(rate).to.equal(100n);
  });
});
