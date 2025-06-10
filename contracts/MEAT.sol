// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { RateHandler } from "./RateHandler.sol";

/// @title Token MEAT - Market-Enabled Agricultural Token
/// @notice Kontrak pintar untuk mint MEAT menggunakan native token dan PRODUCT subtype token.
/// @dev Reasoning Path FINAL Compliant → NO awareness of GOAT token. NO swap GOAT ↔ MEAT allowed.

contract MEAT is ERC20 {
    address private immutable _owner;

    // Authorized addresses allowed to mint or burn subtype balances
    mapping(address => bool) public isMinter;
    mapping(address => bool) public isBurner;

    struct SubtypeBalance {
        uint256 balance;
        uint256 lineageID;
    }

    // Balance per user per subtype with lineage info
    mapping(address => mapping(bytes32 => SubtypeBalance)) public subtypeBalances;

    // Total supply per subtype
    mapping(bytes32 => uint256) public subtypeTotalSupply;

    uint256 private _rate = 100;
    /// @notice Divisor untuk perhitungan jumlah MEAT yang dicetak saat menerima token native.
    uint256 public constant DEPOSIT_DIVISOR = 1000;

    RateHandler public rateHandler;

    event DepositRateChanged(uint256 oldRate, uint256 newRate);
    event MintedWithNative(address indexed user, uint256 nativeReceived, uint256 meatMinted);
    event NativeWithdrawn(address indexed to, uint256 amount);
    event InitialSupplyMinted(address indexed to, uint256 amount);
    event MeatRedeemed(address indexed user, uint256 amount);
    event RateHandlerUpdated(address indexed oldAddress, address indexed newAddress);
    event SubtypeMinted(address indexed to, bytes32 indexed subtype, uint256 amount);
    event SubtypeBurned(address indexed from, bytes32 indexed subtype, uint256 amount);
    event SubtypeLineageUpdated(address indexed user, bytes32 indexed subtype, uint256 lineageID);

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

    constructor() ERC20("Market-Enabled Agricultural Token", "MEAT") {
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

        SubtypeBalance storage s = subtypeBalances[to][subtype];
        s.balance += amount;
        subtypeTotalSupply[subtype] += amount;

        _mint(to, amount);

        emit SubtypeMinted(to, subtype, amount);
    }

    function burnSubtype(address from, bytes32 subtype, uint256 amount) public onlyBurner {
        require(subtype != bytes32(0), "Invalid subtype");
        require(amount > 0, "Invalid amount");
        SubtypeBalance storage s = subtypeBalances[from][subtype];
        require(s.balance >= amount, "Insufficient subtype balance");

        s.balance -= amount;
        subtypeTotalSupply[subtype] -= amount;

        _burn(from, amount);

        emit SubtypeBurned(from, subtype, amount);
    }

    function getBalanceOfSubtype(address user, bytes32 subtype) public view returns (uint256) {
        return subtypeBalances[user][subtype].balance;
    }

    function getTotalSupplyOfSubtype(bytes32 subtype) public view returns (uint256) {
        return subtypeTotalSupply[subtype];
    }

    function setSubtypeLineage(address user, bytes32 subtype, uint256 lineageID) external onlyOwner {
        SubtypeBalance storage s = subtypeBalances[user][subtype];
        s.lineageID = lineageID;
        emit SubtypeLineageUpdated(user, subtype, lineageID);
    }

    function balanceOfSubtypeWithLineage(address user, bytes32 subtype) external view returns (uint256 balance, uint256 lineageID) {
        SubtypeBalance storage s = subtypeBalances[user][subtype];
        return (s.balance, s.lineageID);
    }

    /// @notice Mengatur alamat kontrak rate handler untuk perhitungan swap (dipakai di BarterContract, bukan di MEAT)
    function setRateHandler(address rateHandlerAddress) external onlyOwner {
        require(rateHandlerAddress != address(0), "Invalid address");
        address old = address(rateHandler);
        rateHandler = RateHandler(rateHandlerAddress);
        emit RateHandlerUpdated(old, rateHandlerAddress);
    }

    /// @notice Mengembalikan alamat pemilik kontrak
    function owner() external view returns (address) {
        return _owner;
    }
}
