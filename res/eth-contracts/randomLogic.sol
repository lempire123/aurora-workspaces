// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract RandomLogic {
    uint256 public constant tickets = 100;
    uint256 public winner;

    function randomSeed() public returns (uint256) {
        bytes32[1] memory value;

        assembly {
            let ret := call(gas(), 0xc104f4840573bed437190daf5d2898c2bdf928ac, 0, 0, 0, value, 32)
        }

        return uint256(value[0]);
    }

    function computeWinner() public {
        uint256 randNum = randomSeed(); 
        winner = (randNum % tickets);
    }

    function getWinner() public view returns (uint256) {
        return winner;
    }
}
