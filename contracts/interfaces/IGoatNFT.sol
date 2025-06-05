// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

interface IGoatNFT {
    function goatValue(uint256 tokenId) external view returns (uint256);
    function burn(uint256 tokenId) external;
    function ownerOf(uint256 tokenId) external view returns (address);
}
