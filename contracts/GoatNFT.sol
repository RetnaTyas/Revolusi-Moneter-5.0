// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC721Burnable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {GoatNFTBurnHook} from "./GoatNFTBurnHook.sol";

/// @title GoatNFT - identitas kambing dalam bentuk token
/// @notice Setiap NFT menyimpan nilai berat kambing hidup, yang dapat diperbarui.
/// @dev Burn NFT memicu mint GOATMEAT token via GoatNFTBurnHook, tidak lagi mencetak GOAT token.
/// @dev Alur:
/// - Mint: Menyimpan berat awal kambing.
/// - UpdateWeight: Memperbarui berat terbaru.
/// - Burn: Membutuhkan berat terbaru (valid), memicu burnHook untuk mint GOATMEAT, lalu membakar NFT.
contract GoatNFT is ERC721Burnable, Ownable {
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

    GoatNFTBurnHook public burnHook;

    /// @notice Jangka waktu validitas pembaruan berat dalam detik (7 hari)
    uint256 public constant WEIGHT_UPDATE_VALIDITY = 7 days;
    /// @notice Jumlah digit desimal yang digunakan untuk berat
    uint256 public constant WEIGHT_DECIMALS = 1;

    constructor() ERC721("Goat Identifier", "GOATNFT") Ownable(msg.sender) {}

    /// @notice Mengatur kontrak hook yang dipanggil saat NFT dibakar
    function setBurnHook(address hookAddress) external onlyOwner {
        address old = address(burnHook);
        burnHook = GoatNFTBurnHook(hookAddress);
        emit BurnHookUpdated(old, hookAddress);
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

    /// @notice Membakar NFT dan memicu mint GOATMEAT melalui burnHook
    /// @param tokenId ID NFT yang dibakar
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

        super.burn(tokenId);
        delete goatValue[tokenId];
        delete goatMetadata[tokenId];
        delete lastWeightUpdateAt[tokenId];
        delete nfcIdToTokenId[hash];

        // Memicu burnHook untuk mint GOATMEAT
        if (address(burnHook) != address(0)) {
            burnHook.onBurn(tokenOwner, currentWeight);
        }

        emit GoatBurned(tokenId, tokenOwner, currentWeight);
    }

    function getGoatData(uint256 tokenId) external view returns (GoatData memory) {
        return goatMetadata[tokenId];
    }



    /// @notice Dipancarkan ketika alamat kontrak hook diubah
    event BurnHookUpdated(address indexed oldAddress, address indexed newAddress);
    /// @notice Dipancarkan ketika NFT dibakar dan burnHook dipanggil
    event GoatBurned(uint256 indexed tokenId, address indexed user, uint256 weight);
    /// @notice Dipancarkan ketika berat kambing diperbarui
    event WeightUpdated(uint256 indexed tokenId, uint256 newWeight);
}
