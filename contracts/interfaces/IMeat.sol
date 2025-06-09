// SPDX-License-Identifier: MIT
pragma solidity ^0.8.29;

interface IMeat {
    function mintSubtype(address to, bytes32 subtype, uint256 amount) external;
}
