// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Vulnerable {
  uint256 public constant REWARD_RATE = 500;
  uint256 public constant BASIS_POINTS = 10000;

  function calculateReward(uint256 amount, uint256 daysActive) public pure returns (uint256) {
    uint256 rewardShare = amount / BASIS_POINTS;

    return rewardShare * REWARD_RATE * daysActive;
  }
}
