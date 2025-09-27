use assert_cmd::Command;

fn run_cmd(args: &[&str]) -> String {
    let output = Command::cargo_bin("wallet-cli")
        .unwrap()
        .args(args)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "Command {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).unwrap()
}

fn extract_wallet_ids(stdout: &str) -> Vec<i32> {
    stdout
        .lines()
        .filter_map(|line| {
            if let Some(id_str) = line.strip_prefix("ID: ") {
                id_str.split(',').next()?.trim().parse().ok()
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn integration_test_both_key_types() {
    let key_types = ["ecdsa", "eddsa"];

    let mut wallet_ids = Vec::new();

    // 1. Create 2 wallets per key type
    for &key_type in &key_types {
        for _ in 0..2 {
            run_cmd(&["new-wallet", "--key-type", key_type]);
        }
        let stdout = run_cmd(&["list-wallets"]);
        wallet_ids.extend(extract_wallet_ids(&stdout));
    }

    // 2. Sign 2 messages per wallet
    let messages = ["msg1", "msg2"];
    for &wallet_id in &wallet_ids {
        for &msg in &messages {
            run_cmd(&[
                "sign",
                "--wallet-id",
                &wallet_id.to_string(),
                "--message",
                msg,
            ]);
        }
    }

    // 3. Verify all signatures exist
    for &wallet_id in &wallet_ids {
        let stdout = run_cmd(&["list-signatures", "--wallet-id", &wallet_id.to_string()]);
        for &msg in &messages {
            assert!(stdout.contains(msg));
        }
    }

    // 4. Clear all signatures
    run_cmd(&["clear-signatures"]);

    for &wallet_id in &wallet_ids {
        let stdout = run_cmd(&["list-signatures", "--wallet-id", &wallet_id.to_string()]);
        for &msg in &messages {
            assert!(!stdout.contains(msg));
        }
    }

    // 5. Clear wallets
    run_cmd(&["clear-wallets"]);
    let stdout = run_cmd(&["list-wallets"]);
    assert!(!stdout.contains("ID:"));
}
