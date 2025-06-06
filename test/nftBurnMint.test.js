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
    const nfcId = "1234";
    const breed = "Boer";
    const birthYear = 2021;
    const weight = 70;
    const tx = await nft.mint(user.address, value, nfcId, breed, birthYear, weight);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2]; // Transfer event

    const stored = await nft.getGoatData(tokenId);
    expect(stored.nfcId).to.equal(nfcId);
    expect(stored.breed).to.equal(breed);
    expect(stored.birthYear).to.equal(birthYear);
    expect(stored.weight).to.equal(weight);
    expect(stored.mintedAt).to.be.gt(0);

    await nft.connect(user).approve(goat.target, tokenId);
    await expect(goat.connect(user).burnAndMint(tokenId)).to.not.be.reverted;

    const afterBurn = await nft.getGoatData(tokenId);
    expect(afterBurn.nfcId).to.equal("");
  
    expect(await goat.balanceOf(user.address)).to.equal(value);
    await expect(nft.ownerOf(tokenId)).to.be.reverted;
  });
});
