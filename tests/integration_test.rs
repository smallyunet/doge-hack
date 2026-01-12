use std::process::Command;

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
