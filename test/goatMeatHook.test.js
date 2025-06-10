const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GoatNFTBurnHook", function () {
  let owner, user, meat, nft, hook;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy();
    await meat.waitForDeployment();
    await meat.setMinter(owner.address, true);

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy();
    await nft.waitForDeployment();

    const Hook = await ethers.getContractFactory("GoatNFTBurnHook");
    hook = await Hook.deploy(nft.target, meat.target);
    await hook.waitForDeployment();

    await meat.setMinter(hook.target, true);
    await nft.setBurnHook(hook.target);
  });

  it("mints GOATMEAT on burn", async function () {
    const weight = 500n; // 50 kg
    const tx = await nft.mint(user.address, weight, "tag1", "Boer", 2022);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    await nft.connect(user).updateWeight(tokenId, weight);

    const yieldBps = await hook.SLAUGHTER_YIELD_BPS();
    const meatAmount = (weight * 10n ** 18n * yieldBps) / 10000n / 10n;

    const subtype = await hook.GOATMEAT_SUBTYPE();
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(meat, "SubtypeMinted")
      .withArgs(user.address, subtype, meatAmount);

    expect(await meat.getBalanceOfSubtype(user.address, subtype)).to.equal(
      meatAmount
    );
  });

  it("owner() returns deployer", async function () {
    expect(await hook.owner()).to.equal(owner.address);
  });

  it("setNFTAddress only owner", async function () {
    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    const newNFT = await GoatNFT.deploy();
    await newNFT.waitForDeployment();

    await expect(
      hook.connect(user).setNFTAddress(newNFT.target)
    ).to.be.revertedWith("Not the owner");

    await expect(hook.setNFTAddress(newNFT.target))
      .to.emit(hook, "NFTAddressUpdated")
      .withArgs(nft.target, newNFT.target);
    expect(await hook.goatNFT()).to.equal(newNFT.target);
  });

  it("setMEATAddress only owner", async function () {
    const MEAT = await ethers.getContractFactory("MEAT");
    const newMeat = await MEAT.deploy();
    await newMeat.waitForDeployment();

    await expect(
      hook.connect(user).setMEATAddress(newMeat.target)
    ).to.be.revertedWith("Not the owner");

    await expect(hook.setMEATAddress(newMeat.target))
      .to.emit(hook, "MeatAddressUpdated")
      .withArgs(meat.target, newMeat.target);
    expect(await hook.meatToken()).to.equal(newMeat.target);
  });
});

