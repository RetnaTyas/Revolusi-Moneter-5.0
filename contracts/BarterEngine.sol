// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { RateHandler } from "./RateHandler.sol";
import { MEAT } from "./MEAT.sol";

/// @title BarterEngine v1
/// @notice Enables PRODUCT↔PRODUCT swap based on RateHandler LOD parity.
/// @dev Fully Reasoning Path FINAL Compliant. NO cross-layer swap allowed.
/// @dev Subtype args are bytes32 generated via ethers.encodeBytes32String
contract BarterEngine {
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

    event MeatSubtypeWithdrawn(
        address indexed to,
        bytes32 indexed subtype,
        uint256 amount
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
    /// Subtype parameters must be bytes32 from ethers.encodeBytes32String
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
        meatToken.setSubtypeLineage(msg.sender, toSubtype, lineageID);

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

    /// @notice Emergency withdraw stuck MEAT subtype from this contract
    /// @param subtype Subtype to withdraw (bytes32 encoded string)
    /// @param amount Amount of subtype to withdraw
    /// @param useTransfer If true, call transferSubtype; otherwise burn+mint
    function emergencyWithdrawMEATSubtype(
        bytes32 subtype,
        uint256 amount,
        bool useTransfer
    ) external onlyOwner {
        require(subtype != bytes32(0), "Invalid subtype");
        require(amount > 0, "Amount must be > 0");

        uint256 balance = meatToken.getBalanceOfSubtype(address(this), subtype);
        require(balance >= amount, "Insufficient subtype balance");

        if (useTransfer) {
            (bool success, ) = address(meatToken).call(
                abi.encodeWithSignature(
                    "transferSubtype(address,address,bytes32,uint256)",
                    address(this),
                    _owner,
                    subtype,
                    amount
                )
            );
            require(success, "transferSubtype failed");
        } else {
            meatToken.burnSubtype(address(this), subtype, amount);
            meatToken.mintSubtype(_owner, subtype, amount);
        }

        emit MeatSubtypeWithdrawn(_owner, subtype, amount);
    }

    /// @notice Returns owner
    function owner() external view returns (address) {
        return _owner;
    }
}

