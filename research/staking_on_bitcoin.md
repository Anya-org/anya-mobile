# Staking on Bitcoin Research

This document summarizes the research on staking-like functionalities available in the Bitcoin ecosystem.

## Is Native Bitcoin Staking Possible?

No. Bitcoin uses a Proof-of-Work (PoW) consensus mechanism, which does not support staking in the traditional sense. Staking is a feature of Proof-of-Stake (PoS) blockchains.

## How to "Stake" Bitcoin

"Staking" in the context of Bitcoin is made possible through various methods that are built on top of or alongside the main Bitcoin blockchain. These methods provide staking-like rewards, but they are not native to Bitcoin's L1.

### 1. Stacks "Stacking"

*   **Description:** Stacks is a Layer 2 blockchain that brings smart contracts and dApps to Bitcoin. It has a unique consensus mechanism called Proof-of-Transfer (PoX).
*   **Mechanism:**
    *   Users lock up their Stacks (STX) tokens.
    *   In return, they earn rewards in Bitcoin (BTC).
    *   This is different from traditional staking, where users earn rewards in the same token they are staking.
    *   The BTC rewards come from the miners on the Stacks network, who bid in BTC to mine new blocks.
*   **Wallet Requirements:**
    *   Support for STX tokens.
    *   Ability to initiate and manage the "Stacking" process through a smart contract on the Stacks network.
    *   Ability to display both STX and BTC balances.

### 2. Wrapped Bitcoin (WBTC) on PoS Chains

*   **Description:** Wrapped Bitcoin (WBTC) is an ERC-20 token on the Ethereum blockchain that is backed 1:1 by Bitcoin. It allows Bitcoin to be used in the Ethereum ecosystem.
*   **Mechanism:**
    *   Users can wrap their BTC into WBTC through a custodian.
    *   Once they have WBTC, they can use it in various DeFi protocols on Ethereum to earn a yield. This can include lending, liquidity providing, or staking in protocols that support WBTC.
    *   Similar wrapped Bitcoin solutions exist on other PoS chains.
*   **Wallet Requirements:**
    *   Support for the PoS chain (e.g., Ethereum).
    *   Support for WBTC or other wrapped Bitcoin assets.
    *   Ability to interact with DeFi protocols on that chain.
    *   Ideally, a user-friendly interface to wrap and unwrap BTC.

### 3. Other Bitcoin Sidechains and L2s

*   Other projects may offer similar staking-like features. The general principle is that they are separate blockchains that are pegged to Bitcoin in some way and have their own consensus mechanisms that may include staking.
*   **Wallet Requirements:** The wallet would need to integrate with each specific sidechain or L2 to support its staking functionality.

## Conclusion

"Native staking support" in the context of this wallet means providing a seamless user experience for these different staking-like mechanisms. The wallet needs to be able to manage assets on multiple chains and interact with various smart contracts and protocols. The architecture must be modular to accommodate these different integrations.
