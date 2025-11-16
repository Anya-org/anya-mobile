# Multi-Layer Bitcoin Wallet

This project is a multi-layer Bitcoin wallet that aims to provide a comprehensive and user-friendly platform for interacting with the entire Bitcoin ecosystem, from the base layer to the various Layer 2 and sidechain solutions.

## Architecture

The wallet is designed with a "Ports and Adapters" (or Hexagonal) architecture. This architecture separates the core logic of the wallet from the external services it interacts with, making the system loosely coupled, easy to test, maintain, and extend.

## Current Status

The project is in the early stages of development. The core logic and basic infrastructure are currently being implemented.

## Getting Started

To get started with the project, you will need to have Node.js and npm installed.

1.  Clone the repository:
    ```
    git clone https://github.com/your-username/multi-layer-bitcoin-wallet.git
    ```
2.  Install the dependencies:
    ```
    npm install
    ```
3.  Run the tests:
    ```
    npx jest
    ```
