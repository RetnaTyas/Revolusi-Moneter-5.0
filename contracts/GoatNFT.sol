// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC721Burnable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

/// @title GoatNFT - tokenized goat identification
/// @notice Each NFT holds a value redeemable for GOAT tokens
contract GoatNFT is ERC721Burnable {
    uint256 public nextId;
    mapping(uint256 => uint256) public goatValue;
    address private immutable _owner;

    constructor() ERC721("Goat Identifier", "GOATNFT") {
        _owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == _owner, "Not the owner");
        _;
    }

    function mint(address to, uint256 value) external onlyOwner returns (uint256) {
        require(value > 0, "Value must be > 0");
        uint256 tokenId = ++nextId;
        goatValue[tokenId] = value;
        _mint(to, tokenId);
        return tokenId;
    }

    function burn(uint256 tokenId) public override {
        address tokenOwner = ownerOf(tokenId);
        require(_isAuthorized(tokenOwner, msg.sender, tokenId), "Not owner");
        super.burn(tokenId);
        delete goatValue[tokenId];
    }

    function owner() external view returns (address) {
        return _owner;
    }
}
