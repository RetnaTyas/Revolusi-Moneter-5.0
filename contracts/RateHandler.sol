// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

/// @title RateHandler - LOD-based Barter Rate Handler (Reasoning Path FINAL PURE CLEAN)
/// @notice Computes PRODUCT↔PRODUCT swap parity based purely on LOD values.
/// @dev No fallback, no deprecated mappings. LOD parity is fully transparent and governed.

contract RateHandler {
    address private _owner;

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

    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);
    event CommodityRepresentationUpdated(bytes32 indexed commodityId);

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    constructor() {
        _owner = msg.sender;
    }

    /// @notice Register or update CommodityRepresentation.
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
        emit CommodityRepresentationUpdated(commodityId);
    }

    /// @notice Get LOD per layer for given commodity.
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

    /// @notice Compute PRODUCT↔PRODUCT barter rate → PURE LOD parity.
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

    /// @notice Transfer ownership.
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Invalid address");
        address old = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(old, newOwner);
    }

    /// @notice Return current owner.
    function owner() external view returns (address) {
        return _owner;
    }
}

