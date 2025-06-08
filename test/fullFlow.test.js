const { expect } = require("chai");
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
const { ethers } = require("hardhat");

describe("Full flow integration", function () {
  let owner, user1, user2, user3;
  let goat, meat, swapConfig, SWAP_RATE;

  beforeEach(async function () {
    [owner, user1, user2, user3] = await ethers.getSigners();
    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    swapConfig = await ethers.deployContract("SwapConfig");
    await swapConfig.waitForDeployment();
    SWAP_RATE = await swapConfig.SWAP_RATE();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy(goat.target);
    await meat.waitForDeployment();

    await goat.setMEATAddress(meat.target);

    const initial = ethers.parseEther("100");
    await meat.transfer(user1.address, initial);
    await meat.transfer(user2.address, initial);
    await meat.transfer(user3.address, initial);
  });

  it("runs through minting, staking and swapping", async function () {
    const deposit1 = ethers.parseEther("1");
    const minted1 = (deposit1 * 100n) / 1000n;
    await expect(user1.sendTransaction({ to: meat.target, value: deposit1 }))
      .to.emit(meat, "MintedWithNative")
      .withArgs(user1.address, deposit1, minted1);
    expect(await meat.balanceOf(user1.address)).to.equal(
      ethers.parseEther("100") + minted1
    );

    await expect(meat.changeDepositRate(200))
      .to.emit(meat, "DepositRateChanged")
      .withArgs(100, 200);
    expect(await meat.DepositRate()).to.equal(200);

    const deposit2 = ethers.parseEther("1");
    const minted2 = (deposit2 * 200n) / 1000n;
    await expect(user2.sendTransaction({ to: meat.target, value: deposit2 }))
      .to.emit(meat, "MintedWithNative")
      .withArgs(user2.address, deposit2, minted2);
    expect(await meat.balanceOf(user2.address)).to.equal(
      ethers.parseEther("100") + minted2
    );

    const totalNative = deposit1 + deposit2;
    await expect(meat.withdrawNative())
      .to.emit(meat, "NativeWithdrawn")
      .withArgs(owner.address, totalNative);
    expect(await ethers.provider.getBalance(meat.target)).to.equal(0n);

    await expect(meat.setSwapEnabled(false))
      .to.emit(meat, "SwapEnabledUpdated")
      .withArgs(false);
    const amountSwap1 = ethers.parseEther("10");
    await meat.connect(user1).approve(meat.target, amountSwap1);
    await expect(meat.connect(user1).swapMEATForGOAT(amountSwap1)).to.be.revertedWith(
      "Swap disabled"
    );
    await expect(meat.setSwapEnabled(true))
      .to.emit(meat, "SwapEnabledUpdated")
      .withArgs(true);

    const goatOut1 = amountSwap1 / SWAP_RATE;
    await expect(meat.connect(user1).swapMEATForGOAT(amountSwap1))
      .to.emit(meat, "SwappedMEATForGOAT")
      .withArgs(user1.address, amountSwap1, goatOut1);

    await expect(goat.connect(user1).stake(goatOut1))
      .to.emit(goat, "Staked")
      .withArgs(user1.address, goatOut1);
    expect(await goat.stakingBalance(user1.address)).to.equal(goatOut1);

    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    const reward1 = await goat.pendingReward(user1.address);
    await expect(goat.connect(user1).compoundReward())
      .to.emit(goat, "Compounded")
      .withArgs(user1.address, anyValue);
    const stakeAfterCompound = await goat.stakingBalance(user1.address);
    expect(stakeAfterCompound).to.be.gt(goatOut1);

    await expect(
      goat.setRewardConfig(1000000000, 365 * 24 * 60 * 60, 24 * 60 * 60)
    )
      .to.emit(goat, "RewardConfigChanged")
      .withArgs(
        5000000000000000000n,
        1000000000,
        365 * 24 * 60 * 60,
        365 * 24 * 60 * 60,
        604800,
        86400
      );

    await ethers.provider.send("evm_increaseTime", [2 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    const reward2 = await goat.pendingReward(user1.address);

    await expect(goat.connect(user1).unstake())
      .to.emit(goat, "Unstaked")
      .withArgs(user1.address, stakeAfterCompound, anyValue);
    expect(await goat.lastStakedTime(user1.address)).to.equal(0n);
    const goatReturn = await goat.balanceOf(user1.address);
    expect(goatReturn).to.be.gt(stakeAfterCompound);

    await goat.connect(user1).approve(meat.target, goatReturn);
    await expect(meat.connect(user1).swapGOATForMEAT(goatReturn))
      .to.emit(meat, "SwappedGOATForMEAT")
      .withArgs(user1.address, goatReturn, goatReturn * SWAP_RATE);

    const finalMeat1 =
      ethers.parseEther("100") +
      minted1 -
      amountSwap1 +
      goatReturn * SWAP_RATE;
    expect(await meat.balanceOf(user1.address)).to.equal(finalMeat1);
    expect(await goat.balanceOf(user1.address)).to.equal(0n);

    const amountSwap2 = ethers.parseEther("20");
    const goatOut2 = amountSwap2 / SWAP_RATE;
    await meat.connect(user2).approve(meat.target, amountSwap2);
    await expect(meat.connect(user2).swapMEATForGOAT(amountSwap2))
      .to.emit(meat, "SwappedMEATForGOAT")
      .withArgs(user2.address, amountSwap2, goatOut2);

    await goat.connect(user2).stake(goatOut2);
    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);
    const reward2a = await goat.pendingReward(user2.address);
    await expect(goat.connect(user2).claimReward())
      .to.emit(goat, "RewardClaimed")
      .withArgs(user2.address, anyValue);

    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);
    const reward2b = await goat.pendingReward(user2.address);
    await expect(goat.connect(user2).unstake())
      .to.emit(goat, "Unstaked")
      .withArgs(user2.address, goatOut2, anyValue);
    expect(await goat.lastStakedTime(user2.address)).to.equal(0n);

    const finalGoat2 = await goat.balanceOf(user2.address);
    expect(finalGoat2).to.be.gt(goatOut2);
    const finalMeat2 = ethers.parseEther("100") + minted2 - amountSwap2;
    expect(await meat.balanceOf(user2.address)).to.equal(finalMeat2);

    const amountSwap3 = ethers.parseEther("15");
    const goatOut3 = amountSwap3 / SWAP_RATE;
    await meat.connect(user3).approve(meat.target, amountSwap3);
    await meat.connect(user3).swapMEATForGOAT(amountSwap3);
    await goat.connect(user3).stake(goatOut3);
    await expect(goat.connect(user3).emergencyUnstake())
      .to.emit(goat, "EmergencyUnstaked")
      .withArgs(user3.address, goatOut3);

    expect(await goat.balanceOf(user3.address)).to.equal(goatOut3);
    expect(await goat.lastStakedTime(user3.address)).to.equal(0n);
    const finalMeat3 = ethers.parseEther("100") - amountSwap3;
    expect(await meat.balanceOf(user3.address)).to.equal(finalMeat3);
  });
});
