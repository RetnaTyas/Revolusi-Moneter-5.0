const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Setter address validation", function () {
  let owner, nonOwner, goat, meat;

  beforeEach(async function () {
    [owner, nonOwner] = await ethers.getSigners();
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

  it("reverts when non-owner calls setMEATAddress", async function () {
    await expect(
      goat.connect(nonOwner).setMEATAddress(nonOwner.address)
    ).to.be.revertedWith("Not the owner");
  });

  it("reverts when non-owner calls setNFTAddress", async function () {
    await expect(
      goat.connect(nonOwner).setNFTAddress(nonOwner.address)
    ).to.be.revertedWith("Not the owner");
  });

  it("reverts when non-owner calls setGOATAddress", async function () {
    await expect(
      meat.connect(nonOwner).setGOATAddress(goat.target)
    ).to.be.revertedWith("Not the owner");
  });
});
