const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GOAT extra", function () {
  let owner, user, goat, failing;
  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();
    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy();
    await goat.waitForDeployment();
    await goat.setWrapperContract(owner.address);

    const Failing = await ethers.getContractFactory("FailingGOAT");
    failing = await Failing.deploy();
    await failing.waitForDeployment();
    await failing.setWrapperContract(owner.address);
  });

  it("setWrapperContract only owner", async function () {
    await expect(
      goat.connect(user).setWrapperContract(user.address)
    ).to.be.revertedWith("Not the owner");

    await expect(goat.setWrapperContract(user.address))
      .to.emit(goat, "WrapperAddressUpdated")
      .withArgs(owner.address, user.address);
    expect(await goat.wrapperContract()).to.equal(user.address);
  });

  it("stake and unstake returns reward", async function () {
    const amount = ethers.parseEther("10");
    await goat.mint(user.address, amount);

    await goat.connect(user).stake(amount);

    const seconds = 8 * 24 * 60 * 60;
    await ethers.provider.send("evm_increaseTime", [seconds]);
    await ethers.provider.send("evm_mine", []);

    const rate = await goat.rewardRate();
    const interval = await goat.rewardInterval();
    const expected =
      (amount * BigInt(seconds) * rate) / (interval * 10n ** 18n);
    const secReward = (amount * rate) / (interval * 10n ** 18n);

    const tx = await goat.connect(user).unstake();
    const receipt = await tx.wait();
    const event = receipt.logs.find((l) => l.eventName === "Unstaked");
    const emittedReward = event.args[2];

    expect(event.args[0]).to.equal(user.address);
    expect(event.args[1]).to.equal(amount);

    const diff =
      emittedReward > expected
        ? emittedReward - expected
        : expected - emittedReward;
    expect(diff).to.be.lte(secReward);

    expect(await goat.balanceOf(user.address)).to.equal(amount + emittedReward);
    expect(await goat.stakingBalance(user.address)).to.equal(0n);
  });

  it("emergencyUnstake mints when balance insufficient", async function () {
    const amount = ethers.parseEther("5");
    await failing.mint(user.address, amount);
    await failing.connect(user).stake(amount);
    await failing.burnFrom(failing.target, amount);

    await expect(failing.connect(user).emergencyUnstake())
      .to.emit(failing, "EmergencyUnstaked")
      .withArgs(user.address, amount);

    expect(await failing.balanceOf(user.address)).to.equal(amount);
  });

  it("pendingReward and nextClaimTime", async function () {
    const amount = ethers.parseEther("1");
    await goat.mint(user.address, amount);
    await goat.connect(user).stake(amount);

    const wait = 2 * 24 * 60 * 60;
    await ethers.provider.send("evm_increaseTime", [wait]);
    await ethers.provider.send("evm_mine", []);

    const rate = await goat.rewardRate();
    const interval = await goat.rewardInterval();
    const expected =
      (amount * BigInt(wait) * rate) / (interval * 10n ** 18n);
    expect(await goat.pendingReward(user.address)).to.equal(expected);

    const last = await goat.lastStakedTime(user.address);
    const min = await goat.minClaimInterval();
    expect(await goat.nextClaimTime(user.address)).to.equal(last + min);
  });

  it("setRewardConfig updates and emits", async function () {
    const oldRate = await goat.rewardRate();
    const oldInterval = await goat.rewardInterval();
    const oldMin = await goat.minClaimInterval();

    const newRate = ethers.parseEther("10");
    const newInterval = 100n;
    const newMin = 1n;
    await expect(goat.setRewardConfig(newRate, newInterval, newMin))
      .to.emit(goat, "RewardConfigChanged")
      .withArgs(oldRate, newRate, oldInterval, newInterval, oldMin, newMin);

    expect(await goat.rewardRate()).to.equal(newRate);
    expect(await goat.rewardInterval()).to.equal(newInterval);
    expect(await goat.minClaimInterval()).to.equal(newMin);
  });
});
