use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_sqlx-sqlite-bank"))
}

fn unique_database_url() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    let db_path = std::env::temp_dir().join(format!(
        "sqlx-sqlite-bank-cli-test-{}-{}.db",
        std::process::id(),
        nanos
    ));

    format!("sqlite://{}", db_path.display())
}

#[test]
fn add_list_and_transfer_money() {
    let database_url = unique_database_url();

    let add_alice = Command::new(binary_path())
        .args(["add", "Alice", "100"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to add Alice");
    assert!(add_alice.status.success());
    assert_eq!(String::from_utf8_lossy(&add_alice.stdout), "");
    assert_eq!(String::from_utf8_lossy(&add_alice.stderr), "");

    let add_bob = Command::new(binary_path())
        .args(["add", "Bob", "40"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to add Bob");
    assert!(add_bob.status.success());
    assert_eq!(String::from_utf8_lossy(&add_bob.stdout), "");
    assert_eq!(String::from_utf8_lossy(&add_bob.stderr), "");

    let transfer = Command::new(binary_path())
        .args(["transfer", "25", "Alice", "Bob"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to transfer money");
    assert!(transfer.status.success());
    assert_eq!(String::from_utf8_lossy(&transfer.stdout), "");
    assert_eq!(String::from_utf8_lossy(&transfer.stderr), "");

    let list = Command::new(binary_path())
        .arg("list")
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to list accounts");
    assert!(list.status.success());
    assert_eq!(
        String::from_utf8_lossy(&list.stdout),
        "name | amount\n---- | ------\nAlice | 75\nBob | 65\n"
    );
    assert_eq!(String::from_utf8_lossy(&list.stderr), "");
}

#[test]
fn panic_during_transfer_rolls_back_transaction() {
    let database_url = unique_database_url();

    for args in [["add", "Alice", "100"], ["add", "Bob", "40"]] {
        let output = Command::new(binary_path())
            .args(args)
            .env("DATABASE_URL", &database_url)
            .output()
            .expect("failed to seed accounts");
        assert!(output.status.success());
    }

    let transfer = Command::new(binary_path())
        .args(["transfer", "25", "Alice", "Bob"])
        .env("DATABASE_URL", &database_url)
        .env("PANIC", "true")
        .output()
        .expect("failed to run crashing transfer");
    assert!(!transfer.status.success());
    assert_eq!(String::from_utf8_lossy(&transfer.stdout), "");
    assert!(
        String::from_utf8_lossy(&transfer.stderr)
            .contains("simulated crash in the middle of transfer")
    );

    let list = Command::new(binary_path())
        .arg("list")
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to list accounts after crash");
    assert!(list.status.success());
    assert_eq!(
        String::from_utf8_lossy(&list.stdout),
        "name | amount\n---- | ------\nAlice | 100\nBob | 40\n"
    );
}

#[test]
fn invalid_arguments_print_usage() {
    let database_url = unique_database_url();

    let invalid = Command::new(binary_path())
        .args(["hello", "world"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run invalid command");

    assert!(!invalid.status.success());
    assert_eq!(String::from_utf8_lossy(&invalid.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&invalid.stderr),
        "Usage: cargo run -- list | cargo run -- add NAME AMOUNT | cargo run -- transfer AMOUNT FROM_NAME TO_NAME\n"
    );
}

#[test]
fn transfer_requires_enough_money() {
    let database_url = unique_database_url();

    for args in [["add", "Alice", "10"], ["add", "Bob", "40"]] {
        let output = Command::new(binary_path())
            .args(args)
            .env("DATABASE_URL", &database_url)
            .output()
            .expect("failed to seed accounts");
        assert!(output.status.success());
    }

    let transfer = Command::new(binary_path())
        .args(["transfer", "25", "Alice", "Bob"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run transfer");

    assert!(!transfer.status.success());
    assert_eq!(String::from_utf8_lossy(&transfer.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&transfer.stderr),
        "Insufficient funds in Alice: has 10, needs 25\n"
    );
}
