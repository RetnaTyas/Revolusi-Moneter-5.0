// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { MEAT } from "./MEAT.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

/// @title RedeemEngine
/// @notice Processes MEAT redemption with lineage verification
/// @dev Subtype arguments must use ethers.encodeBytes32String
contract RedeemEngine is Ownable {
    MEAT public immutable meat;

    event RedeemExecuted(
        address indexed user,
        bytes32 indexed subtype,
        uint256 lineageID,
        uint256 amount
    );

    constructor(address meatAddress) Ownable(msg.sender) {
        require(meatAddress != address(0), "Invalid MEAT address");
        meat = MEAT(payable(meatAddress));
    }

    /// Subtype parameter should be bytes32 via ethers.encodeBytes32String
    /// @notice Redeem MEAT subtype after verifying lineage
    function redeem(bytes32 subtype, uint256 amount) external {
        require(amount > 0, "Invalid amount");
        (uint256 balance, uint256 lineageID) = meat.balanceOfSubtypeWithLineage(msg.sender, subtype);
        require(balance >= amount, "Insufficient balance");
        require(lineageID != 0, "Lineage not set");
        meat.burnSubtype(msg.sender, subtype, amount);
        emit RedeemExecuted(msg.sender, subtype, lineageID, amount);
    }
}
