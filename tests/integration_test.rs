use std::process::Command;

#[test]
fn test_cli_execution() {
    // This integration test attempts to run the binary inside the cargo environment
    // to ensure it doesn't panic and produces expected output lines.
    
    let output = Command::new("cargo")
        .args(&["run"])
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
