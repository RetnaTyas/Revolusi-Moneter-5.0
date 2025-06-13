const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("NFT burn reentrancy protection", function () {
  let owner, user, nft, sapi, goatHook, sapiHook;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy();
    await nft.waitForDeployment();

    const SapiNFT = await ethers.getContractFactory("SapiNFT");
    sapi = await SapiNFT.deploy();
    await sapi.waitForDeployment();

    const MaliciousGoat = await ethers.getContractFactory("MaliciousGoatHook");
    goatHook = await MaliciousGoat.deploy(nft.target);
    await goatHook.waitForDeployment();

    const MaliciousSapi = await ethers.getContractFactory("MaliciousSapiHook");
    sapiHook = await MaliciousSapi.deploy(sapi.target);
    await sapiHook.waitForDeployment();

    await nft.setBurnHook(goatHook.target);
    await sapi.setBurnHook(sapiHook.target);
  });

  it("clears state before external hook (GoatNFT)", async function () {
    const tx = await nft.mint(user.address, 500, "reent", "Boer", 2023);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await nft.connect(user).updateWeight(tokenId, 500n);

    await nft.connect(user).approve(goatHook.target, tokenId);
    await goatHook.setTargetTokenId(tokenId);

    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "GoatBurned")
      .withArgs(tokenId, user.address, 500n);

    await expect(nft.ownerOf(tokenId)).to.be.reverted;
  });

  it("clears state before external hook (SapiNFT)", async function () {
    const tx = await sapi.mint(user.address, 700, "reentS", "Brahman", 2023);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await sapi.connect(user).updateWeight(tokenId, 700n);

    await sapi.connect(user).approve(sapiHook.target, tokenId);
    await sapiHook.setTargetTokenId(tokenId);

    await expect(sapi.connect(user).burn(tokenId))
      .to.emit(sapi, "SapiBurned")
      .withArgs(tokenId, user.address, 700n);

    await expect(sapi.ownerOf(tokenId)).to.be.reverted;
  });
});
