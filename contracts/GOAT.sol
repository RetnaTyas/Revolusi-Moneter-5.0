// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

/// @title Token GOAT - Guardian of Agricultural Trade (Capital Layer Token)
/// @notice Token GOAT hanya dapat dicetak oleh GoatNFTWrapper (wrap) dan digunakan untuk staking / ROI.
/// @dev Tidak ada path mintTo oleh MEAT. Tidak ada cross-layer leak. Reasoning Path FINAL Compliant.

contract GOAT is ERC20, Ownable {

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
    event EmergencyUnstaked(address indexed user, uint256 amount);
    event RewardClaimed(address indexed user, uint256 reward);
    event Compounded(address indexed user, uint256 reward);
    event WrapperAddressUpdated(address indexed oldAddress, address indexed newAddress);

    mapping(address => uint256) public stakingBalance;
    mapping(address => uint256) public lastStakedTime;

    uint256 public rewardRate = 5e18; // 500% per year, scaled to 1e18
    uint256 public rewardInterval = 365 days;
    uint256 public minClaimInterval = 7 days;
    uint256 private constant REWARD_PRECISION = 1e18;

    address public wrapperContract;

    constructor() ERC20("Guardian of Agricultural Trade", "GOAT") Ownable(msg.sender) {}

    /// @notice Set wrapper contract (GoatNFTWrapper) — only this contract can mint/burn GOAT
    function setWrapperContract(address wrapperAddress) external onlyOwner {
        require(wrapperAddress != address(0), "Invalid address");
        address old = wrapperContract;
        wrapperContract = wrapperAddress;
        emit WrapperAddressUpdated(old, wrapperAddress);
    }

    /// @notice Mint GOAT — only called by GoatNFTWrapper
    function mint(address to, uint256 amount) external {
        require(msg.sender == wrapperContract, "Unauthorized mint");
        _mint(to, amount);
    }

    /// @notice Burn GOAT — only called by GoatNFTWrapper
    function burnFrom(address from, uint256 amount) external {
        require(msg.sender == wrapperContract, "Unauthorized burn");
        _burn(from, amount);
    }

    /// @notice Stake token GOAT untuk memperoleh reward
    function stake(uint256 amount) external {
        require(amount > 0, "Amount must be > 0");
        _transfer(msg.sender, address(this), amount);
        stakingBalance[msg.sender] += amount;
        lastStakedTime[msg.sender] = block.timestamp;
        emit Staked(msg.sender, amount);
    }

    /// @notice Unstake darurat kapan saja tanpa klaim reward
    function emergencyUnstake() external {
        uint256 staked = stakingBalance[msg.sender];
        require(staked > 0, "Nothing to unstake");
        stakingBalance[msg.sender] = 0;
        lastStakedTime[msg.sender] = 0;
        uint256 available = balanceOf(address(this));
        if (available >= staked) {
            _transfer(address(this), msg.sender, staked);
        } else {
            if (available > 0) _transfer(address(this), msg.sender, available);
            _mint(msg.sender, staked - available);
        }
        emit EmergencyUnstaked(msg.sender, staked);
    }

    /// @notice Unstake dan klaim reward jika memenuhi syarat
    function unstake() external {
        uint256 staked = stakingBalance[msg.sender];
        require(staked > 0, "Nothing to unstake");
        uint256 lastTime = lastStakedTime[msg.sender];
        require(block.timestamp - lastTime >= minClaimInterval, "Claim not allowed yet");
        uint256 reward = calculateReward(msg.sender, lastTime);
        uint256 total = staked + reward;
        stakingBalance[msg.sender] = 0;
        lastStakedTime[msg.sender] = 0;
        uint256 available = balanceOf(address(this));
        if (available >= total) {
            _transfer(address(this), msg.sender, total);
        } else {
            if (available > 0) _transfer(address(this), msg.sender, available);
            _mint(msg.sender, total - available);
        }
        emit Unstaked(msg.sender, staked, reward);
    }

    /// @notice Klaim hanya reward tanpa unstake
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

    /// @notice Menggabungkan reward ke saldo staking
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

    /// @notice Perhitungan reward internal berdasarkan durasi dan jumlah
    function calculateReward(address user, uint256 lastTime) internal view returns (uint256) {
        uint256 staked = stakingBalance[user];
        if (staked == 0) return 0;
        uint256 duration = block.timestamp - lastTime;
        return (staked * duration * rewardRate) / (rewardInterval * REWARD_PRECISION);
    }

    /// @notice Menampilkan jumlah reward yang menunggu
    function pendingReward(address user) external view returns (uint256) {
        return calculateReward(user, lastStakedTime[user]);
    }

    /// @notice Mengembalikan timestamp klaim berikutnya
    function nextClaimTime(address user) external view returns (uint256) {
        uint256 last = lastStakedTime[user];
        if (last == 0) return 0;
        return last + minClaimInterval;
    }

    /// @notice Memperbarui konfigurasi reward oleh pemilik
    function setRewardConfig(uint256 newRate, uint256 newInterval, uint256 newMinClaimTime) external onlyOwner {
        require(newInterval > 0, "Interval must be > 0");
        emit RewardConfigChanged(rewardRate, newRate, rewardInterval, newInterval, minClaimInterval, newMinClaimTime);
        rewardRate = newRate;
        rewardInterval = newInterval;
        minClaimInterval = newMinClaimTime;
    }

    // Ownable already exposes owner() view
}
