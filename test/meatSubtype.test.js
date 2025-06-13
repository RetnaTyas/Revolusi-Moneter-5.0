const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MEAT subtype functions", function () {
  let owner, minter, burner, user, other, meat;

  beforeEach(async function () {
    [owner, minter, burner, user, other] = await ethers.getSigners();

    const MEAT = await ethers.getContractFactory("MEAT");
    meat = await MEAT.deploy();
    await meat.waitForDeployment();

    await meat.setMinter(minter.address, true);
    await meat.setBurner(burner.address, true);
  });

  it("initializes with owner as minter and 1000 GOATMEAT", async function () {
    const subtype = ethers.keccak256(ethers.toUtf8Bytes("GOATMEAT"));
    const expected = ethers.parseEther("1000");

    expect(await meat.isMinter(owner.address)).to.equal(true);
    expect(await meat.getBalanceOfSubtype(owner.address, subtype)).to.equal(expected);
    expect(await meat.totalSupply()).to.equal(expected);
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

    await meat.connect(user).approve(burner.address, amount);

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
    ).to.be.revertedWithCustomError(meat, "OwnableUnauthorizedAccount").withArgs(
      user.address
    );

    await expect(meat.setMinter(user.address, true))
      .to.emit(meat, "MinterUpdated")
      .withArgs(user.address, true);
    expect(await meat.isMinter(user.address)).to.equal(true);
  });

  it("setBurner onlyOwner and updates", async function () {
    await expect(
      meat.connect(user).setBurner(user.address, true)
    ).to.be.revertedWithCustomError(meat, "OwnableUnauthorizedAccount").withArgs(
      user.address
    );

    await expect(meat.setBurner(user.address, true))
      .to.emit(meat, "BurnerUpdated")
      .withArgs(user.address, true);
    expect(await meat.isBurner(user.address)).to.equal(true);
  });

  it("setSubtypeLineage onlyOwner and emits", async function () {
    const subtype = ethers.encodeBytes32String("GOATMEAT");
    await meat.connect(minter).mintSubtype(user.address, subtype, 1);

    await expect(
      meat.connect(user).setSubtypeLineage(user.address, subtype, 99)
    ).to.be.revertedWith("Not authorized");

    await expect(meat.setSubtypeLineage(user.address, subtype, 99))
      .to.emit(meat, "SubtypeLineageUpdated")
      .withArgs(user.address, subtype, 99);

    const result = await meat.balanceOfSubtypeWithLineage(user.address, subtype);
    expect(result[1]).to.equal(99n);
  });

  it("lineage persists after transfer", async function () {
    const subtype = ethers.encodeBytes32String("GOATMEAT");
    const amount = ethers.parseEther("2");

    await meat.connect(minter).mintSubtype(user.address, subtype, amount);
    await meat.setSubtypeLineage(user.address, subtype, 7);

    await meat.connect(user).transfer(other.address, amount);

    const result = await meat.balanceOfSubtypeWithLineage(other.address, subtype);
    expect(result[0]).to.equal(amount);
    expect(result[1]).to.equal(7n);
  });

  it("transfers multiple subtypes correctly", async function () {
    const goat = ethers.encodeBytes32String("GOATMEAT");
    const beef = ethers.encodeBytes32String("BEEFMEAT");
    const amtGoat = ethers.parseEther("10");
    const amtBeef = ethers.parseEther("20");

    await meat.connect(minter).mintSubtype(user.address, goat, amtGoat);
    await meat.connect(minter).mintSubtype(user.address, beef, amtBeef);
    await meat.setSubtypeLineage(user.address, goat, 1);
    await meat.setSubtypeLineage(user.address, beef, 2);

    const transferAmt = ethers.parseEther("15");
    await meat.connect(user).transfer(other.address, transferAmt);

    expect(await meat.getBalanceOfSubtype(user.address, goat)).to.equal(0n);
    expect(await meat.getBalanceOfSubtype(user.address, beef)).to.equal(
      amtBeef - (transferAmt - amtGoat)
    );
    expect(await meat.getBalanceOfSubtype(other.address, goat)).to.equal(amtGoat);
    expect(await meat.getBalanceOfSubtype(other.address, beef)).to.equal(
      transferAmt - amtGoat
    );

    const resGoat = await meat.balanceOfSubtypeWithLineage(other.address, goat);
    const resBeef = await meat.balanceOfSubtypeWithLineage(other.address, beef);
    expect(resGoat[1]).to.equal(1n);
    expect(resBeef[1]).to.equal(2n);
  });
});
