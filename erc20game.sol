pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract GameContract {
    address public owner;

    IERC20 public gameToken;

    struct Player {
        uint256 playerId;
        address playerAddress;
        uint256 score;
        uint256 balance; 
        bool isActive;
    }

    Player[] public players;

    uint256 public playerCount;

    mapping(address => bool) public playerExists;

    constructor(address _gameToken) {
        owner = msg.sender;
        gameToken = IERC20(_gameToken);
    }

    function addPlayer() public {
        require(!playerExists[msg.sender], "Player already exists.");
        uint256 newPlayerId = playerCount;
        players.push(Player(newPlayerId, msg.sender, 0, 0, true));
        playerExists[msg.sender] = true;
        playerCount++;
    }

    function increaseScore(uint256 playerId, uint256 points) public {
        require(msg.sender == players[playerId].playerAddress, "Only the player can increase their score.");
        require(players[playerId].isActive, "Player is not active.");
        players[playerId].score += points;
    }

    function sendTokens(uint256 playerId, uint256 amount) public {
        require(msg.sender == owner, "Only the contract owner can send tokens.");
        require(gameToken.transfer(players[playerId].playerAddress, amount), "Token transfer failed.");
        players[playerId].balance += amount;
    }

    function deactivatePlayer(uint256 playerId) public {
        require(msg.sender == owner, "Only the contract owner can deactivate players.");
        players[playerId].isActive = false;
    }
}
