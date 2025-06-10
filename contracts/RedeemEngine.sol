// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { MEAT } from "./MEAT.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

/// @title RedeemEngine
/// @notice Processes MEAT redemption with lineage verification
/// @dev Subtype arguments must use ethers.encodeBytes32String
contract RedeemEngine is Ownable {
    MEAT public immutable meat;

    struct RedeemConfig {
        uint256 gramsPerTokenUnit;
        bool isActive;
    }

    mapping(bytes32 => RedeemConfig) public redeemConfigs;

    event RedeemExecuted(
        address indexed user,
        bytes32 indexed subtype,
        uint256 lineageID,
        uint256 amount,
        uint256 grams
    );

    constructor(address meatAddress) Ownable(msg.sender) {
        require(meatAddress != address(0), "Invalid MEAT address");
        meat = MEAT(payable(meatAddress));
    }

    /// @notice Configure grams redemption and activation state for a subtype
    function setRedeemConfig(
        bytes32 subtype,
        uint256 gramsPerTokenUnit,
        bool active
    ) external onlyOwner {
        require(subtype != bytes32(0), "Invalid subtype");
        RedeemConfig storage cfg = redeemConfigs[subtype];
        cfg.gramsPerTokenUnit = gramsPerTokenUnit;
        cfg.isActive = active;
    }

    /// Subtype parameter should be bytes32 via ethers.encodeBytes32String
    /// @notice Redeem MEAT subtype after verifying lineage and config
    function redeem(bytes32 subtype, uint256 amount) external {
        require(amount > 0, "Invalid amount");
        RedeemConfig storage cfg = redeemConfigs[subtype];
        require(cfg.isActive, "Redeem inactive");

        (uint256 balance, uint256 lineageID) = meat.balanceOfSubtypeWithLineage(msg.sender, subtype);
        require(balance >= amount, "Insufficient balance");
        require(lineageID != 0, "Lineage not set");
        meat.burnSubtype(msg.sender, subtype, amount);

        uint256 grams = (amount * cfg.gramsPerTokenUnit) / 1e18;
        emit RedeemExecuted(msg.sender, subtype, lineageID, amount, grams);
    }
}
