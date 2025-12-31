# 🗺️ Doge-Hack Roadmap

This document outlines the development trajectory for `doge-hack`, moving from a proof-of-concept to a robust tool for Dogecoin developers.

## 🟢 Phase 1: Core Fundamentals (Current Status: v0.1)
- [x] **Keypair Generation**: Generate ECDSA keys using `secp256k1`.
- [x] **Address Encoding**: Custom Base58Check encoding for Dogecoin Testnet (`n`/`m` prefixes).
- [x] **Transaction Construction**: Manual P2PKH script generation and transaction assembly.
- [x] **Signing**: ECDSA signature generation and ScriptSig construction.
- [x] **CI/CD**: Basic GitHub Actions pipeline for testing and verification.

---

## 🟡 Phase 2: Interactivity & Network (Next Steps)
The current tool works with mocked data. The next goal is to interact with the real chain.

### 2.1 RPC Integration
- **Goal**: Fetch real UTXOs from a running Dogecoin Node (Dogecoind).
- **Tasks**:
    - Implement a JSON-RPC client.
    - Add `fetch_utxo(txid, vout)` command.
    - Add `broadcast_tx(hex)` command to push transactions to the network.

### 2.2 Public API Support
- **Goal**: Support lightweight usage without a full node.
- **Tasks**:
    - Integrate with public Block Explorers (e.g., Chain.so, BlockCypher) API.
    - Fetch UTXOs by address (scan for funds).

### 2.3 Command Line Interface (CLI) Polish
- **Goal**: Make the tool usable via arguments rather than hardcoded `main.rs`.
- **Tasks**:
    - Adopt `clap` for argument parsing.
    - Example: `doge-hack gen-key`, `doge-hack sign --txid <ID> --sk <KEY>`.

---

## 🟠 Phase 3: Advanced Transaction Types
Expand support beyond simple P2PKH transfers.

### 3.1 P2SH (Pay to Script Hash)
- **Goal**: Support multisig and custom scripts.
- **Tasks**:
    - Implement P2SH address encoding (different prefix).
    - Construct `OP_HASH160` redeem scripts.

### 3.2 Dogecoin Mainnet Support
- **Goal**: Support real Dogecoin addresses (`D` prefix).
- **Tasks**:
    - Make `Network` configurable (Testnet vs Mainnet).
    - Update address generation logic to switch prefixes dynamically.

---

## 🔴 Phase 4: Library Extraction
Refactor the hack into a reusable crate.

### 4.1 Modularization
- **Goal**: Separate `src/lib.rs` from `src/main.rs`.
- **Tasks**:
    - Publish `doge-hack-core` or similar to crates.io (optional).
    - Provide a clean API for other Rust projects to interact with Dogecoin without forking `rust-bitcoin`.

---

## 📅 Roadmap Timeline

| Milestone | Estimated ETA | Focus |
| :--- | :--- | :--- |
| **v0.2** | Q1 2025 | CLI Args (`clap`) + Mainnet Support |
| **v0.3** | Q2 2025 | JSON-RPC Integration (Real network interaction) |
| **v1.0** | Q3 2025 | Stable Release + Comprehensive Docs |
