// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IGOAT } from "./interfaces/IGOAT.sol";

/// @title MEAT Token - Monetary Exchange for Agricultural Transactions
/// @notice Smart contract untuk mint MEAT pakai Native Token dan swap ke GOAT serta sebaliknya.
/// Dirancang untuk revolusi moneter berbasis aset peternakan dan pertanian.
contract MEAT is ERC20 {
    IGOAT public GOAT;
    address private immutable _owner;

    uint256 private _rate = 100;
    uint256 public constant SwapRate = 85;
    bool public swapEnabled = true;

    event DepositRateChanged(uint256 oldRate, uint256 newRate);
    event MintedWithNative(address indexed user, uint256 nativeReceived, uint256 meatMinted);
    event NativeWithdrawn(address indexed to, uint256 amount);
    event SwappedGOATForMEAT(address indexed user, uint256 goatIn, uint256 meatOut);
    event SwappedMEATForGOAT(address indexed user, uint256 meatIn, uint256 goatOut);
    event InitialSupplyMinted(address indexed to, uint256 amount);
    event MeatRedeemed(address indexed user, uint256 amount);

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    constructor(address goatAddress) ERC20("Market-Enabled Agricultural Token", "MEAT") {
        GOAT = IGOAT(goatAddress);
        _owner = msg.sender;

        uint256 initialSupply = 1000 * 1e18;
        _mint(_owner, initialSupply);
        emit InitialSupplyMinted(_owner, initialSupply);
    }

    /// @notice Terima native token & mint MEAT berdasarkan rate
    receive() external payable {
        require(msg.value != 0, "Must send Native Token to mint MEAT");
        uint256 meatAmount = (msg.value * _rate) / 1e3;
        require(meatAmount != 0, "Mint amount too low");

        uint256 contractBalance = balanceOf(address(this));
        if (contractBalance >= meatAmount) {
            _transfer(address(this), msg.sender, meatAmount);
        } else {
            if (contractBalance > 0) {
                _transfer(address(this), msg.sender, contractBalance);
            }
            uint256 mintAmount = meatAmount - contractBalance;
            _mint(msg.sender, mintAmount);
        }

        emit MintedWithNative(msg.sender, msg.value, meatAmount);
    }

    function withdrawNative() external onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "No Native Token to withdraw");
        (bool sent, ) = payable(_owner).call{value: balance}("");
        require(sent, "Native transfer failed");
        emit NativeWithdrawn(_owner, balance);
    }

    function changeDepositRate(uint256 newRate) external onlyOwner {
        require(newRate > 0, "Rate must be greater than zero");
        uint256 oldRate = _rate;
        _rate = newRate;
        emit DepositRateChanged(oldRate, newRate);
    }

    function DepositRate() external view returns (uint256) {
        return _rate;
    }

    function swapGOATForMEAT(uint256 goatAmount) external {
        require(swapEnabled, "Swap disabled");
        require(goatAmount > 0, "Amount must be > 0");
        GOAT.transferFrom(msg.sender, address(this), goatAmount);

        uint256 meatAmount = goatAmount * SwapRate;
        uint256 contractBalance = balanceOf(address(this));
        if (contractBalance >= meatAmount) {
            _transfer(address(this), msg.sender, meatAmount);
        } else {
            if (contractBalance > 0) {
                _transfer(address(this), msg.sender, contractBalance);
            }
            uint256 remaining = meatAmount - contractBalance;
            _mint(msg.sender, remaining);
        }

        emit SwappedGOATForMEAT(msg.sender, goatAmount, meatAmount);
    }

    function swapMEATForGOAT(uint256 meatAmount) external {
        require(swapEnabled, "Swap disabled");
        require(meatAmount > 0, "Amount must be > 0");
        IERC20(address(this)).transferFrom(msg.sender, address(this), meatAmount);

        uint256 goatAmount = meatAmount / SwapRate;
        uint256 goatBalance = GOAT.balanceOf(address(this));
        if (goatBalance < goatAmount) {
            GOAT.mintTo(address(this), goatAmount - goatBalance);
        }
        GOAT.transfer(msg.sender, goatAmount);
        emit SwappedMEATForGOAT(msg.sender, meatAmount, goatAmount);
    }

    function redeemForMeat(uint256 amount) external {
        require(amount > 0, "Amount must be > 0");
        _burn(msg.sender, amount);
        emit MeatRedeemed(msg.sender, amount);
    }

    function setSwapEnabled(bool status) external onlyOwner {
        swapEnabled = status;
    }

    function setGOATAddress(address goatAddress) external onlyOwner {
        require(goatAddress != address(0), "Invalid address");
        GOAT = IGOAT(goatAddress);
    }

    function owner() external view returns (address) {
        return _owner;
    }

    function getEquivalentMEAT(uint256 goatAmount) external pure returns (uint256) {
        return goatAmount * SwapRate;
    }

    function getEquivalentGOAT(uint256 meatAmount) external pure returns (uint256) {
        return meatAmount / SwapRate;
    }
}
