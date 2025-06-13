// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { RateHandler } from "./RateHandler.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

/// @title Token MEAT - Market-Enabled Agricultural Token
/// @notice Kontrak pintar untuk mint MEAT menggunakan native token dan PRODUCT subtype token.
/// @dev Reasoning Path FINAL Compliant → NO awareness of GOAT token. NO swap GOAT ↔ MEAT allowed.

/// @dev Subtype parameters expect bytes32 values from ethers.encodeBytes32String
contract MEAT is ERC20, Ownable {

    // Authorized addresses allowed to mint or burn subtype balances
    mapping(address => bool) public isMinter;
    mapping(address => bool) public isBurner;

    bytes32 public constant GOATMEAT_SUBTYPE = keccak256(abi.encodePacked("GOATMEAT"));

    struct SubtypeBalance {
        uint256 balance;
        uint256 lineageID;
    }

    // Balance per user per subtype with lineage info
    // Subtype values must be bytes32 generated via ethers.encodeBytes32String
    mapping(address => mapping(bytes32 => SubtypeBalance)) public subtypeBalances;

    // List of owned subtypes per user for iteration
    mapping(address => bytes32[]) private _userSubtypes;

    // Total supply per subtype
    mapping(bytes32 => uint256) public subtypeTotalSupply;

    uint256 private _rate = 100;
    /// @notice Divisor untuk perhitungan jumlah MEAT yang dicetak saat menerima token native.
    uint256 public constant DEPOSIT_DIVISOR = 1000;

    RateHandler public rateHandler;

    event DepositRateChanged(uint256 oldRate, uint256 newRate);
    event MintedWithNative(address indexed user, uint256 nativeReceived, uint256 meatMinted);
    event NativeWithdrawn(address indexed to, uint256 amount);
    event MeatRedeemed(address indexed user, uint256 amount);
    event RateHandlerUpdated(address indexed oldAddress, address indexed newAddress);
    event SubtypeMinted(address indexed to, bytes32 indexed subtype, uint256 amount);
    event SubtypeBurned(address indexed from, bytes32 indexed subtype, uint256 amount);
    event SubtypeLineageUpdated(address indexed user, bytes32 indexed subtype, uint256 lineageID);
    event MinterUpdated(address indexed account, bool status);
    event BurnerUpdated(address indexed account, bool status);

    // ----- internal helpers for subtype tracking -----
    function _addUserSubtype(address user, bytes32 subtype) internal {
        bytes32[] storage list = _userSubtypes[user];
        for (uint256 i = 0; i < list.length; i++) {
            if (list[i] == subtype) {
                return;
            }
        }
        list.push(subtype);
    }

    function _removeUserSubtype(address user, bytes32 subtype) internal {
        bytes32[] storage list = _userSubtypes[user];
        for (uint256 i = 0; i < list.length; i++) {
            if (list[i] == subtype) {
                list[i] = list[list.length - 1];
                list.pop();
                break;
            }
        }
    }



    modifier onlyMinter() {
        require(isMinter[msg.sender], "Not minter");
        _;
    }

    modifier onlyBurner() {
        require(isBurner[msg.sender], "Not burner");
        _;
    }

    modifier onlyOwnerOrMinter() {
        require(msg.sender == owner() || isMinter[msg.sender], "Not authorized");
        _;
    }

    /// @notice Constructor grants the deployer minter rights
    ///         and mints 1000 GOATMEAT via `mintSubtype` to the owner.
    /// @dev Initial supply is tracked through the `SubtypeMinted` event
    ///      emitted by `mintSubtype`.
    constructor() ERC20("Market-Enabled Agricultural Token", "MEAT") Ownable(msg.sender) {

        isMinter[owner()] = true;

        // Mint 1000 GOATMEAT to the owner via mintSubtype
        mintSubtype(owner(), GOATMEAT_SUBTYPE, 1000 * 1e18);
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
        (bool sent, ) = payable(owner()).call{value: balance}("");
        require(sent, "Native transfer failed");
        emit NativeWithdrawn(owner(), balance);
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
        emit MinterUpdated(account, status);
    }

    /// @notice Menetapkan atau mencabut hak burner untuk alamat tertentu
    function setBurner(address account, bool status) external onlyOwner {
        isBurner[account] = status;
        emit BurnerUpdated(account, status);
    }

    function mintSubtype(address to, bytes32 subtype, uint256 amount) public onlyMinter {
        require(subtype != bytes32(0), "Invalid subtype");
        require(amount > 0, "Invalid amount");

        SubtypeBalance storage s = subtypeBalances[to][subtype];
        if (s.balance == 0) {
            _addUserSubtype(to, subtype);
        }
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
        if (s.balance == 0) {
            _removeUserSubtype(from, subtype);
        }
        subtypeTotalSupply[subtype] -= amount;

        if (msg.sender != from) {
            _spendAllowance(from, msg.sender, amount);
        }

        _burn(from, amount);

        emit SubtypeBurned(from, subtype, amount);
    }

    function getBalanceOfSubtype(address user, bytes32 subtype) public view returns (uint256) {
        return subtypeBalances[user][subtype].balance;
    }

    function getTotalSupplyOfSubtype(bytes32 subtype) public view returns (uint256) {
        return subtypeTotalSupply[subtype];
    }

    function setSubtypeLineage(address user, bytes32 subtype, uint256 lineageID) external onlyOwnerOrMinter {
        SubtypeBalance storage s = subtypeBalances[user][subtype];
        s.lineageID = lineageID;
        emit SubtypeLineageUpdated(user, subtype, lineageID);
    }

    function balanceOfSubtypeWithLineage(address user, bytes32 subtype) external view returns (uint256 balance, uint256 lineageID) {
        SubtypeBalance storage s = subtypeBalances[user][subtype];
        return (s.balance, s.lineageID);
    }

    /// @dev Move subtype balances and lineage metadata during ERC20 transfers
    function _update(
        address from,
        address to,
        uint256 amount
    ) internal override {
        super._update(from, to, amount);

        if (from == address(0) || to == address(0) || amount == 0) {
            return;
        }

        uint256 remaining = amount;
        bytes32[] storage list = _userSubtypes[from];
        uint256 i = 0;
        while (i < list.length && remaining > 0) {
            bytes32 st = list[i];
            SubtypeBalance storage sFrom = subtypeBalances[from][st];
            uint256 available = sFrom.balance;
            if (available == 0) {
                list[i] = list[list.length - 1];
                list.pop();
                continue;
            }

            uint256 tAmt = remaining > available ? available : remaining;
            sFrom.balance = available - tAmt;
            if (sFrom.balance == 0) {
                _removeUserSubtype(from, st);
                // list may change, reload
                list = _userSubtypes[from];
            } else {
                i++;
            }

            SubtypeBalance storage sTo = subtypeBalances[to][st];
            if (sTo.balance == 0) {
                _addUserSubtype(to, st);
                sTo.lineageID = sFrom.lineageID;
                emit SubtypeLineageUpdated(to, st, sFrom.lineageID);
            } else {
                require(
                    sTo.lineageID == sFrom.lineageID,
                    "Lineage mismatch"
                );
            }
            sTo.balance += tAmt;
            remaining -= tAmt;
        }
    }

    /// @notice Mengatur alamat kontrak rate handler untuk perhitungan swap (dipakai di BarterEngine, bukan di MEAT)
    function setRateHandler(address rateHandlerAddress) external onlyOwner {
        require(rateHandlerAddress != address(0), "Invalid address");
        address old = address(rateHandler);
        rateHandler = RateHandler(rateHandlerAddress);
        emit RateHandlerUpdated(old, rateHandlerAddress);
    }

    // Ownable already exposes owner() view
}
