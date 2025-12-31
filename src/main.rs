mod address;
mod transaction;

use std::str::FromStr;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::{thread_rng, Rng};
use crate::address::DogeAddress;

fn main() {
    println!("Doge-Hack: Dogecoin Transaction Constructor Experiment");

    // Phase 1: Wallet
    println!("\n--- Phase 1: Wallet ---");
    
use rand::Rng; // Add this import at top if needed, or use rand::thread_rng directly is fine but need trait for gen

// ... inside main ...
    // Generate Keypair
    let secp = Secp256k1::new();
    
    let mut secret_bytes = [0u8; 32];
    rand::thread_rng().fill(&mut secret_bytes);
    
    let secret_key = SecretKey::from_slice(&secret_bytes).expect("Valid secret key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    
    println!("Secret Key: {}", secret_key.display_secret());
    println!("Public Key: {}", public_key);

    // Generate Doge Address
    let address = DogeAddress::from_pubkey(&public_key);
    let address_str = address.to_string();
    println!("Doge Testnet Address: {}", address_str);

    // Verify prefix
    if address_str.starts_with('n') || address_str.starts_with('m') {
         println!("SUCCESS: Address starts with expected prefix.");
    } else {
         println!("WARNING: Address prefix mismatch!");
    }

    // Phase 2: Mock Data
    println!("\n--- Phase 2: Mock Data ---");
    // Hardcoded UTXO (Random valid-looking hash)
    let mock_txid = "fb48f9e2068d0674c965e9057b6f87494df9278065a7f98ee591f7d3d7568553"; // Example
    let mock_vout = 0;
    println!("Mock UTXO: {} : {}", mock_txid, mock_vout);

    // Phase 3: Construction
    println!("\n--- Phase 3: Construction ---");
    
    use crate::transaction::TransactionBuilder;
    
    let mut builder = TransactionBuilder::new();
    builder.add_input(mock_txid, mock_vout);
    
    // Send 50 DOGE (50 * 100_000_000 satoshis) back to self
    let amount = 50 * 100_000_000;
    builder.add_output(&address, amount);
    
    let tx = builder.clone().build();
    println!("Transaction Constructed!");
    println!("Tx Version: {:?}", tx.version);
    println!("Tx LockTime: {:?}", tx.lock_time);
    println!("Tx Inputs: {}", tx.input.len());
    println!("Tx Outputs: {}", tx.output.len());
    
    // Serialize to Hex (Unsigned)
    let tx_hex = bitcoin::consensus::encode::serialize_hex(&tx);
    println!("Unsigned Transaction Hex:\n{}", tx_hex);

    // Phase 4: Signing
    println!("\n--- Phase 4: Signing ---");
    
    // We need the "ScriptCode" (previous output's script_pubkey) to sign
    // Let's assume the UTXO we are spending belongs to US (the keypair we generated)
    // So we recreate the P2PKH script for our address
    let pubkey_hash = address.pubkey_hash();
    let prev_script_pubkey = bitcoin::script::Builder::new()
        .push_opcode(bitcoin::opcodes::all::OP_DUP)
        .push_opcode(bitcoin::opcodes::all::OP_HASH160)
        .push_slice(<&bitcoin::script::PushBytes>::try_from(pubkey_hash).unwrap())
        .push_opcode(bitcoin::opcodes::all::OP_EQUALVERIFY)
        .push_opcode(bitcoin::opcodes::all::OP_CHECKSIG)
        .into_script();
        
    println!("Signing Input 0...");
    builder.sign_input(0, &secret_key, &prev_script_pubkey);
    
    let signed_tx = builder.build();
    let signed_tx_hex = bitcoin::consensus::encode::serialize_hex(&signed_tx);
    
    println!("SIGNED Transaction Hex:\n{}", signed_tx_hex);
    println!("\nSUCCESS: Transaction constructed and signed manually!");
}
