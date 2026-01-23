use std::process::Command;
use doge_hack::address::DogeAddress;
use doge_hack::network::Network;

#[test]
fn test_cli_help() {
    // Test that the help command works
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("doge-hack"));
    assert!(stdout.contains("gen-key"));
    assert!(stdout.contains("address"));
    assert!(stdout.contains("sign"));
    assert!(stdout.contains("broadcast"));
    assert!(stdout.contains("p2sh-multisig"));
    assert!(stdout.contains("fetch-utxos-address"));
    assert!(stdout.contains("broadcast-public"));
}

#[test]
fn test_cli_gen_key_testnet() {
    // Test generating a keypair on testnet
    let output = Command::new("cargo")
        .args(&["run", "--", "--network", "testnet", "gen-key"])
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Doge-Hack: Generating New Keypair"));
    assert!(stdout.contains("Network: testnet"));
    assert!(stdout.contains("Secret Key (hex):"));
    assert!(stdout.contains("Address:"));
}

#[test]
fn test_cli_gen_key_mainnet() {
    // Test generating a keypair on mainnet
    let output = Command::new("cargo")
        .args(&["run", "--", "--network", "mainnet", "gen-key"])
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Network: mainnet"));
    // Mainnet addresses start with 'D'
    assert!(stdout.contains("Address: D"));
}

#[test]
fn test_cli_demo_mode() {
    // Test demo mode (original behavior)
    let output = Command::new("cargo")
        .args(&["run", "--", "demo"])
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Doge-Hack: Dogecoin Transaction Constructor Experiment"));
    assert!(stdout.contains("SUCCESS: Address starts with expected prefix."));
    assert!(stdout.contains("Transaction Constructed!"));
    assert!(stdout.contains("SIGNED Transaction Hex:"));
    assert!(stdout.contains("SUCCESS: Transaction constructed and signed manually!"));
}

#[test]
fn test_cli_address_derivation() {
    // Test address derivation from a known secret key
    let test_secret_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "address", "--secret-key", test_secret_key])
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Network: testnet"));
    assert!(stdout.contains("Address:"));
}

#[test]
fn test_cli_sign_with_manual_prevout() {
    // Build a deterministic address from a known secret key, then use it as both sender/recipient.
    let test_secret_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let secret_bytes = hex::decode(test_secret_key).unwrap();
    let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&secret_bytes).unwrap();
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let public_key = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    let from = DogeAddress::from_pubkey(&public_key, Network::Testnet);
    let to_addr = from.to_string();

    // Construct a matching P2PKH scriptPubKey for the from-address.
    let pubkey_hash = from.pubkey_hash();
    let prev_script_pubkey = bitcoin::script::Builder::new()
        .push_opcode(bitcoin::opcodes::all::OP_DUP)
        .push_opcode(bitcoin::opcodes::all::OP_HASH160)
        .push_slice(<&bitcoin::script::PushBytes>::try_from(pubkey_hash).unwrap())
        .push_opcode(bitcoin::opcodes::all::OP_EQUALVERIFY)
        .push_opcode(bitcoin::opcodes::all::OP_CHECKSIG)
        .into_script();
    let prev_script_hex = hex::encode(prev_script_pubkey.as_bytes());

    // Use a fake txid (only needs to be valid hex) and provide prevout params manually.
    let txid = "fb48f9e2068d0674c965e9057b6f87494df9278065a7f98ee591f7d3d7568553";
    let prev_value = 2 * 100_000_000u64; // 2 DOGE
    let amount = 1 * 100_000_000u64; // 1 DOGE
    let fee = 1_000_000u64; // 0.01 DOGE

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "sign",
            "--txid",
            txid,
            "--vout",
            "0",
            "--secret-key",
            test_secret_key,
            "--to",
            &to_addr,
            "--amount",
            &amount.to_string(),
            "--fee",
            &fee.to_string(),
            "--prev-value",
            &prev_value.to_string(),
            "--prev-script-hex",
            &prev_script_hex,
        ])
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SIGNED Transaction Hex:"));
}
