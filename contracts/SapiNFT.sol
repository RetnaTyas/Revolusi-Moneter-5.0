// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC721Burnable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {SapiNFTBurnHook} from "./SapiNFTBurnHook.sol";

/// @title SapiNFT - identitas sapi dalam bentuk token
/// @notice Menyimpan berat sapi hidup dan metadata terkait
contract SapiNFT is ERC721Burnable, Ownable {
    uint256 public nextId;
    mapping(uint256 => uint256) public sapiValue;

    struct SapiData {
        string nfcId;
        string breed;
        uint256 birthYear;
        uint256 weight;
        uint256 mintedAt;
    }

    mapping(uint256 => SapiData) public sapiMetadata;
    mapping(uint256 => uint256) public lastWeightUpdateAt;
    mapping(bytes32 => uint256) public nfcIdToTokenId;

    SapiNFTBurnHook public burnHook;

    uint256 public constant WEIGHT_UPDATE_VALIDITY = 7 days;
    uint256 public constant WEIGHT_DECIMALS = 1;

    constructor() ERC721("Sapi Identifier", "SAPINFT") Ownable(msg.sender) {}

    function setBurnHook(address hookAddress) external onlyOwner {
        address old = address(burnHook);
        burnHook = SapiNFTBurnHook(hookAddress);
        emit BurnHookUpdated(old, hookAddress);
    }

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
        sapiValue[tokenId] = weight;
        sapiMetadata[tokenId] = SapiData(nfcId, breed, birthYear, weight, block.timestamp);
        lastWeightUpdateAt[tokenId] = block.timestamp;
        _mint(to, tokenId);
        return tokenId;
    }

    function updateWeight(uint256 tokenId, uint256 newWeight) external {
        address tokenOwner = ownerOf(tokenId);
        require(msg.sender == tokenOwner, "Not token owner");
        require(newWeight > 0, "Weight must be > 0");

        sapiValue[tokenId] = newWeight;
        sapiMetadata[tokenId].weight = newWeight;
        lastWeightUpdateAt[tokenId] = block.timestamp;
        emit WeightUpdated(tokenId, newWeight);
    }

    function burn(uint256 tokenId) public override {
        address tokenOwner = ownerOf(tokenId);
        require(_isAuthorized(tokenOwner, msg.sender, tokenId), "Not owner");
        require(block.timestamp - lastWeightUpdateAt[tokenId] <= WEIGHT_UPDATE_VALIDITY, "Weight update too old");

        uint256 currentWeight = sapiValue[tokenId];
        require(currentWeight > 0, "Invalid weight");

        bytes32 hash = keccak256(bytes(sapiMetadata[tokenId].nfcId));

        super.burn(tokenId);
        delete sapiValue[tokenId];
        delete sapiMetadata[tokenId];
        delete lastWeightUpdateAt[tokenId];
        delete nfcIdToTokenId[hash];

        if (address(burnHook) != address(0)) {
            burnHook.onBurn(tokenOwner, currentWeight);
        }

        emit SapiBurned(tokenId, tokenOwner, currentWeight);
    }

    function getSapiData(uint256 tokenId) external view returns (SapiData memory) {
        return sapiMetadata[tokenId];
    }



    event BurnHookUpdated(address indexed oldAddress, address indexed newAddress);
    event SapiBurned(uint256 indexed tokenId, address indexed user, uint256 weight);
    event WeightUpdated(uint256 indexed tokenId, uint256 newWeight);
}
