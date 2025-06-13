const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MEAT deployment", function () {
  it("assigns deployer as minter and mints initial GOATMEAT", async function () {
    const [owner] = await ethers.getSigners();
    const MEAT = await ethers.getContractFactory("MEAT");
    const meat = await MEAT.deploy();
    await meat.waitForDeployment();

    const subtype = ethers.keccak256(ethers.toUtf8Bytes("GOATMEAT"));
    const expected = ethers.parseEther("1000");

    expect(await meat.isMinter(owner.address)).to.equal(true);
    expect(await meat.getBalanceOfSubtype(owner.address, subtype)).to.equal(expected);
  });
});
