# Doge-Hack

**Doge-Hack** is a technical experiment and CLI tool demonstrating how to construct valid **Dogecoin (Testnet)** transactions using the standard `rust-bitcoin` library, without forking it.

## 🏴‍☠️ The "Dogecoin is Bitcoin" Hack

The core philosophy of this project is simple: **Dogecoin is just Bitcoin in a yellow vest.**

Most Bitcoin Rust libraries can be used for Dogecoin by simply swapping a few parameters (Magic Bytes, Address Prefixes). This project demonstrates this "technical trivia" by using **pure `rust-bitcoin`**—the best Bitcoin library on the planet—to interact with Dogecoin.

*   **Goal:** Manually construct and sign a Dogecoin transaction using standard Bitcoin tooling.
*   **Why:** To profoundly understand the underlying data structures of the blockchain.

## 🚀 Key Features

*   **No Forks**: Uses pure `rust-bitcoin` crate.
*   **Custom Address Encodings**: Manually implements Base58Check encoding with Dogecoin Testnet prefixes (`0x71`).
*   **Manual Transaction Construction**: Builds legacy P2PKH transactions field-by-field.
*   **ECDSA Signing**: Uses `secp256k1` and `SighashCache` to sign inputs manually.

## 🦀 Why Rust?

Using Rust for this experiment compels you to handle the raw data structures explicitly.

-   **Deep Understanding**: You can't just call `wallet.send()`. You have to understand `Script`, `Witness`, `Sighash`, and `Derivation` paths.
-   **Type Safety vs. The Hack**: You'll fight the type system that tries to prevent you from using Bitcoin libraries for Dogecoin, forcing you to understand *exactly* where the two protocols differ (and where they are identical).
-   **Performance**: It's the native language of modern blockchain development.

## 🛠️ Installation

Ensure you have Rust installed.

```bash
git clone https://github.com/smallyu/doge-hack.git
cd doge-hack
cargo build
```

## 📖 Usage

Run the CLI tool to execute the full demonstration flow:

```bash
cargo run
```

### What happens?

1.  **Wallet Phase**: Generates a random ECDSA keypair and derives a Dogecoin Testnet address (starting with `n` or `m`).
2.  **Mock Data Phase**: Creates a pointer to a fake UTXO (Mock TxID + Vout 0).
3.  **Construction Phase**: Builds a transaction that spends the mock UTXO and sends 50 DOGE back to the generated address.
4.  **Signing Phase**: Signs the transaction input using the generated private key and outputs the final **signed transaction hex**.

## 🧠 How it works

Dogecoin is binary-compatible with Bitcoin for most primitive structures (Transactions, Scripts). The main differences are Address prefixes and Magic bytes.

This tool "hacks" around the `rust-bitcoin` library's limitation (which usually enforces Bitcoin `Network` types) by:
1.  Bypassing `Address` struct validation and encoding Base58 manually.
2.  Constructing `Transaction` structs directly instead of using high-level builders that might validate network consistency.
3.  Building `Script` opcodes manually for P2PKH ([OP_DUP, OP_HASH160, ..., OP_CHECKSIG]).

## 🧪 Testing

Run unit tests:
```bash
cargo test
```
