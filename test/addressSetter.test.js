const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Setter address validation", function () {
  let owner, nonOwner, goat, meat, nft;

  beforeEach(async function () {
    [owner, nonOwner] = await ethers.getSigners();
    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy(goat.target);
    await meat.waitForDeployment();

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy(goat.target);
    await nft.waitForDeployment();
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

  it("emits MeatAddressUpdated when MEAT address changes", async function () {
    await expect(goat.setMEATAddress(meat.target))
      .to.emit(goat, "MeatAddressUpdated")
      .withArgs(owner.address, meat.target);
  });

  it("emits NftAddressUpdated when NFT address changes", async function () {
    await expect(goat.setNFTAddress(nft.target))
      .to.emit(goat, "NftAddressUpdated")
      .withArgs(ethers.ZeroAddress, nft.target);
  });

  it("emits GoatAddressUpdated when GOAT address changes", async function () {
    const GOAT = await ethers.getContractFactory("GOAT");
    const newGoat = await GOAT.deploy(owner.address);
    await newGoat.waitForDeployment();

    await expect(meat.setGOATAddress(newGoat.target))
      .to.emit(meat, "GoatAddressUpdated")
      .withArgs(goat.target, newGoat.target);
  });
});
