# Library API

The library is exported from `src/lib.rs` and split into small modules:

- `address`: Base58Check encoding/decoding and Dogecoin-specific version bytes.
- `network`: `Network` enum (`testnet` / `mainnet`) and network parameters.
- `transaction`: `TransactionBuilder` for legacy transaction construction and signing.
- `rpc`: `DogeRpcClient` JSON-RPC client for `dogecoind`.
- `explorer`: public API clients (Chain.so / SoChain v3).
- `script`: helpers for redeem scripts and P2SH scriptPubKey.

This crate is intentionally low-level: it avoids wallet state and keeps signing explicit.
