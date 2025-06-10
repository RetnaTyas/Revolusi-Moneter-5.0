const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GoatNFT owner restrictions", function () {
  let owner, nonOwner, nft;

  beforeEach(async function () {
    [owner, nonOwner] = await ethers.getSigners();
    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy();
    await nft.waitForDeployment();
  });

  it("reverts when a non-owner calls mint", async function () {
    await expect(
      nft
        .connect(nonOwner)
        .mint(nonOwner.address, 50, "nfc", "breed", 2020)
    ).to.be.revertedWith("Not the owner");
  });

  it("owner() returns deployer", async function () {
    expect(await nft.owner()).to.equal(owner.address);
  });

  it("setBurnHook only owner", async function () {
    const addr = nonOwner.address;
    await expect(nft.connect(nonOwner).setBurnHook(addr)).to.be.revertedWith(
      "Not the owner"
    );
    await expect(nft.setBurnHook(addr))
      .to.emit(nft, "BurnHookUpdated")
      .withArgs(ethers.ZeroAddress, addr);
  });

  it("allows owner to mint", async function () {
    await expect(
      nft.mint(nonOwner.address, 50, "nfc", "breed", 2020)
    ).to.not.be.reverted;
  });
});
