// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {SwapConfig} from "./SwapConfig.sol";

contract RateHandler {
    address private _owner;

    uint256 public dynamicRate;
    uint256 public lastUpdateTimestamp;
    bool public dynamicRateValid;

    struct CommodityLOD {
        uint256 lodPerDay;
        uint256 lastSet;
    }

    mapping(bytes32 => CommodityLOD) public commodityLOD;

    struct CommodityRepresentation {
        address nftAddress;
        address tokenVirtualAddress;
        address tokenProductAddress;
        bytes32 tokenProductSubtype;
        bool isNftActive;
        bool isTokenVirtualActive;
        bool isTokenProductActive;
        uint256 lodPerDayNft;
        uint256 lodPerDayVirtual;
        uint256 lodPerDayProduct;
    }

    mapping(bytes32 => CommodityRepresentation) public commodityRegistry;

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

    function setCommodityLOD(bytes32 commodity, uint256 lodPerDay) external onlyOwner {
        require(commodity != bytes32(0), "Invalid commodity");
        commodityLOD[commodity] = CommodityLOD(lodPerDay, block.timestamp);
    }

    function setCommodityRepresentation(bytes32 commodityId, CommodityRepresentation memory data) public onlyOwner {
        commodityRegistry[commodityId] = data;
    }

    function getLODPerDay(bytes32 commodity) public view returns (uint256) {
        return commodityLOD[commodity].lodPerDay;
    }

    function getLODPerDay(bytes32 commodityId, string memory layer) public view returns (uint256) {
        CommodityRepresentation memory cr = commodityRegistry[commodityId];
        if (keccak256(bytes(layer)) == keccak256("NFT")) {
            return cr.lodPerDayNft;
        } else if (keccak256(bytes(layer)) == keccak256("VIRTUAL")) {
            return cr.lodPerDayVirtual;
        } else if (keccak256(bytes(layer)) == keccak256("PRODUCT")) {
            return cr.lodPerDayProduct;
        } else {
            revert("Invalid layer");
        }
    }

    function computeBarterRate(bytes32 commodity) public view returns (uint256) {
        uint256 base = getCurrentRate();
        uint256 lod = commodityLOD[commodity].lodPerDay;
        if (lod == 0) return base;
        uint256 daysPassed = (block.timestamp - lastUpdateTimestamp) / 1 days;
        return base + (lod * daysPassed);
    }

    function computeBarterRate(
        bytes32 fromCommodity,
        string memory fromLayer,
        bytes32 toCommodity,
        string memory toLayer
    ) public view returns (uint256) {
        uint256 fromLOD = getLODPerDay(fromCommodity, fromLayer);
        uint256 toLOD = getLODPerDay(toCommodity, toLayer);

        require(fromLOD > 0, "Invalid FROM LOD");
        require(toLOD > 0, "Invalid TO LOD");

        uint256 rate = (fromLOD * 1e18) / toLOD;
        return rate;
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
