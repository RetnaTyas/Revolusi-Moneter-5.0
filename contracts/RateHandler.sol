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
        uint256 protein_g_per_kg;
        uint256 fat_g_per_kg;
        uint256 micronutrient_index_x1000;
        uint256 yield_per_cycle_kg;
        uint256 cycle_time_days;
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

    function setCommodityRepresentation(bytes32 commodityId, CommodityRepresentation calldata data) public onlyOwner {
        CommodityRepresentation storage c = commodityRegistry[commodityId];
        c.nftAddress = data.nftAddress;
        c.tokenVirtualAddress = data.tokenVirtualAddress;
        c.tokenProductAddress = data.tokenProductAddress;
        c.tokenProductSubtype = data.tokenProductSubtype;
        c.isNftActive = data.isNftActive;
        c.isTokenVirtualActive = data.isTokenVirtualActive;
        c.isTokenProductActive = data.isTokenProductActive;
        c.lodPerDayNft = data.lodPerDayNft;
        c.lodPerDayVirtual = data.lodPerDayVirtual;
        c.lodPerDayProduct = data.lodPerDayProduct;
        c.protein_g_per_kg = data.protein_g_per_kg;
        c.fat_g_per_kg = data.fat_g_per_kg;
        c.micronutrient_index_x1000 = data.micronutrient_index_x1000;
        c.yield_per_cycle_kg = data.yield_per_cycle_kg;
        c.cycle_time_days = data.cycle_time_days;
    }

    /// @notice [DEPRECATED] Use getLODPerDay(bytes32 commodityId, string layer)
    /// @dev Kept for governance audit compatibility only.
    function getLODPerDay(bytes32 commodity) public view returns (uint256) {
        return commodityLOD[commodity].lodPerDay;
    }

    function getLODPerDay(bytes32 commodityId, string memory layer) public view returns (uint256) {
        CommodityRepresentation storage cr = commodityRegistry[commodityId];
        bytes32 l = keccak256(bytes(layer));
        if (l == keccak256("NFT")) {
            return cr.lodPerDayNft;
        } else if (l == keccak256("VIRTUAL")) {
            return cr.lodPerDayVirtual;
        } else if (l == keccak256("PRODUCT")) {
            return cr.lodPerDayProduct;
        } else {
            revert("Invalid layer");
        }
    }


    /// @notice Compute barter rate between two commodities
    /// @dev Only PRODUCT↔PRODUCT swaps are allowed and enforced.
    function computeBarterRate(
        bytes32 fromCommodity,
        string memory fromLayer,
        bytes32 toCommodity,
        string memory toLayer
    ) public view returns (uint256) {
        bytes32 productHash = keccak256("PRODUCT");
        require(
            keccak256(bytes(fromLayer)) == productHash,
            "FROM layer must be PRODUCT"
        );
        require(
            keccak256(bytes(toLayer)) == productHash,
            "TO layer must be PRODUCT"
        );

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
