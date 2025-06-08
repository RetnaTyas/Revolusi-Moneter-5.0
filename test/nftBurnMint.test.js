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
    const weight = 700;
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
    const newWeight = 800n;
    await nft.connect(user).updateWeight(tokenId, newWeight);

    const stored = await nft.getGoatData(tokenId);
    expect(stored.weight).to.equal(newWeight);

    const goatAmount = (newWeight * 10n ** 18n) / SWAP_RATE / 10n;
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, newWeight, goatAmount);

    expect(await goat.balanceOf(user.address)).to.equal(goatAmount);
    await expect(nft.ownerOf(tokenId)).to.be.reverted;
  });

  it("mints 0.5 GOAT when burning weight 425", async function () {
    const tx = await nft.mint(user.address, 425, "half", "Boer", 2021);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    const expected = (425n * 10n ** 18n) / SWAP_RATE / 10n;
    // weight already fresh so just burn
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, 425n, expected);
    expect(expected).to.equal(5n * 10n ** 17n);
    expect(await goat.balanceOf(user.address)).to.equal(expected);
  });

  it("emits WeightUpdated when updating weight", async function () {
    const tx = await nft.mint(user.address, 500, "tag", "type", 2022);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    await expect(nft.connect(user).updateWeight(tokenId, 550n))
      .to.emit(nft, "WeightUpdated")
      .withArgs(tokenId, 550n);
  });

  it("reverts burn when weight update is stale", async function () {
    const tx = await nft.mint(user.address, 600, "n", "b", 2020);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    // fast forward beyond validity window
    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    await expect(nft.connect(user).burn(tokenId)).to.be.revertedWith(
      "Weight update too old"
    );
  });

  it("reverts when minting with duplicate nfcId", async function () {
    await nft.mint(user.address, 400, "dup", "Boer", 2022);
    await expect(
      nft.mint(user.address, 500, "dup", "Boer", 2022)
    ).to.be.revertedWith("NFC ID already used");
  });

  it("allows reusing nfcId after burn", async function () {
    const tx = await nft.mint(user.address, 300, "reuse", "Boer", 2022);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];
    await nft.connect(user).burn(tokenId);
    await expect(nft.mint(user.address, 350, "reuse", "Boer", 2022)).to.not.be.reverted;
  });
});
