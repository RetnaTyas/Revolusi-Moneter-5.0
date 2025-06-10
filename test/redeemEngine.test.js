const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("RedeemEngine", function () {
  let owner, user, meat, engine;
  const SUBTYPE = ethers.encodeBytes32String("GOATMEAT");
  const GRAMS_PER_TOKEN = 1000; // 1 token = 1000 grams

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy();
    await meat.waitForDeployment();

    const Engine = await ethers.getContractFactory("RedeemEngine");
    engine = await Engine.deploy(meat.target);
    await engine.waitForDeployment();

    await meat.setMinter(owner.address, true);
    await meat.setBurner(engine.target, true);

    await engine.setRedeemConfig(SUBTYPE, GRAMS_PER_TOKEN, true);

    const amount = ethers.parseEther("5");
    await meat.mintSubtype(user.address, SUBTYPE, amount);
    await meat.setSubtypeLineage(user.address, SUBTYPE, 7);
  });

  it("redeems MEAT with lineage check", async function () {
    const redeemAmount = ethers.parseEther("2");

    await meat.connect(user).approve(engine.target, redeemAmount);

    const expectedGrams = (redeemAmount * BigInt(GRAMS_PER_TOKEN)) / 10n ** 18n;

    await expect(engine.connect(user).redeem(SUBTYPE, redeemAmount))
      .to.emit(engine, "RedeemExecuted")
      .withArgs(user.address, SUBTYPE, 7, redeemAmount, expectedGrams);

    expect(await meat.getBalanceOfSubtype(user.address, SUBTYPE)).to.equal(
      ethers.parseEther("3")
    );
  });

  it("reverts when lineage not set", async function () {
    const MEAT = await ethers.getContractFactory("MEAT");
    const freshMeat = await MEAT.deploy();
    await freshMeat.waitForDeployment();

    const Engine = await ethers.getContractFactory("RedeemEngine");
    const freshEngine = await Engine.deploy(freshMeat.target);
    await freshEngine.waitForDeployment();

    await freshMeat.setMinter(owner.address, true);
    await freshMeat.setBurner(freshEngine.target, true);
    await freshMeat.mintSubtype(user.address, SUBTYPE, ethers.parseEther("1"));
    await freshEngine.setRedeemConfig(SUBTYPE, GRAMS_PER_TOKEN, true);

    await expect(
      freshEngine.connect(user).redeem(SUBTYPE, ethers.parseEther("1"))
    ).to.be.revertedWith("Lineage not set");
  });

  it("reverts when subtype inactive", async function () {
    await engine.setRedeemConfig(SUBTYPE, GRAMS_PER_TOKEN, false);

    await meat.connect(user).approve(engine.target, ethers.parseEther("1"));

    await expect(
      engine.connect(user).redeem(SUBTYPE, ethers.parseEther("1"))
    ).to.be.revertedWith("Redeem inactive");
  });

  it("only owner can set config", async function () {
    await expect(
      engine.connect(user).setRedeemConfig(SUBTYPE, 500, true)
    )
      .to.be.revertedWithCustomError(engine, "OwnableUnauthorizedAccount")
      .withArgs(user.address);

    await engine.setRedeemConfig(SUBTYPE, 500, true);

    const cfg = await engine.redeemConfigs(SUBTYPE);
    expect(cfg.gramsPerTokenUnit).to.equal(500n);
    expect(cfg.isActive).to.equal(true);
  });
});

