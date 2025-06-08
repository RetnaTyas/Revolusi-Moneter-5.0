// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {SwapConfig} from "./SwapConfig.sol";

contract RateHandler {
    address private _owner;

    uint256 public dynamicRate;
    uint256 public lastUpdateTimestamp;
    bool public dynamicRateValid;

    event RateUpdated(uint256 newRate, uint256 timestamp);
    event RateInvalidated(uint256 timestamp);
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    constructor() {
        _owner = msg.sender;
        dynamicRateValid = false;
    }

    function updateRate(uint256 newRate) external onlyOwner {
        require(newRate > 0, "Rate must be > 0");
        dynamicRate = newRate;
        lastUpdateTimestamp = block.timestamp;
        dynamicRateValid = true;
        emit RateUpdated(newRate, block.timestamp);
    }

    function invalidateRate() external onlyOwner {
        dynamicRateValid = false;
        emit RateInvalidated(block.timestamp);
    }

    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Invalid address");
        address old = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(old, newOwner);
    }

    function getCurrentRate() public view returns (uint256) {
        if (dynamicRateValid) {
            return dynamicRate;
        } else {
            return SwapConfig.SWAP_RATE;
        }
    }

    function owner() external view returns (address) {
        return _owner;
    }
}
