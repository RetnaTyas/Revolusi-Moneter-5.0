// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {IGoatNFT} from "./interfaces/IGoatNFT.sol";
import {IMeat} from "./interfaces/IMeat.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title GoatNFTBurnHook
/// @notice Mints GOATMEAT tokens when a GoatNFT is burned
contract GoatNFTBurnHook is Ownable {
    IGoatNFT public goatNFT;
    IMeat public meatToken;


    bytes32 public constant GOATMEAT_SUBTYPE = keccak256("GOATMEAT");
    uint256 public constant SLAUGHTER_YIELD_BPS = 6000; // 60% of live weight
    uint256 public constant WEIGHT_DECIMALS = 1;

    event MeatAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event NFTAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event GoatMeatMinted(address indexed to, uint256 amount);

    constructor(address nftAddress, address meatAddress) Ownable(msg.sender) {
        require(nftAddress != address(0) && meatAddress != address(0), "Invalid address");
        goatNFT = IGoatNFT(nftAddress);
        meatToken = IMeat(meatAddress);
    }

    function setNFTAddress(address nftAddress) external onlyOwner {
        require(nftAddress != address(0), "Invalid address");
        address old = address(goatNFT);
        goatNFT = IGoatNFT(nftAddress);
        emit NFTAddressUpdated(old, nftAddress);
    }

    function setMEATAddress(address meatAddress) external onlyOwner {
        require(meatAddress != address(0), "Invalid address");
        address old = address(meatToken);
        meatToken = IMeat(meatAddress);
        emit MeatAddressUpdated(old, meatAddress);
    }

    /// @notice Called by GoatNFT when a token is burned
    /// @param to recipient of GOATMEAT
    /// @param weight weight value from GoatNFT (scaled with WEIGHT_DECIMALS)
    function onBurn(address to, uint256 weight) external {
        require(msg.sender == address(goatNFT), "Unauthorized");
        if (weight == 0) return;
        uint256 meatAmount = (weight * 1e18 * SLAUGHTER_YIELD_BPS) / (10000 * (10 ** WEIGHT_DECIMALS));
        meatToken.mintSubtype(to, GOATMEAT_SUBTYPE, meatAmount);
        emit GoatMeatMinted(to, meatAmount);
    }

    // Ownable already exposes owner() view
}

