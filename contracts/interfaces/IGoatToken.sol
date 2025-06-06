// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

/// @title Interface for GOAT token minting by GoatNFT
interface IGoatToken {
    /// @notice Mint tokens to `to`
    /// @param to recipient address
    /// @param amount amount of GOAT tokens
    function mint(address to, uint256 amount) external;
}
