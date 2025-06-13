// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { MEAT } from "../MEAT.sol";

/// @title Contract that attempts to reenter MEAT.withdrawNative
contract ReentrantWithdrawer {
    MEAT public meat;
    bool public attackInProgress;

    constructor(address meatAddr) {
        meat = MEAT(payable(meatAddr));
    }

    receive() external payable {
        if (attackInProgress) {
            attackInProgress = false;
            // attempt reentrant call
            meat.withdrawNative();
        }
    }

    function attackWithdraw() external {
        attackInProgress = true;
        meat.withdrawNative();
    }
}
