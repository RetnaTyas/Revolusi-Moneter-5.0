// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IGOAT} from "../interfaces/IGOAT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title Mock GOAT token that can simulate transfer failures
contract FailingGOAT is ERC20, IGOAT {
    bool public failTransfer;

    constructor() ERC20("Failing GOAT", "FGOAT") {}

    /// @notice Toggle transfer failure mode
    function setFailTransfer(bool value) external {
        failTransfer = value;
    }

    function transfer(address to, uint256 amount)
        public
        override(ERC20, IERC20)
        returns (bool)
    {
        if (failTransfer) {
            return false;
        }
        return super.transfer(to, amount);
    }

    function transferFrom(address from, address to, uint256 amount)
        public
        override(ERC20, IERC20)
        returns (bool)
    {
        if (failTransfer) {
            return false;
        }
        return super.transferFrom(from, to, amount);
    }

    /// @inheritdoc IGOAT
    function mintTo(address to, uint256 amount) external override {
        _mint(to, amount);
    }
}
