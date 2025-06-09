const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GOAT", function () {
  it("should deploy and set the owner", async function () {
    const [owner] = await ethers.getSigners();
    const GOAT = await ethers.getContractFactory("GOAT");
    const goat = await GOAT.deploy();
    await goat.waitForDeployment();
    expect(await goat.owner()).to.equal(owner.address);
  });
});
