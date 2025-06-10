// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { RateHandler } from "./RateHandler.sol";
import { MEAT } from "./MEAT.sol";

/// @title BarterContract v1
/// @notice Enables PRODUCT↔PRODUCT swap based on RateHandler LOD parity.
/// @dev Fully Reasoning Path FINAL Compliant. NO cross-layer swap allowed.
contract BarterContract {
    address private immutable _owner;
    RateHandler public rateHandler;
    MEAT public meatToken;

    event BarterExecuted(
        address indexed user,
        bytes32 indexed fromSubtype,
        uint256 fromAmount,
        bytes32 indexed toSubtype,
        uint256 toAmount
    );

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    constructor(address rateHandlerAddress, address meatAddress) {
        require(rateHandlerAddress != address(0), "Invalid RateHandler address");
        require(meatAddress != address(0), "Invalid MEAT address");
        _owner = msg.sender;
        rateHandler = RateHandler(rateHandlerAddress);
        meatToken = MEAT(payable(meatAddress));
    }

    /// @notice Governance can update RateHandler address if needed
    function setRateHandler(address rateHandlerAddress) external onlyOwner {
        require(rateHandlerAddress != address(0), "Invalid RateHandler address");
        rateHandler = RateHandler(rateHandlerAddress);
    }

    /// @notice Governance can update MEAT token address if needed
    function setMEAT(address meatAddress) external onlyOwner {
        require(meatAddress != address(0), "Invalid MEAT address");
        meatToken = MEAT(payable(meatAddress));
    }

    /// @notice PRODUCT↔PRODUCT swap
    /// @param fromSubtype Subtype of MEAT being burned
    /// @param toSubtype Subtype of MEAT being minted
    /// @param fromAmount Amount of fromSubtype to burn
    function barterProductToProduct(
        bytes32 fromSubtype,
        bytes32 toSubtype,
        uint256 fromAmount
    ) external {
        require(fromSubtype != bytes32(0) && toSubtype != bytes32(0), "Invalid subtype");
        require(fromAmount > 0, "Amount must be > 0");
        require(fromSubtype != toSubtype, "Cannot swap same subtype");

        uint256 rate = rateHandler.computeBarterRate(
            fromSubtype,
            "PRODUCT",
            toSubtype,
            "PRODUCT"
        );
        require(rate > 0, "Invalid barter rate");

        uint256 toAmount = (fromAmount * rate) / 1e18;
        require(toAmount > 0, "Resulting toAmount too low");

        (uint256 balance, uint256 lineageID) =
            meatToken.balanceOfSubtypeWithLineage(msg.sender, fromSubtype);
        require(balance >= fromAmount, "Insufficient subtype balance");
        require(lineageID != 0, "Invalid lineage");

        meatToken.burnSubtype(msg.sender, fromSubtype, fromAmount);
        meatToken.mintSubtype(msg.sender, toSubtype, toAmount);

        emit BarterExecuted(msg.sender, fromSubtype, fromAmount, toSubtype, toAmount);
    }

    /// @notice View function to get current PRODUCT↔PRODUCT rate
    function getCurrentBarterRate(bytes32 fromSubtype, bytes32 toSubtype)
        external
        view
        returns (uint256)
    {
        return
            rateHandler.computeBarterRate(
                fromSubtype,
                "PRODUCT",
                toSubtype,
                "PRODUCT"
            );
    }

    /// @notice Returns owner
    function owner() external view returns (address) {
        return _owner;
    }
}

