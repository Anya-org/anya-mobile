# Wormhole Integration Research

This document outlines the research on integrating Wormhole into the multi-layer Bitcoin wallet.

## Wormhole Architecture

Wormhole is a generic messaging protocol that connects to multiple blockchains. It allows for the transfer of tokens and data between these chains. It achieves this by deploying a "Core Bridge" smart contract on each supported chain, which is governed by a proof-of-authority consensus mechanism with 19 validators.

## Bitcoin Integration

Wormhole does not have a direct, "native" integration with the Bitcoin L1. Instead, it interacts with Bitcoin through **wrapped tokens**. The most common example is Wrapped Bitcoin (WBTC) on Ethereum and other EVM-compatible chains.

## Wallet Integration Requirements

To provide "native" support for Wormhole, the wallet must:

1.  **Support a Wormhole-compatible chain:** The wallet needs to support at least one blockchain that is fully integrated with Wormhole and also has a mechanism for representing Bitcoin (e.g., Ethereum with WBTC, or Rootstock with RBTC).
2.  **Facilitate BTC wrapping/unwrapping:** The wallet needs to provide a user-friendly way to convert native BTC into its wrapped equivalent on the supported chain, and vice-versa. This will likely involve integrating with a third-party bridge or service.
3.  **Implement Wormhole transfer logic:** The wallet needs to integrate the Wormhole SDK or API to allow users to:
    *   Select a destination chain from the list of Wormhole-supported chains.
    *   Initiate a transfer of the wrapped Bitcoin asset.
    .
4.  **Display Wormhole assets:** The wallet must be able to display balances of wrapped Bitcoin on different chains.

## Conclusion

"Native" Wormhole support in the context of a Bitcoin wallet means abstracting away the wrapping and bridging process for the user, making it feel like a seamless cross-chain transfer of Bitcoin. The wallet's architecture needs to be able to manage assets on multiple chains and interact with both the Bitcoin network and the Wormhole protocol.
