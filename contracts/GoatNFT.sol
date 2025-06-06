// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC721Burnable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

/// @title GoatNFT - tokenized goat identification
/// @notice Each NFT holds a value redeemable for GOAT tokens
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
    address private immutable _owner;

    constructor() ERC721("Goat Identifier", "GOATNFT") {
        _owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    function mint(
        address to,
        uint256 value,
        string memory nfcId,
        string memory breed,
        uint256 birthYear,
        uint256 weight
    ) external onlyOwner returns (uint256) {
        require(value > 0, "Value must be > 0");
        uint256 tokenId = ++nextId;
        goatValue[tokenId] = value;
        goatMetadata[tokenId] = GoatData(nfcId, breed, birthYear, weight, block.timestamp);
        _mint(to, tokenId);
        return tokenId;
    }

    function burn(uint256 tokenId) public override {
        address tokenOwner = ownerOf(tokenId);
        require(_isAuthorized(tokenOwner, msg.sender, tokenId), "Not owner");
        super.burn(tokenId);
        delete goatValue[tokenId];
        delete goatMetadata[tokenId];
    }

    function getGoatData(uint256 tokenId) external view returns (GoatData memory) {
        return goatMetadata[tokenId];
    }

    function owner() external view returns (address) {
        return _owner;
    }
}
