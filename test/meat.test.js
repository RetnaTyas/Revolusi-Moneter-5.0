const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MEAT", function () {
  it("should deploy and mint initial supply", async function () {
    const [owner] = await ethers.getSigners();
    const MEAT = await ethers.getContractFactory("MEAT");
    const meat = await MEAT.deploy();
    await meat.waitForDeployment();

    expect(await meat.owner()).to.equal(owner.address);
    const balance = await meat.balanceOf(owner.address);
    expect(balance).to.equal(ethers.parseEther("1000"));
  });

  it("reverts when sending no native token", async function () {
    const [owner] = await ethers.getSigners();

    const MEAT = await ethers.getContractFactory("MEAT");
    const meat = await MEAT.deploy();
    await meat.waitForDeployment();

    await expect(
      owner.sendTransaction({ to: meat.target, value: 0 })
    ).to.be.revertedWith("Must send Native Token to mint MEAT");
  });


  it("should burn tokens on redeem", async function () {
    const [owner, user] = await ethers.getSigners();

    const MEAT = await ethers.getContractFactory("MEAT");
    const meat = await MEAT.deploy();
    await meat.waitForDeployment();

    const amount = ethers.parseEther("50");
    await meat.transfer(user.address, amount);

    await expect(meat.connect(user).redeemForMeat(amount))
      .to.emit(meat, "MeatRedeemed")
      .withArgs(user.address, amount);

    expect(await meat.balanceOf(user.address)).to.equal(0n);
  });

});
