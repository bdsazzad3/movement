// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {IAtomicBridgeInitiator} from "./IAtomicBridgeInitiator.sol";
import {IWETH9} from "./IWETH9.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

contract AtomicBridgeInitiator is IAtomicBridgeInitiator, OwnableUpgradeable {
    enum MessageState {
        INITIALIZED,
        COMPLETED,
        REFUNDED
    }

    struct BridgeTransfer {
        uint256 amount;
        address originator;
        bytes32 recipient;
        bytes32 hashLock;
        uint256 timeLock; // in blocks
        MessageState state;
    }

    // Mapping of bridge transfer ids to BridgeTransfer structs
    mapping(bytes32 => BridgeTransfer) public bridgeTransfers;
    // Mapping of user addresses to WETH balances
    mapping(address => uint256) public balances;

    address public counterpartyContract; 
    IWETH9 public weth;
    uint256 private nonce;

    function initialize(address _weth, address owner) public initializer {
        if (_weth == address(0)) {
            revert ZeroAddress();
        }
        weth = IWETH9(_weth);
        __Ownable_init(owner);
    }

    function setCounterpartyContract(address _counterpartyContract) external onlyOwner {
        require(_counterpartyContract != address(0), "Invalid counterparty address");
        counterpartyContract = _counterpartyContract;
    }

    function initiateBridgeTransfer(uint256 wethAmount, bytes32 recipient, bytes32 hashLock, uint256 timeLock)
        external
        payable
        returns (bytes32 bridgeTransferId)
    {
        address originator = msg.sender;
        uint256 ethAmount = msg.value;
        uint256 totalAmount = wethAmount + ethAmount;
        // Ensure there is a valid total amount
        if (totalAmount == 0) {
            revert ZeroAmount();
        }
        // If msg.value is greater than 0, convert ETH to WETH
        if (ethAmount > 0) weth.deposit{value: ethAmount}();
        // Transfer WETH to this contract, revert if transfer fails
        if (wethAmount > 0) {
            if (!weth.transferFrom(originator, address(this), wethAmount)) revert WETHTransferFailed();
        }

        // Update the balance of the originator 
        balances[originator] += totalAmount;

        nonce++; // increment the nonce
        bridgeTransferId =
            keccak256(abi.encodePacked(originator, recipient, hashLock, timeLock, block.number, nonce));

        bridgeTransfers[bridgeTransferId] = BridgeTransfer({
            amount: totalAmount,
            originator: originator,
            recipient: recipient,
            hashLock: hashLock,
            timeLock: block.number + timeLock,
            state: MessageState.INITIALIZED
        });

        emit BridgeTransferInitiated(bridgeTransferId, originator, recipient, totalAmount, hashLock, timeLock);
        return bridgeTransferId;
    }

    function completeBridgeTransfer(bytes32 bridgeTransferId, bytes32 preImage) external {
        BridgeTransfer storage bridgeTransfer = bridgeTransfers[bridgeTransferId];
        if (bridgeTransfer.state != MessageState.INITIALIZED) revert BridgeTransferHasBeenCompleted();
        if (keccak256(abi.encodePacked(preImage)) != bridgeTransfer.hashLock) revert InvalidSecret();
        if (block.number > bridgeTransfer.timeLock) revert TimelockExpired();
        bridgeTransfer.state = MessageState.COMPLETED;

        emit BridgeTransferCompleted(bridgeTransferId, preImage);
    }

    function refundBridgeTransfer(bytes32 bridgeTransferId) external {
        BridgeTransfer storage bridgeTransfer = bridgeTransfers[bridgeTransferId];
        if (bridgeTransfer.state != MessageState.INITIALIZED) revert BridgeTransferStateNotInitialized();
        if (block.number < bridgeTransfer.timeLock) revert TimeLockNotExpired();
        bridgeTransfer.state = MessageState.REFUNDED;
        // Decrease balance of originator and transfer WETH back to originator
        balances[bridgeTransfer.originator] -= bridgeTransfer.amount;
        if (!weth.transfer(bridgeTransfer.originator, bridgeTransfer.amount)) revert WETHTransferFailed();

        emit BridgeTransferRefunded(bridgeTransferId);
    }

    // Counterparty contract to withdraw WETH for originator
    function withdrawWETH(address originator, uint256 amount) external {
        if (msg.sender != counterpartyContract) revert Unauthorized();
        if (balances[originator] < amount) revert ZeroAmount();
        balances[originator] -= amount;
        if (!weth.transfer(originator, amount)) revert WETHTransferFailed();
    }
}

