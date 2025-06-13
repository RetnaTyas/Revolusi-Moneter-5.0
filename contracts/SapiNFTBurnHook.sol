// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { ISapiNFT } from "./interfaces/ISapiNFT.sol";
import { IMeat } from "./interfaces/IMeat.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

/// @title SapiNFTBurnHook
/// @notice Mints BEEFMEAT tokens when a SapiNFT is burned
contract SapiNFTBurnHook is Ownable {
    ISapiNFT public sapiNFT;
    IMeat public meatToken;


    bytes32 public constant BEEFMEAT_SUBTYPE = keccak256("BEEFMEAT");
    uint256 public constant SLAUGHTER_YIELD_BPS = 6500;
    uint256 public constant WEIGHT_DECIMALS = 1;

    event MeatAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event NFTAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event BeefMeatMinted(address indexed to, uint256 amount);

    constructor(address nftAddress, address meatAddress) Ownable(msg.sender) {
        require(nftAddress != address(0) && meatAddress != address(0), "Invalid address");
        sapiNFT = ISapiNFT(nftAddress);
        meatToken = IMeat(meatAddress);
    }

    function setNFTAddress(address nftAddress) external onlyOwner {
        require(nftAddress != address(0), "Invalid address");
        address old = address(sapiNFT);
        sapiNFT = ISapiNFT(nftAddress);
        emit NFTAddressUpdated(old, nftAddress);
    }

    function setMEATAddress(address meatAddress) external onlyOwner {
        require(meatAddress != address(0), "Invalid address");
        address old = address(meatToken);
        meatToken = IMeat(meatAddress);
        emit MeatAddressUpdated(old, meatAddress);
    }

    /// @notice Called by SapiNFT when a token is burned
    /// @param to recipient of BEEFMEAT
    /// @param weight weight value from SapiNFT (scaled with WEIGHT_DECIMALS)
    function onBurn(address to, uint256 weight) external {
        require(msg.sender == address(sapiNFT), "Unauthorized");
        if (weight == 0) return;
        uint256 meatAmount = (weight * 1e18 * SLAUGHTER_YIELD_BPS) / (10000 * (10 ** WEIGHT_DECIMALS));
        meatToken.mintSubtype(to, BEEFMEAT_SUBTYPE, meatAmount);
        emit BeefMeatMinted(to, meatAmount);
    }

    // Ownable already exposes owner() view
}
