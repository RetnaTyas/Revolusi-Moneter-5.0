// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC721Holder} from "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import {ISapiNFT} from "./interfaces/ISapiNFT.sol";
import {IGoatToken} from "./interfaces/IGoatToken.sol";
import {SwapConfig} from "./SwapConfig.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title SapiNFTWrapper
/// @notice Mengunci SapiNFT dan mencetak GOAT sebagai jaminan (berdasarkan SAPI_WRAP_RATE)
contract SapiNFTWrapper is ERC721Holder, Ownable {
    IERC721 public immutable sapiNFT;
    ISapiNFT public immutable sapiValueFeed;
    IGoatToken public goatToken;

    uint256 public constant WEIGHT_DECIMALS = 1;

    struct WrappedInfo {
        address owner;
        uint256 goatAmount;
    }

    mapping(uint256 => WrappedInfo) public wrapped;

    event Wrapped(address indexed user, uint256 indexed tokenId, uint256 goatAmount);
    event Unwrapped(address indexed user, uint256 indexed tokenId, uint256 goatAmount);

    constructor(address nftAddress, address goatAddress) Ownable(msg.sender) {
        require(nftAddress != address(0) && goatAddress != address(0), "Invalid address");
        sapiNFT = IERC721(nftAddress);
        sapiValueFeed = ISapiNFT(nftAddress);
        goatToken = IGoatToken(goatAddress);
    }

    function _getRate() internal pure returns (uint256) {
        return SwapConfig.SAPI_WRAP_RATE;
    }

    function wrap(uint256 tokenId) external {
        require(sapiNFT.ownerOf(tokenId) == msg.sender, "Not token owner");
        sapiNFT.safeTransferFrom(msg.sender, address(this), tokenId);

        uint256 weight = sapiValueFeed.sapiValue(tokenId);
        require(weight > 0, "Invalid weight");

        uint256 rate = _getRate();
        uint256 goatAmount = (weight * 1e18) / rate / (10 ** WEIGHT_DECIMALS);

        goatToken.mint(msg.sender, goatAmount);
        wrapped[tokenId] = WrappedInfo(msg.sender, goatAmount);

        emit Wrapped(msg.sender, tokenId, goatAmount);
    }

    function unwrap(uint256 tokenId) external {
        WrappedInfo memory info = wrapped[tokenId];
        require(info.owner == msg.sender, "Not owner");

        goatToken.burnFrom(msg.sender, info.goatAmount);
        delete wrapped[tokenId];

        sapiNFT.safeTransferFrom(address(this), msg.sender, tokenId);
        emit Unwrapped(msg.sender, tokenId, info.goatAmount);
    }

    // Ownable already exposes owner() view
}
