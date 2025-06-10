const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MEAT subtype functions", function () {
  let owner, minter, burner, user, meat;

  beforeEach(async function () {
    [owner, minter, burner, user] = await ethers.getSigners();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy();
    await meat.waitForDeployment();

    await meat.setMinter(minter.address, true);
    await meat.setBurner(burner.address, true);
  });

  it("mints and burns subtype tokens", async function () {
    const subtype = ethers.encodeBytes32String("GOATMEAT");
    const amount = ethers.parseEther("10");

    await expect(
      meat.connect(minter).mintSubtype(user.address, subtype, amount)
    )
      .to.emit(meat, "SubtypeMinted")
      .withArgs(user.address, subtype, amount);

    expect(
      await meat.getBalanceOfSubtype(user.address, subtype)
    ).to.equal(amount);
    expect(await meat.getTotalSupplyOfSubtype(subtype)).to.equal(amount);
    expect(await meat.balanceOf(user.address)).to.equal(amount);

    await expect(
      meat.connect(burner).burnSubtype(user.address, subtype, amount)
    )
      .to.emit(meat, "SubtypeBurned")
      .withArgs(user.address, subtype, amount);

    expect(
      await meat.getBalanceOfSubtype(user.address, subtype)
    ).to.equal(0n);
    expect(await meat.getTotalSupplyOfSubtype(subtype)).to.equal(0n);
    expect(await meat.balanceOf(user.address)).to.equal(0n);
  });

  it("reverts burn with insufficient balance", async function () {
    const subtype = ethers.encodeBytes32String("DUCKMEAT");
    await expect(
      meat.connect(burner).burnSubtype(user.address, subtype, 1)
    ).to.be.revertedWith("Insufficient subtype balance");
  });

  it("returns balance and lineage", async function () {
    const subtype = ethers.encodeBytes32String("GOATMEAT");
    const amount = ethers.parseEther("5");

    await meat.connect(minter).mintSubtype(user.address, subtype, amount);
    await meat.setSubtypeLineage(user.address, subtype, 42);

    const result = await meat.balanceOfSubtypeWithLineage(user.address, subtype);
    expect(result[0]).to.equal(amount);
    expect(result[1]).to.equal(42n);
  });

  it("owner() returns deployer", async function () {
    expect(await meat.owner()).to.equal(owner.address);
  });

  it("setMinter onlyOwner and updates", async function () {
    await expect(
      meat.connect(user).setMinter(user.address, true)
    ).to.be.revertedWith("Not the owner");

    await meat.setMinter(user.address, true);
    expect(await meat.isMinter(user.address)).to.equal(true);
  });

  it("setBurner onlyOwner and updates", async function () {
    await expect(
      meat.connect(user).setBurner(user.address, true)
    ).to.be.revertedWith("Not the owner");

    await meat.setBurner(user.address, true);
    expect(await meat.isBurner(user.address)).to.equal(true);
  });

  it("setSubtypeLineage onlyOwner and emits", async function () {
    const subtype = ethers.encodeBytes32String("GOATMEAT");
    await meat.connect(minter).mintSubtype(user.address, subtype, 1);

    await expect(
      meat.connect(user).setSubtypeLineage(user.address, subtype, 99)
    ).to.be.revertedWith("Not the owner");

    await expect(meat.setSubtypeLineage(user.address, subtype, 99))
      .to.emit(meat, "SubtypeLineageUpdated")
      .withArgs(user.address, subtype, 99);

    const result = await meat.balanceOfSubtypeWithLineage(user.address, subtype);
    expect(result[1]).to.equal(99n);
  });
});
