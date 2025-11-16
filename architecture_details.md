# Wallet Architecture: Ports and Adapters - Details

This document provides a detailed breakdown of the ports and adapters for the multi-layer Bitcoin wallet.

## 1. Driving Ports (API)

These are the interfaces that the outside world (e.g., the UI) will use to interact with the wallet's core logic.

### 1.1. `WalletService`

*   `createWallet(passphrase: string): Wallet`
*   `loadWallet(passphrase: string): Wallet`
*   `lockWallet()`
*   `unlockWallet(passphrase: string)`
*   `getWalletStatus(): WalletStatus`

### 1.2. `AccountService`

*   `getAccounts(wallet: Wallet): Account[]`
*   `createAccount(wallet: Wallet, name: string): Account`
*   `getAccountBalance(account: Account, asset: Asset): Balance`

### 1.3. `TransactionService`

*   `createTransaction(sourceAccount: Account, destinationAddress: string, asset: Asset, amount: Amount): Transaction`
*   `signTransaction(transaction: Transaction, privateKey: PrivateKey): SignedTransaction`
*   `broadcastTransaction(signedTransaction: SignedTransaction): TransactionID`
*   `getTransactionHistory(account: Account): Transaction[]`

### 1.4. `StakingService`

*   `getStakingOptions(asset: Asset): StakingOption[]`
*   `stake(account: Account, option: StakingOption, amount: Amount): StakingPosition`
*   `unstake(position: StakingPosition): Transaction`
*   `getStakingPositions(account: Account): StakingPosition[]`

## 2. Driven Ports (SPI)

These are the interfaces that the core logic will use to interact with the outside world (e.g., blockchain nodes, oracles).

### 2.1. `BlockchainClient`

*   `getBalance(address: string, asset: Asset): Balance`
*   `getTransaction(transactionID: TransactionID): Transaction`
*   `broadcastTransaction(signedTransaction: SignedTransaction): TransactionID`
*   `getFeeEstimates(asset: Asset): FeeEstimates`

### 2.2. `OracleClient`

*   `getPrice(asset: Asset, currency: string): Price`

### 2.3. `Persistence`

*   `saveWallet(wallet: Wallet)`
*   `loadWallet(): Wallet`

## 3. Adapters

These are the implementations of the ports.

### 3.1. Driving Adapters

*   **GUI:** A graphical user interface for desktop and mobile platforms.
*   **CLI:** A command-line interface for advanced users and automation.
*   **API Server:** A server that exposes the driving ports as a REST or gRPC API.

### 3.2. Driven Adapters

*   **BitcoinNodeClient:** An implementation of `BlockchainClient` that interacts with a Bitcoin node.
*   **LightningNodeClient:** An implementation of `BlockchainClient` that interacts with a Lightning node.
*   **LiquidNodeClient:** An implementation of `BlockchainClient` that interacts with a Liquid node.
*   **RootstockNodeClient:** An implementation of `BlockchainClient` that interacts with a Rootstock node.
*   **StacksNodeClient:** An implementation of `BlockchainClient` that interacts with a Stacks node.
*   **PythOracleClient:** An implementation of `OracleClient` that gets price data from the Pyth network.
*   **DiaOracleClient:** An implementation of `OracleClient` that gets price data from the DIA oracle.
*   **FilePersistence:** An implementation of `Persistence` that saves the wallet to an encrypted file.
*   **DatabasePersistence:** An implementation of `Persistence` that saves the wallet to a database.

## 4. Next Steps

With the ports and adapters defined, the next step is to start implementing the core logic and the adapters. The core logic can be implemented and tested in isolation, and the adapters can be developed in parallel.
