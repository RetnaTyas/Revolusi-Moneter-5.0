const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("SapiNFT burn", function () {
  let owner, user, nft;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const SapiNFT = await ethers.getContractFactory("SapiNFT");
    nft = await SapiNFT.deploy();
    await nft.waitForDeployment();
  });

  it("burns NFT and emits event", async function () {
    const tx = await nft.mint(user.address, 800, "c123", "Brahman", 2019);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    await nft.connect(user).updateWeight(tokenId, 850n);

    await expect(nft.connect(user).burn(tokenId))
      .to.emit(nft, "SapiBurned")
      .withArgs(tokenId, user.address, 850n);

    await expect(nft.ownerOf(tokenId)).to.be.reverted;
  });

  it("reverts burn when weight update is stale", async function () {
    const tx = await nft.mint(user.address, 600, "sapi", "Brahman", 2020);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    await ethers.provider.send("evm_increaseTime", [8 * 24 * 60 * 60]);
    await ethers.provider.send("evm_mine", []);

    await expect(nft.connect(user).burn(tokenId)).to.be.revertedWith(
      "Weight update too old"
    );
  });
});
