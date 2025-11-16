# Wallet Architecture: Ports and Adapters

## 1. Introduction

This document describes the architecture of the multi-layer Bitcoin wallet. The architecture is based on the Ports and Adapters (or Hexagonal) pattern, which is designed to create a loosely coupled system that is easy to test, maintain, and extend.

## 2. Core Principles

*   **Separation of Concerns:** The core logic of the wallet is separated from the external services it interacts with.
*   **Dependency Inversion:** The core logic does not depend on the specific technologies used to implement the external services. Instead, it depends on abstract interfaces (the "ports").
*   **Modularity:** The wallet is composed of small, independent modules that can be developed and tested in isolation.

## 3. High-Level Architecture

The wallet is divided into three main parts:

*   **The Core:** This is the heart of the wallet. It contains the business logic for managing accounts, creating transactions, and interacting with the different blockchain layers. The core is completely independent of any specific technology.
*   **The Ports:** These are the interfaces that define how the core interacts with the outside world. There are two types of ports:
    *   **Driving Ports:** These are the interfaces that are used by the outside world to interact with the core. For example, the user interface will use a driving port to send a transaction.
    *   **Driven Ports:** These are the interfaces that are used by the core to interact with the outside world. For example, the core will use a driven port to get the latest price of Bitcoin from an oracle.
*   **The Adapters:** These are the implementations of the ports. They are the "glue" that connects the core to the specific technologies used in the wallet. There are two types of adapters:
    *   **Driving Adapters:** These are the adapters that drive the core. For example, the user interface is a driving adapter.
    *   **Driven Adapters:** These are the adapters that are driven by the core. For example, a Chainlink oracle adapter is a driven adapter.

## 4. Diagram

```
+-----------------------------------------------------------------+
|                                                                 |
|      +-----------------+      +-----------------+               |
|      | Driving Adapter |      | Driving Adapter |               |
|      | (e.g., UI)      |      | (e.g., CLI)     |               |
|      +-------+---------+      +--------+--------+               |
|              |                         |                        |
|              |       +-----------------+-------+                |
|              +-------> Driving Port (e.g., API) |                |
|                      |                         |                |
|                      |          CORE           |                |
|                      |                         |                |
|                      |  Driven Port (e.g., DB) <-------+         |
|                      +-----------------+-------+        |         |
|                                        |                |         |
|              +-------------------------+                |         |
|              |                                          |         |
|      +-------+---------+      +-----------------+       |         |
|      | Driven Adapter  |      | Driven Adapter  |       |         |
|      | (e.g., Oracle)  |      | (e.g., Node RPC)|       |         |
|      +-----------------+      +-----------------+       |         |
|                                                         |         |
+-----------------------------------------------------------------+
```

## 5. Next Steps

The next step is to define the specific ports and adapters that will be needed for the wallet. This will be done in a separate document.
