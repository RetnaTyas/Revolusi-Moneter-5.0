// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IGOAT } from "./interfaces/IGOAT.sol";
import { SwapConfig } from "./SwapConfig.sol";
import { RateHandler } from "./RateHandler.sol";

/// @title Token MEAT - Monetary Exchange for Agricultural Transactions
/// @notice Kontrak pintar untuk mencetak MEAT menggunakan token native dan melakukan swap dengan GOAT.
/// Dirancang sebagai revolusi moneter berbasis aset peternakan dan pertanian.
contract MEAT is ERC20 {
    IGOAT public GOAT;
    address private immutable _owner;

    // Authorized addresses allowed to mint or burn subtype balances
    mapping(address => bool) public isMinter;
    mapping(address => bool) public isBurner;

    // Balance per user per subtype
    mapping(address => mapping(bytes32 => uint256)) public subtypeBalances;

    // Total supply per subtype
    mapping(bytes32 => uint256) public subtypeTotalSupply;

    uint256 private _rate = 100;
    /// @notice Divisor untuk perhitungan jumlah MEAT yang dicetak saat menerima
    /// token native. Nilai default 1000 berarti deposit rate dihitung per 1000
    /// unit native token.
    uint256 public constant DEPOSIT_DIVISOR = 1000;
    RateHandler public rateHandler;

    event DepositRateChanged(uint256 oldRate, uint256 newRate);
    event MintedWithNative(address indexed user, uint256 nativeReceived, uint256 meatMinted);
    event NativeWithdrawn(address indexed to, uint256 amount);
    event InitialSupplyMinted(address indexed to, uint256 amount);
    event MeatRedeemed(address indexed user, uint256 amount);
    event GoatAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event RateHandlerUpdated(address indexed oldAddress, address indexed newAddress);
    event SubtypeMinted(address indexed to, bytes32 indexed subtype, uint256 amount);
    event SubtypeBurned(address indexed from, bytes32 indexed subtype, uint256 amount);

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    modifier onlyMinter() {
        require(isMinter[msg.sender], "Not minter");
        _;
    }

    modifier onlyBurner() {
        require(isBurner[msg.sender], "Not burner");
        _;
    }

    constructor(address goatAddress) ERC20("Market-Enabled Agricultural Token", "MEAT") {
        require(goatAddress != address(0), "Invalid address");
        GOAT = IGOAT(goatAddress);
        _owner = msg.sender;

        uint256 initialSupply = 1000 * 1e18;
        _mint(_owner, initialSupply);
        emit InitialSupplyMinted(_owner, initialSupply);
    }

    /// @notice Menerima token native dan mencetak MEAT sesuai rate
    receive() external payable {
        require(msg.value != 0, "Must send Native Token to mint MEAT");
        uint256 meatAmount = (msg.value * _rate) / DEPOSIT_DIVISOR;
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


    function redeemForMeat(uint256 amount) external {
        require(amount > 0, "Amount must be > 0");
        _burn(msg.sender, amount);
        emit MeatRedeemed(msg.sender, amount);
    }

    function setGOATAddress(address goatAddress) external onlyOwner {
        require(goatAddress != address(0), "Invalid address");
        address old = address(GOAT);
        GOAT = IGOAT(goatAddress);
        emit GoatAddressUpdated(old, goatAddress);
    }

    /// @notice Menetapkan atau mencabut hak minter untuk alamat tertentu
    function setMinter(address account, bool status) external onlyOwner {
        isMinter[account] = status;
    }

    /// @notice Menetapkan atau mencabut hak burner untuk alamat tertentu
    function setBurner(address account, bool status) external onlyOwner {
        isBurner[account] = status;
    }

    function mintSubtype(address to, bytes32 subtype, uint256 amount) public onlyMinter {
        require(subtype != bytes32(0), "Invalid subtype");
        require(amount > 0, "Invalid amount");

        subtypeBalances[to][subtype] += amount;
        subtypeTotalSupply[subtype] += amount;

        _mint(to, amount);

        emit SubtypeMinted(to, subtype, amount);
    }

    function burnSubtype(address from, bytes32 subtype, uint256 amount) public onlyBurner {
        require(subtype != bytes32(0), "Invalid subtype");
        require(amount > 0, "Invalid amount");
        require(subtypeBalances[from][subtype] >= amount, "Insufficient subtype balance");

        subtypeBalances[from][subtype] -= amount;
        subtypeTotalSupply[subtype] -= amount;

        _burn(from, amount);

        emit SubtypeBurned(from, subtype, amount);
    }

    function getBalanceOfSubtype(address user, bytes32 subtype) public view returns (uint256) {
        return subtypeBalances[user][subtype];
    }

    function getTotalSupplyOfSubtype(bytes32 subtype) public view returns (uint256) {
        return subtypeTotalSupply[subtype];
    }

    /// @notice Mengatur alamat kontrak rate handler untuk perhitungan swap
    /// @param rateHandlerAddress Alamat kontrak RateHandler
    function setRateHandler(address rateHandlerAddress) external onlyOwner {
        require(rateHandlerAddress != address(0), "Invalid address");
        address old = address(rateHandler);
        rateHandler = RateHandler(rateHandlerAddress);
        emit RateHandlerUpdated(old, rateHandlerAddress);
    }

    function _getSwapRate() internal view returns (uint256) {
        if (address(rateHandler) != address(0)) {
            return rateHandler.getCurrentRate();
        }
        return SwapConfig.SWAP_RATE;
    }

    function owner() external view returns (address) {
        return _owner;
    }

    function getEquivalentMEAT(uint256 goatAmount) external view returns (uint256) {
        uint256 rate = _getSwapRate();
        return goatAmount * rate;
    }

    function getEquivalentGOAT(uint256 meatAmount) external view returns (uint256) {
        uint256 rate = _getSwapRate();
        return meatAmount / rate;
    }
}
