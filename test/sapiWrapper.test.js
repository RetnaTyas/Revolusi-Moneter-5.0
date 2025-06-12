const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("SapiNFTWrapper", function () {
  let owner, user, goat, nft, wrapper, swapRate;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy();
    await goat.waitForDeployment();

    const SapiNFT = await ethers.getContractFactory("SapiNFT");
    nft = await SapiNFT.deploy();
    await nft.waitForDeployment();

    const Wrapper = await ethers.getContractFactory("SapiNFTWrapper");
    wrapper = await Wrapper.deploy(nft.target, goat.target);
    await wrapper.waitForDeployment();

    await goat.setWrapperContract(wrapper.target);

    swapRate = 595n;
  });

  it("wraps NFT and mints GOAT", async function () {
    const tx = await nft.mint(user.address, 500, "wrapS", "Brahman", 2021);
    const tokenId = (await tx.wait()).logs[0].args[2];

    await nft.connect(user).approve(wrapper.target, tokenId);

    const expected = (500n * 10n ** 18n) / swapRate / 10n;
    await expect(wrapper.connect(user).wrap(tokenId))
      .to.emit(wrapper, "Wrapped")
      .withArgs(user.address, tokenId, expected);

    expect(await nft.ownerOf(tokenId)).to.equal(wrapper.target);
    expect(await goat.balanceOf(user.address)).to.equal(expected);
  });

  it("unwraps and burns GOAT", async function () {
    const tx = await nft.mint(user.address, 600, "unwrapS", "Brahman", 2021);
    const tokenId = (await tx.wait()).logs[0].args[2];
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
    const tx = await nft.mint(owner.address, 500, "tag", "Brahman", 2020);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await nft.connect(owner).approve(wrapper.target, tokenId);

    await expect(wrapper.connect(user).wrap(tokenId)).to.be.revertedWith(
      "Not token owner"
    );
  });

  it("reverts unwrap when caller not token owner", async function () {
    const tx = await nft.mint(user.address, 650, "tag2", "Brahman", 2020);
    const tokenId = (await tx.wait()).logs[0].args[2];
    await nft.connect(user).approve(wrapper.target, tokenId);
    await wrapper.connect(user).wrap(tokenId);

    await expect(wrapper.unwrap(tokenId)).to.be.revertedWith("Not owner");
  });

  it("owner() returns deployer", async function () {
    expect(await wrapper.owner()).to.equal(owner.address);
  });
});
