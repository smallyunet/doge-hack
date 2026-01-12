# 🗺️ Doge-Hack Roadmap

**Goal**: Prove that Dogecoin is just Bitcoin in a yellow vest by constructing valid transactions using only standard Bitcoin tools.

## 🟢 Phase 1: The "Yellow Vest" Hack (Status: ✅ Complete - v0.1)
*Focus: Identity and Structure*

We start by mimicking the Dogecoin identity (Keys & Addresses) using Bitcoin cryptography.
- [x] **Keypair Generation**: Generate ECDSA keys using `secp256k1`.
- [x] **Address Encoding**: Custom Base58Check encoding for Dogecoin Testnet (`n`/`m` prefixes).
- [x] **Transaction Construction**: Manual P2PKH script generation and transaction assembly.
- [x] **Signing**: ECDSA signature generation and ScriptSig construction.
- [x] **CI/CD**: Basic GitHub Actions pipeline for testing and verification.

---

## 🟢 Phase 2: Interactivity & Network (Status: ✅ Complete - v0.2)
*Focus: CLI, Mainnet Support, and Network Interaction*

### 2.1 Command Line Interface (CLI) ✅
- **Goal**: Make the tool usable via arguments rather than hardcoded `main.rs`.
- **Implementation**:
    - [x] Adopted `clap` for argument parsing.
    - [x] Subcommands: `gen-key`, `address`, `sign`, `broadcast`, `fetch-utxo`, `demo`.
    - [x] Global `--network` flag for testnet/mainnet selection.

### 2.2 Dogecoin Mainnet Support ✅
- **Goal**: Support real Dogecoin addresses (`D` prefix).
- **Implementation**:
    - [x] `Network` enum with configurable version bytes.
    - [x] Testnet: `0x71` → `n`/`m` prefix.
    - [x] Mainnet: `0x1E` → `D` prefix.

### 2.3 RPC Integration ✅
- **Goal**: Fetch real UTXOs from a running Dogecoin Node (Dogecoind).
- **Implementation**:
    - [x] JSON-RPC client with configurable endpoint.
    - [x] `fetch_utxo(txid, vout)` command via `getrawtransaction`.
    - [x] `broadcast_tx(hex)` command via `sendrawtransaction`.

### 2.4 Library Modularization ✅
- **Goal**: Separate `src/lib.rs` from `src/main.rs`.
- **Implementation**:
    - [x] Created `lib.rs` exporting public modules.
    - [x] Modules: `address`, `transaction`, `network`, `rpc`.

---

## 🟡 Phase 3: Advanced Transaction Types (Next Steps)
Expand support beyond simple P2PKH transfers.

### 3.1 P2SH (Pay to Script Hash)
- **Goal**: Support multisig and custom scripts.
- **Tasks**:
    - [ ] Implement P2SH address encoding (different prefix).
    - [ ] Construct `OP_HASH160` redeem scripts.

### 3.2 Public API Support
- **Goal**: Support lightweight usage without a full node.
- **Tasks**:
    - [ ] Integrate with public Block Explorers (e.g., Chain.so, BlockCypher) API.
    - [ ] Fetch UTXOs by address (scan for funds).

---

## 🔴 Phase 4: Library Extraction
Refactor the hack into a reusable crate.

### 4.1 Publish to crates.io
- **Goal**: Publish `doge-hack-core` for other Rust projects.
- **Tasks**:
    - [ ] Add comprehensive documentation.
    - [ ] Provide clean API for Dogecoin interaction.

---

## 📅 Roadmap Timeline

| Milestone | Status | Focus |
| :--- | :--- | :--- |
| **v0.1** | ✅ Complete | Keypair, Address, Transaction, Signing |
| **v0.2** | ✅ Complete | CLI (`clap`) + Mainnet + RPC Integration |
| **v0.3** | 🟡 Next | P2SH Support + Public API |
| **v1.0** | 🔴 Planned | Stable Release + Comprehensive Docs |
