const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GOAT claim timing", function () {
  let owner, user, goat;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();
    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();
    const amount = ethers.parseEther("10");
    await goat.mintTo(user.address, amount);
  });

  it("reverts when claiming too early", async function () {
    const stakeAmt = await goat.balanceOf(user.address);
    await goat.connect(user).stake(stakeAmt);
    await expect(goat.connect(user).claimReward()).to.be.revertedWith(
      "Claim not allowed yet"
    );
  });

  it("allows claiming after the interval", async function () {
    const stakeAmt = await goat.balanceOf(user.address);
    await goat.connect(user).stake(stakeAmt);

    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    const before = await goat.lastStakedTime(user.address);
    await expect(goat.connect(user).claimReward()).to.not.be.reverted;
    const after = await goat.lastStakedTime(user.address);
    expect(after).to.be.gt(before);
  });
});
