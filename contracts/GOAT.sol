// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IGoatNFT } from "./interfaces/IGoatNFT.sol";

/// @title GOAT Token - Guardian of Organic Agriculture Trust
/// @notice A smart contract enabling staking, compounding, and minting by MEAT contract
contract GOAT is ERC20 { address private immutable _owner;

event RewardConfigChanged(
    uint256 oldRate,
    uint256 newRate,
    uint256 oldInterval,
    uint256 newInterval,
    uint256 oldMinClaimTime,
    uint256 newMinClaimTime    
);
event Staked(address indexed user, uint256 amount);
event Unstaked(address indexed user, uint256 amount, uint256 reward);
event RewardClaimed(address indexed user, uint256 reward);
event Compounded(address indexed user, uint256 reward);
mapping(address => uint256) public stakingBalance;
mapping(address => uint256) public lastStakedTime;
uint256 public rewardRate = 5e18; // 500% per year, scaled to 1e18
uint256 public rewardInterval = 365 days;
uint256 public minClaimInterval = 7 days;
uint256 private constant REWARD_PRECISION = 1e18;
address public meatContract;
address public nftContract;

constructor(address meatAddress) ERC20("Guardian of Agricultural Trade", "GOAT") {
    _owner = msg.sender;
    meatContract = meatAddress;
}
/// @notice Ensures only the contract owner can execute
modifier onlyOwner() {
    require(msg.sender == _owner, "Not the owner");
    _;
}
/// @notice Sets the MEAT contract address
/// @param meatAddress Address of the MEAT contract
function setMEATAddress(address meatAddress) external onlyOwner {
    require(meatAddress != address(0), "Invalid address");
    meatContract = meatAddress;
}
/// @notice Sets the Goat NFT contract address
/// @param nftAddress Address of the GoatNFT contract
function setNFTAddress(address nftAddress) external onlyOwner {
    require(nftAddress != address(0), "Invalid address");
    nftContract = nftAddress;
}
/// @notice Allows the MEAT contract to mint GOAT tokens to any address
/// @param to The recipient address
/// @param amount The amount of GOAT to mint
function mintTo(address to, uint256 amount) external {
    require(msg.sender == meatContract, "Unauthorized mint");
    _mint(to, amount);
}
/// @notice Burn a Goat NFT to receive GOAT tokens
/// @dev The GoatNFT contract must be trusted; a malicious contract could reenter
///      or change the token's value during burn.
/// @param tokenId ID of the NFT to burn
function burnAndMint(uint256 tokenId) external {
    require(nftContract != address(0), "NFT not set");
    IGoatNFT nft = IGoatNFT(nftContract);
    require(nft.ownerOf(tokenId) == msg.sender, "Not token owner");
    uint256 amount = nft.goatValue(tokenId); // store before burn (Checks-Effects-Interactions)
    nft.burn(tokenId);
    _mint(msg.sender, amount);
}
/// @notice Stake GOAT tokens to earn rewards
/// @param amount The amount of GOAT tokens to stake
function stake(uint256 amount) external {
    require(amount > 0, "Amount must be > 0");
    _transfer(msg.sender, address(this), amount);
    stakingBalance[msg.sender] += amount;
    lastStakedTime[msg.sender] = block.timestamp;
    emit Staked(msg.sender, amount);
}
/// @notice Emergency unstake anytime without claiming rewards
function emergencyUnstake() external {
    uint256 staked = stakingBalance[msg.sender];
    require(staked > 0, "Nothing to unstake");
    stakingBalance[msg.sender] = 0;
    uint256 available = balanceOf(address(this));
    if (available >= staked) {
        _transfer(address(this), msg.sender, staked);
    } else {
        if (available > 0) _transfer(address(this), msg.sender, available);
        _mint(msg.sender, staked - available);
    }
    emit Unstaked(msg.sender, staked, 0);
}
/// @notice Unstake and claim staking rewards if eligible
function unstake() external {
    uint256 staked = stakingBalance[msg.sender];
    require(staked > 0, "Nothing to unstake");
    uint256 lastTime = lastStakedTime[msg.sender];
    require(block.timestamp - lastTime >= minClaimInterval, "Claim not allowed yet");
    uint256 reward = calculateReward(msg.sender, lastTime);
    uint256 total = staked + reward;
    stakingBalance[msg.sender] = 0;    
    uint256 available = balanceOf(address(this));
    if (available >= total) {
        _transfer(address(this), msg.sender, total);
    } else {
        if (available > 0) _transfer(address(this), msg.sender, available);
        _mint(msg.sender, total - available);
    }
    emit Unstaked(msg.sender, staked, reward);
}
/// @notice Claim only the reward without unstaking
function claimReward() external {
    uint256 lastTime = lastStakedTime[msg.sender];
    require(block.timestamp - lastTime >= minClaimInterval, "Claim not allowed yet");
    uint256 reward = calculateReward(msg.sender, lastTime);
    require(reward > 0, "No reward to claim");
    uint256 available = balanceOf(address(this));
    if (available >= reward) {
        _transfer(address(this), msg.sender, reward);
    } else {
        if (available > 0) _transfer(address(this), msg.sender, available);
        _mint(msg.sender, reward - available);
    }
    lastStakedTime[msg.sender] = block.timestamp;
    emit RewardClaimed(msg.sender, reward);
}
/// @notice Compound current reward into staked balance
function compoundReward() external {
    uint256 lastTime = lastStakedTime[msg.sender];
    require(block.timestamp - lastTime >= minClaimInterval, "Claim not allowed yet");
    uint256 reward = calculateReward(msg.sender, lastTime);
    require(reward > 0, "No reward to compound");
    uint256 available = balanceOf(address(this));
    if (available < reward) {
        _mint(address(this), reward - available);
    }
    stakingBalance[msg.sender] += reward;
    lastStakedTime[msg.sender] = block.timestamp;    
    emit Compounded(msg.sender, reward);
}
/// @notice Internal reward calculation based on duration and amount
/// @param user The address of the staker
/// @param lastTime Timestamp of the last stake/claim
/// @return reward amount in GOAT token wei
function calculateReward(address user, uint256 lastTime) internal view returns (uint256) {
    uint256 staked = stakingBalance[user];
    if (staked == 0) return 0;
    uint256 duration = block.timestamp - lastTime;
    return (staked * duration * rewardRate) / (rewardInterval * REWARD_PRECISION);
}
/// @notice Shows pending reward amount
/// @param user The address of the staker
/// @return amount pending to be claimed
function pendingReward(address user) external view returns (uint256) {
    return calculateReward(user, lastStakedTime[user]);
}
/// @notice Returns the next eligible claim timestamp
/// @param user The address of the staker
/// @return timestamp when user can claim reward again
function nextClaimTime(address user) external view returns (uint256) {
    uint256 last = lastStakedTime[user];
    if (last == 0) return 0;
    return last + minClaimInterval;
}
/// @notice Updates the reward settings by the owner
/// @param newRate New annual reward rate (scaled to 1e18)
/// @param newInterval Duration of reward cycle in seconds
/// @param newMinClaimTime Minimum interval before reward can be claimed
function setRewardConfig(uint256 newRate, uint256 newInterval, uint256 newMinClaimTime) external onlyOwner {
    require(newInterval > 0, "Interval must be > 0");
    emit RewardConfigChanged(rewardRate, newRate, rewardInterval, newInterval, minClaimInterval, newMinClaimTime);
    rewardRate = newRate;
    rewardInterval = newInterval;
    minClaimInterval = newMinClaimTime;    
}
/// @notice Returns contract owner address
/// @return Address of contract deployer
function owner() external view returns (address) {
    return _owner;
}
}

