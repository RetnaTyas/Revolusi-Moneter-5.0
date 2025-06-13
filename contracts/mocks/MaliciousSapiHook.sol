// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {ISapiNFT} from "../interfaces/ISapiNFT.sol";

/// @title Malicious hook that attempts reentrancy during burn of SapiNFT
contract MaliciousSapiHook {
    ISapiNFT public sapiNFT;
    uint256 public targetTokenId;

    constructor(address nftAddress) {
        sapiNFT = ISapiNFT(nftAddress);
    }

    function setTargetTokenId(uint256 tokenId) external {
        targetTokenId = tokenId;
    }

    function onBurn(address /*to*/, uint256 /*weight*/) external {
        if (targetTokenId != 0) {
            try sapiNFT.burn(targetTokenId) {
            } catch {}
        }
    }
}
