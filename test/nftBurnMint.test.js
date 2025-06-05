const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GoatNFT burn and GOAT mint", function () {
  let owner, user, goat, nft;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy();
    await nft.waitForDeployment();

    await goat.setNFTAddress(nft.target);
  });

  it("burns NFT and mints GOAT", async function () {
    const value = ethers.parseEther("5");
    const tx = await nft.mint(user.address, value);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2]; // Transfer event

    await nft.connect(user).approve(goat.target, tokenId);
    await expect(goat.connect(user).burnAndMint(tokenId)).to.not.be.reverted;

    expect(await goat.balanceOf(user.address)).to.equal(value);
    await expect(nft.ownerOf(tokenId)).to.be.reverted;
  });
});
