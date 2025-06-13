// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

import {IGoatNFT} from "../interfaces/IGoatNFT.sol";

/// @title Malicious hook that attempts reentrancy during burn
contract MaliciousGoatHook {
    IGoatNFT public goatNFT;
    uint256 public targetTokenId;

    constructor(address nftAddress) {
        goatNFT = IGoatNFT(nftAddress);
    }

    function setTargetTokenId(uint256 tokenId) external {
        targetTokenId = tokenId;
    }

    function onBurn(address /*to*/, uint256 /*weight*/) external {
        // attempt reentrant burn
        if (targetTokenId != 0) {
            try goatNFT.burn(targetTokenId) {
                // no-op
            } catch {}
        }
    }
}
