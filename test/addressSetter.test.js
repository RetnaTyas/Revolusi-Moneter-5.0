const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Setter address validation", function () {
  let owner, goat, meat;

  beforeEach(async function () {
    [owner] = await ethers.getSigners();
    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy(goat.target);
    await meat.waitForDeployment();
  });

  it("reverts when setting MEAT address to zero", async function () {
    await expect(goat.setMEATAddress(ethers.ZeroAddress)).to.be.revertedWith(
      "Invalid address"
    );
  });

  it("reverts when setting NFT address to zero", async function () {
    await expect(goat.setNFTAddress(ethers.ZeroAddress)).to.be.revertedWith(
      "Invalid address"
    );
  });

  it("reverts when setting GOAT address to zero", async function () {
    await expect(meat.setGOATAddress(ethers.ZeroAddress)).to.be.revertedWith(
      "Invalid address"
    );
  });
});
