// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title Antarmuka token GOAT yang digunakan MEAT
interface IGOAT is IERC20 {
    /// @notice Mencetak token GOAT ke alamat tertentu
    /// @param to Penerima token
    /// @param amount Jumlah token yang dicetak
    function mintTo(address to, uint256 amount) external;
}
