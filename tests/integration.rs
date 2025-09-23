use assert_cmd::Command;

#[test]
fn integration_test() {
    // 1. Create wallet 1
    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("new-wallet")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Created wallet"));

    // 2. Create wallet 2
    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("new-wallet")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Created wallet"));

    // 3. List wallets and extract IDs
    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("list-wallets")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    let wallet_ids: Vec<i32> = stdout
        .lines()
        .filter_map(|line| {
            if let Some(id_str) = line.strip_prefix("ID: ") {
                id_str.split(',').next()?.trim().parse().ok()
            } else {
                None
            }
        })
        .collect();
        
    assert!(wallet_ids.len() >= 2, "Expected at least 2 wallets");

    let w1 = wallet_ids[0];
    let w2 = wallet_ids[1];

    // 4. Sign 2 messages per wallet
    let messages = ["msg1", "msg2"];
    for &msg in &messages {
        let output = Command::cargo_bin("wallet_cli")
            .unwrap()
            .args(&["sign", "--wallet-id", &w1.to_string(), "--message", msg])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Signature:"));

        let output = Command::cargo_bin("wallet_cli")
            .unwrap()
            .args(&["sign", "--wallet-id", &w2.to_string(), "--message", msg])
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Signature:"));
    }

    // 5. Create wallet 3 and sign 2 messages
    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("new-wallet")
        .output()
        .unwrap();
    assert!(output.status.success());

    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("list-wallets")
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let w3: i32 = stdout
        .lines()
        .filter_map(|line| {
            if let Some(id_str) = line.strip_prefix("ID: ") {
                id_str.split(',').next()?.trim().parse().ok()
            } else {
                None
            }
        })
        .max()
        .unwrap(); // newest wallet

    for &msg in &messages {
        let output = Command::cargo_bin("wallet_cli")
            .unwrap()
            .args(&["sign", "--wallet-id", &w3.to_string(), "--message", msg])
            .output()
            .unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Signature:"));
    }

    // 6. Verify all signatures exist
    for &w in &[w1, w2, w3] {
        let output = Command::cargo_bin("wallet_cli")
            .unwrap()
            .args(&["list-signatures", "--wallet-id", &w.to_string()])
            .output()
            .unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("msg1"));
        assert!(stdout.contains("msg2"));
    }

    // 7. Clear all signatures
    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("clear-signatures")
        .output()
        .unwrap();
    assert!(output.status.success());

    // 8. Verify signatures are cleared out.
    for &w in &[w1, w2, w3] {
        let output = Command::cargo_bin("wallet_cli")
            .unwrap()
            .args(&["list-signatures", "--wallet-id", &w.to_string()])
            .output()
            .unwrap();

        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(!stdout.contains("msg1"));
        assert!(!stdout.contains("msg2"));
    }

    // 9. Clear wallets
    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("clear-wallets")
        .output()
        .unwrap();
    assert!(output.status.success());

    // 10. Verify wallets are cleared out.
    let output = Command::cargo_bin("wallet_cli")
        .unwrap()
        .arg("list-wallets")
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("ID:"));
}
