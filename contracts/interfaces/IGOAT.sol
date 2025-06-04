// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title Interface for the GOAT token used by MEAT
interface IGOAT is IERC20 {
    /// @notice Mint GOAT tokens to a specific address
    /// @param to Recipient of the tokens
    /// @param amount Amount of tokens to mint
    function mintTo(address to, uint256 amount) external;
}
