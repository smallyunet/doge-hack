# Overview

Doge-Hack is a technical experiment showing that Dogecoin transactions can be constructed using standard Bitcoin primitives (via `bitcoin` / `rust-bitcoin`) by swapping a small set of network parameters.

Core idea:

- Transactions and scripts are largely binary-compatible between Bitcoin and Dogecoin.
- Address prefixes / version bytes differ by network.
- If you control serialization and script building yourself, you can build valid Dogecoin transactions without forking Bitcoin libraries.

This book documents the CLI, the library modules, and the transaction-building flow.
