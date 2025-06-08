// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ERC721Burnable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {IGoatToken} from "./interfaces/IGoatToken.sol";
import {SwapConfig} from "./SwapConfig.sol";
import {RateHandler} from "./RateHandler.sol";

/// @title GoatNFT - tokenized goat identification
/// @notice Each NFT holds a weight value redeemable for GOAT tokens
/// @dev Enforces fresh weight updates before burning so GOAT minted matches the actual commodity value.
/// @dev Flow:
/// - Mint: Stores initial goat weight.
/// - UpdateWeight: Updates latest weight.
/// - Burn: Requires fresh weight update, mints GOAT based on latest weight, burns NFT.
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

    /// @notice Weight update validity window in seconds (7 days)
    uint256 public constant WEIGHT_UPDATE_VALIDITY = 7 days;

    constructor(address goatTokenAddress) ERC721("Goat Identifier", "GOATNFT") {
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

    /// @notice Sets the rate handler contract used for GOAT mint amounts
    /// @param rateHandlerAddress Address of the RateHandler contract
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

    /// @notice Update the goat's latest weight
    /// @param tokenId ID of the NFT to update
    /// @param newWeight New weight value in kilograms
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
        uint256 goatAmount = (currentWeight * 1e18) / rate;

        // Mint GOAT tokens directly to the token owner
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

    /// @notice Emitted when the GOAT token contract address changes
    event GoatTokenAddressUpdated(address indexed oldAddress, address indexed newAddress);
    event RateHandlerUpdated(address indexed oldAddress, address indexed newAddress);
    /// @notice Emitted when NFT is burned and GOAT token minted automatically
    event GoatBurned(uint256 indexed tokenId, address indexed user, uint256 weight, uint256 goatAmount);
    /// @notice Emitted when the goat weight is updated
    event WeightUpdated(uint256 indexed tokenId, uint256 newWeight);
}
