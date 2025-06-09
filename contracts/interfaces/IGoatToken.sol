// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

/// @title Antarmuka pencetakan token GOAT oleh GoatNFT
interface IGoatToken {
    /// @notice Mencetak token ke `to`
    /// @param to alamat penerima
    /// @param amount jumlah token GOAT
    function mint(address to, uint256 amount) external;

    /// @notice Membakar token dari `from`
    /// @param from alamat sumber
    /// @param amount jumlah token GOAT
    function burnFrom(address from, uint256 amount) external;
}
