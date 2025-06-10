const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GoatNFT burn", function () {
  let owner, user, goat, nft, SWAP_RATE;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy();
    await goat.waitForDeployment();

    SWAP_RATE = 85n;

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy();
    await nft.waitForDeployment();
  });

  it("burns NFT without minting GOAT", async function () {
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

    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, newWeight);

    expect(await goat.balanceOf(user.address)).to.equal(0n);
    await expect(nft.ownerOf(tokenId)).to.be.reverted;
  });

  it("emits GoatBurned event for weight 425 without minting", async function () {
    const tx = await nft.mint(user.address, 425, "half", "Boer", 2021);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    // weight already fresh so just burn
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, 425n);
    expect(await goat.balanceOf(user.address)).to.equal(0n);
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

  it("reverts updateWeight when caller not owner", async function () {
    const tx = await nft.mint(user.address, 400, "owner", "breed", 2022);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await expect(nft.updateWeight(tokenId, 450n)).to.be.revertedWith(
      "Not token owner"
    );
  });

  it("reverts burn when caller not owner", async function () {
    const tx = await nft.mint(user.address, 550, "ownr", "breed", 2022);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await nft.connect(user).updateWeight(tokenId, 560n);
    await expect(nft.burn(tokenId)).to.be.revertedWith("Not owner");
  });

  it("getGoatData cleared after burn", async function () {
    const tx = await nft.mint(user.address, 600, "clr", "Boer", 2022);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await nft.connect(user).updateWeight(tokenId, 610n);
    await nft.connect(user).burn(tokenId);
    const data = await nft.getGoatData(tokenId);
    expect(data.weight).to.equal(0n);
    expect(data.birthYear).to.equal(0n);
    expect(data.nfcId).to.equal("");
    expect(data.breed).to.equal("");
  });
});
