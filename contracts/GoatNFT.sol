// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC721Burnable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {IGoatToken} from "./interfaces/IGoatToken.sol";
import {SwapConfig} from "./SwapConfig.sol";
import {RateHandler} from "./RateHandler.sol";

/// @title GoatNFT - identitas kambing dalam bentuk token
/// @notice Setiap NFT menyimpan nilai berat yang dapat ditebus menjadi token GOAT
/// @dev Memastikan pembaruan berat terkini sebelum dibakar agar jumlah GOAT yang dicetak sesuai nilai komoditas
/// @dev Alur:
/// - Mint: Menyimpan berat awal kambing
/// - UpdateWeight: Memperbarui berat terbaru
/// - Burn: Membutuhkan berat terbaru, mencetak GOAT berdasarkan berat terakhir, lalu membakar NFT
contract GoatNFT is ERC721Burnable {
    uint256 public nextId;
    mapping(uint256 => uint256) public goatValue;

    struct GoatData {
        string nfcId;
        string breed;
        uint256 birthYear;
        uint256 weight;
        uint256 mintedAt;
    }

    mapping(uint256 => GoatData) public goatMetadata;
    mapping(uint256 => uint256) public lastWeightUpdateAt;
    mapping(bytes32 => uint256) public nfcIdToTokenId;

    address private immutable _owner;
    IGoatToken public goatTokenContract;
    RateHandler public rateHandler;

    /// @notice Jangka waktu validitas pembaruan berat dalam detik (7 hari)
    uint256 public constant WEIGHT_UPDATE_VALIDITY = 7 days;
    /// @notice Jumlah digit desimal yang digunakan untuk berat
    uint256 public constant WEIGHT_DECIMALS = 1;

    constructor(address goatTokenAddress) ERC721("Goat Identifier", "GOATNFT") {
        require(goatTokenAddress != address(0), "Invalid address");
        _owner = msg.sender;
        goatTokenContract = IGoatToken(goatTokenAddress);
    }

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    function setGoatTokenContract(address goatTokenAddress) external onlyOwner {
        require(goatTokenAddress != address(0), "Invalid address");
        address old = address(goatTokenContract);
        goatTokenContract = IGoatToken(goatTokenAddress);
        emit GoatTokenAddressUpdated(old, goatTokenAddress);
    }

    /// @notice Mengatur kontrak rate handler yang dipakai untuk jumlah mint GOAT
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

    /// @param weight Nilai berat yang diskalakan dengan `WEIGHT_DECIMALS`
    function mint(
        address to,
        uint256 weight,
        string memory nfcId,
        string memory breed,
        uint256 birthYear
    ) external onlyOwner returns (uint256) {
        require(weight > 0, "Weight must be > 0");
        bytes32 hash = keccak256(bytes(nfcId));
        require(nfcIdToTokenId[hash] == 0, "NFC ID already used");
        uint256 tokenId = ++nextId;
        nfcIdToTokenId[hash] = tokenId;
        goatValue[tokenId] = weight;
        goatMetadata[tokenId] = GoatData(nfcId, breed, birthYear, weight, block.timestamp);
        lastWeightUpdateAt[tokenId] = block.timestamp;
        _mint(to, tokenId);
        return tokenId;
    }

    /// @notice Memperbarui berat terkini kambing
    /// @param tokenId ID NFT yang diperbarui
    /// @param newWeight Nilai berat baru yang diskalakan dengan `WEIGHT_DECIMALS`
    function updateWeight(uint256 tokenId, uint256 newWeight) external {
        address tokenOwner = ownerOf(tokenId);
        require(msg.sender == tokenOwner, "Not token owner");
        require(newWeight > 0, "Weight must be > 0");

        goatValue[tokenId] = newWeight;
        goatMetadata[tokenId].weight = newWeight;
        lastWeightUpdateAt[tokenId] = block.timestamp;
        emit WeightUpdated(tokenId, newWeight);
    }

    function burn(uint256 tokenId) public override {
        address tokenOwner = ownerOf(tokenId);
        require(_isAuthorized(tokenOwner, msg.sender, tokenId), "Not owner");
        require(
            block.timestamp - lastWeightUpdateAt[tokenId] <= WEIGHT_UPDATE_VALIDITY,
            "Weight update too old"
        );

        uint256 currentWeight = goatValue[tokenId];
        require(currentWeight > 0, "Invalid weight");

        bytes32 hash = keccak256(bytes(goatMetadata[tokenId].nfcId));

        uint256 rate = _getSwapRate();
        uint256 goatAmount =
            (currentWeight * 1e18) / rate / (10 ** WEIGHT_DECIMALS);

        // Mencetak token GOAT langsung ke pemilik NFT
        goatTokenContract.mint(tokenOwner, goatAmount);

        emit GoatBurned(tokenId, tokenOwner, currentWeight, goatAmount);

        super.burn(tokenId);
        delete goatValue[tokenId];
        delete goatMetadata[tokenId];
        delete lastWeightUpdateAt[tokenId];
        delete nfcIdToTokenId[hash];
    }

    function getGoatData(uint256 tokenId) external view returns (GoatData memory) {
        return goatMetadata[tokenId];
    }

    function owner() external view returns (address) {
        return _owner;
    }

    /// @notice Dipancarkan ketika alamat kontrak token GOAT berubah
    event GoatTokenAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event RateHandlerUpdated(address indexed oldAddress, address indexed newAddress);
    /// @notice Dipancarkan ketika NFT dibakar dan token GOAT otomatis dicetak
    event GoatBurned(uint256 indexed tokenId, address indexed user, uint256 weight, uint256 goatAmount);
    /// @notice Dipancarkan ketika berat kambing diperbarui
    event WeightUpdated(uint256 indexed tokenId, uint256 newWeight);
}
