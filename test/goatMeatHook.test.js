const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GoatNFTBurnHook", function () {
  let owner, user, goat, meat, nft, hook;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const GOAT = await ethers.getContractFactory("GOAT");
    goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy(goat.target);
    await meat.waitForDeployment();
    await meat.setMinter(owner.address, true);

    const GoatNFT = await ethers.getContractFactory("GoatNFT");
    nft = await GoatNFT.deploy(goat.target);
    await nft.waitForDeployment();

    const Hook = await ethers.getContractFactory("GoatNFTBurnHook");
    hook = await Hook.deploy(nft.target, meat.target);
    await hook.waitForDeployment();

    await goat.setNFTAddress(nft.target);
    await goat.setMEATAddress(meat.target);

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
});

