const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GoatNFT burn and GOAT mint", function () {
  let owner, user, goat, nft, swapConfig, SWAP_RATE;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    swapConfig = await ethers.deployContract("SwapConfig");
    await swapConfig.waitForDeployment();
    SWAP_RATE = await swapConfig.SWAP_RATE();

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy(goat.target);
    await nft.waitForDeployment();

    await goat.setNFTAddress(nft.target);
  });

  it("burns NFT and mints GOAT", async function () {
    const nfcId = "1234";
    const breed = "Boer";
    const birthYear = 2021;
    const weight = 70;
    const tx = await nft.mint(
      user.address,
      weight,
      nfcId,
      breed,
      birthYear
    );
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    // owner updates weight before burning
    const newWeight = 80n;
    await nft.connect(user).updateWeight(tokenId, newWeight);

    const stored = await nft.getGoatData(tokenId);
    expect(stored.weight).to.equal(newWeight);

    const goatAmount = (newWeight * 10n ** 18n) / SWAP_RATE;
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, newWeight, goatAmount);

    expect(await goat.balanceOf(user.address)).to.equal(goatAmount);
    await expect(nft.ownerOf(tokenId)).to.be.reverted;
  });

  it("emits WeightUpdated when updating weight", async function () {
    const tx = await nft.mint(user.address, 50, "tag", "type", 2022);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    await expect(nft.connect(user).updateWeight(tokenId, 55n))
      .to.emit(nft, "WeightUpdated")
      .withArgs(tokenId, 55n);
  });

  it("reverts burn when weight update is stale", async function () {
    const tx = await nft.mint(user.address, 60, "n", "b", 2020);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    // fast forward beyond validity window
    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    await expect(nft.connect(user).burn(tokenId)).to.be.revertedWith(
      "Weight update too old"
    );
  });
});
