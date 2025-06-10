// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {IGOAT} from "../interfaces/IGOAT.sol";
import {GOAT} from "../GOAT.sol";

/// @title Token GOAT tiruan yang dapat mensimulasikan kegagalan transfer
contract FailingGOAT is GOAT, IGOAT {
    bool public failTransfer;

    constructor() GOAT() {}

    /// @notice Mengaktifkan atau menonaktifkan mode gagal transfer
    function setFailTransfer(bool value) external {
        failTransfer = value;
    }

    function _update(address from, address to, uint256 value) internal override {
        require(!failTransfer, "Transfer failed");
        super._update(from, to, value);
    }


    /// @inheritdoc IGOAT
    function mintTo(address to, uint256 amount) external override {
        _mint(to, amount);
    }
}
