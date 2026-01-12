//! Doge-Hack Library
//!
//! A Rust library for constructing Dogecoin transactions using Bitcoin primitives.
//! Proves that Dogecoin is essentially "Bitcoin in a yellow vest" by demonstrating
//! binary-compatible transaction construction.
//!
//! # Modules
//!
//! - `address` - Dogecoin address generation (P2PKH)
//! - `transaction` - Transaction construction and signing
//! - `network` - Network configuration (Testnet/Mainnet)
//! - `rpc` - JSON-RPC client for node communication

pub mod address;
pub mod transaction;
pub mod network;
pub mod rpc;

pub use address::DogeAddress;
pub use transaction::TransactionBuilder;
pub use network::Network;
pub use rpc::DogeRpcClient;
