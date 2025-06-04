const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MEAT", function () {
  it("should deploy and mint initial supply", async function () {
    const [owner] = await ethers.getSigners();
    const GOAT = await ethers.getContractFactory("GOAT");
    const goat = await GOAT.deploy(owner.address);
    await goat.waitForDeployment();

    const MEAT = await ethers.getContractFactory("MEAT");
    const meat = await MEAT.deploy(goat.target);
    await meat.waitForDeployment();

    expect(await meat.owner()).to.equal(owner.address);
    const balance = await meat.balanceOf(owner.address);
    expect(balance).to.equal(ethers.parseEther("1000"));
  });
});
