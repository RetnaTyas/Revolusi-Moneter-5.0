const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("SapiNFTBurnHook", function () {
  let owner, user, meat, nft, hook;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy();
    await meat.waitForDeployment();
    await meat.setMinter(owner.address, true);

    const SapiNFT = await ethers.getContractFactory("SapiNFT");
    nft = await SapiNFT.deploy();
    await nft.waitForDeployment();

    const Hook = await ethers.getContractFactory("SapiNFTBurnHook");
    hook = await Hook.deploy(nft.target, meat.target);
    await hook.waitForDeployment();

    await meat.setMinter(hook.target, true);
    await nft.setBurnHook(hook.target);
  });

  it("mints BEEFMEAT on burn", async function () {
    const weight = 700n; // 70 kg
    const tx = await nft.mint(user.address, weight, "tagS", "Brahman", 2021);
    const receipt = await tx.wait();
    const tokenId = receipt.logs[0].args[2];

    await nft.connect(user).updateWeight(tokenId, weight);

    const yieldBps = await hook.SLAUGHTER_YIELD_BPS();
    const meatAmount = (weight * 10n ** 18n * yieldBps) / 10000n / 10n;

    const subtype = await hook.BEEFMEAT_SUBTYPE();
    await expect(nft.connect(user).burn(tokenId))
      .to.emit(meat, "SubtypeMinted")
      .withArgs(user.address, subtype, meatAmount);

    expect(await meat.getBalanceOfSubtype(user.address, subtype)).to.equal(meatAmount);
  });
});
