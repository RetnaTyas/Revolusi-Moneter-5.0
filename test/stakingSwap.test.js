const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GOAT staking and MEAT swap", function () {
  let owner, user;
  let goat, meat;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy(goat.target);
    await meat.waitForDeployment();

    await goat.setMEATAddress(meat.target);

    const initialMeat = ethers.parseEther("100");
    await meat.transfer(user.address, initialMeat);
  });

  it("should stake, claim, unstake and swap", async function () {
    const meatAmount = ethers.parseEther("100");
    await meat.connect(user).approve(meat.target, meatAmount);

    await meat.connect(user).swapMEATForGOAT(meatAmount);
    const goatAmount = await goat.balanceOf(user.address);
    expect(goatAmount).to.be.gt(0n);

    await goat.connect(user).stake(goatAmount);
    expect(await goat.stakingBalance(user.address)).to.equal(goatAmount);

    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    const pending = await goat.pendingReward(user.address);
    expect(pending).to.be.gt(0n);

    await goat.connect(user).claimReward();

    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    await goat.connect(user).unstake();
    expect(await goat.stakingBalance(user.address)).to.equal(0n);
    const afterUnstake = await goat.balanceOf(user.address);
    expect(afterUnstake).to.be.gt(goatAmount);

    await goat.connect(user).approve(meat.target, afterUnstake);
    await meat.connect(user).swapGOATForMEAT(afterUnstake);

    const finalMeat = await meat.balanceOf(user.address);
    expect(finalMeat).to.be.gt(meatAmount);
    expect(await goat.balanceOf(user.address)).to.equal(0n);
  });
});
