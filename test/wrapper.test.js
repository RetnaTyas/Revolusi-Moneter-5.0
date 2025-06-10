const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GoatNFTWrapper", function () {
  let owner, user, goat, nft, wrapper, swapRate;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy();
    await goat.waitForDeployment();

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy();
    await nft.waitForDeployment();

    const Wrapper = await ethers.getContractFactory("GoatNFTWrapper");
    wrapper = await Wrapper.deploy(nft.target, goat.target);
    await wrapper.waitForDeployment();

    await goat.setWrapperContract(wrapper.target);

    swapRate = 85n;
  });

  it("wraps NFT and mints GOAT", async function () {
    const tx = await nft.mint(user.address, 500, "wrap", "Boer", 2021);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    await nft.connect(user).approve(wrapper.target, tokenId);

    const expected = (500n * 10n ** 18n) / swapRate / 10n;
    await expect(wrapper.connect(user).wrap(tokenId))
      .to.emit(wrapper, "Wrapped")
      .withArgs(user.address, tokenId, expected);

    expect(await nft.ownerOf(tokenId)).to.equal(wrapper.target);
    expect(await goat.balanceOf(user.address)).to.equal(expected);
  });

  it("unwraps and burns GOAT", async function () {
    const tx = await nft.mint(user.address, 600, "unwrap", "Boer", 2021);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];
    await nft.connect(user).approve(wrapper.target, tokenId);

    const amount = (600n * 10n ** 18n) / swapRate / 10n;
    await wrapper.connect(user).wrap(tokenId);

    await expect(wrapper.connect(user).unwrap(tokenId))
      .to.emit(wrapper, "Unwrapped")
      .withArgs(user.address, tokenId, amount);

    expect(await nft.ownerOf(tokenId)).to.equal(user.address);
    expect(await goat.balanceOf(user.address)).to.equal(0n);
  });

  it("reverts wrap when caller not NFT owner", async function () {
    const tx = await nft.mint(owner.address, 500, "tag", "Boer", 2020);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await nft.connect(owner).approve(wrapper.target, tokenId);

    await expect(wrapper.connect(user).wrap(tokenId)).to.be.revertedWith(
      "Not token owner"
    );
  });

  it("reverts unwrap when caller not token owner", async function () {
    const tx = await nft.mint(user.address, 600, "tag", "Boer", 2020);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await nft.connect(user).approve(wrapper.target, tokenId);
    await wrapper.connect(user).wrap(tokenId);

    await expect(wrapper.unwrap(tokenId)).to.be.revertedWith("Not owner");
  });

  it("owner() returns deployer", async function () {
    expect(await wrapper.owner()).to.equal(owner.address);
  });
});

