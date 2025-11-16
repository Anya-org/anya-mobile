# Oracle Research

This document summarizes the research on Chainlink CCIP, Pyth, and DIA oracles, and their potential integration into the multi-layer Bitcoin wallet.

## Chainlink CCIP (Cross-Chain Interoperability Protocol)

*   **Functionality:** CCIP is a protocol for transferring tokens and arbitrary data between different blockchains. It is not a traditional data oracle, but it is secured by Chainlink's decentralized oracle networks.
*   **Wallet Integration:**
    *   CCIP can be used to enable seamless cross-chain transfers of assets, such as wrapped Bitcoin, between different chains supported by the wallet.
    *   It can also be used to send messages between chains to trigger actions in smart contracts, which could be useful for more complex cross-chain interactions.
*   **Key Takeaway:** CCIP is a powerful tool for cross-chain communication and will be a key component in building the multi-layer functionality of the wallet.

## Pyth Network

*   **Functionality:** Pyth is a first-party oracle network that provides real-time market data directly from financial institutions, such as exchanges and market makers.
*   **Wallet Integration:**
    *   Pyth can be used to display real-time price feeds for all the assets supported by the wallet.
    *   It can provide price data for any integrated DeFi protocols.
    *   It can power any other features that require accurate, real-time market data.
*   **Key Takeaway:** Pyth is a high-quality source of real-time price data, which is essential for a good user experience in a crypto wallet.

## DIA (Decentralised Information Asset)

*   **Functionality:** DIA is an open-source, community-governed oracle platform that provides transparent and verifiable data feeds. It sources data directly from a variety of on-chain and off-chain sources.
*   **Wallet Integration:**
    *   Similar to Pyth, DIA can be used to provide price feeds for the assets in the wallet.
    *   Its focus on transparency and verifiability could be a selling point for users who are concerned about the reliability of their data.
    *   The community-governed aspect could also be a good fit for a decentralized wallet.
*   **Key Takeaway:** DIA is another strong option for providing price data to the wallet, with a particular focus on transparency and community governance.

## Conclusion

The wallet will need to integrate with both a cross-chain communication protocol and a data oracle.

*   **For cross-chain communication, Chainlink CCIP is a strong candidate.** It is a well-established and secure protocol that is widely used in the industry.
*   **For price data, both Pyth and DIA are good options.** The choice between them may come down to specific technical requirements, such as the availability of certain price feeds, the cost of using the oracle, and the desired level of decentralization. It may also be possible to use both oracles and allow the user to choose their preferred data source.

The wallet's architecture should be designed to be agnostic to the specific oracle provider, allowing for flexibility and future integrations.
