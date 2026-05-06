use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_sqlx-sqlite-counter"))
}

fn unique_database_url() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    let db_path = std::env::temp_dir().join(format!(
        "sqlx-sqlite-counter-cli-test-{}-{}.db",
        std::process::id(),
        nanos
    ));

    format!("sqlite://{}", db_path.display())
}

#[test]
fn command_writes_expected_stdout_and_stderr() {
    let database_url = unique_database_url();

    let increment = Command::new(binary_path())
        .arg("apple")
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for increment");

    assert!(increment.status.success());
    assert_eq!(String::from_utf8_lossy(&increment.stdout), "apple 1\n");
    assert_eq!(String::from_utf8_lossy(&increment.stderr), "");

    let list = Command::new(binary_path())
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for list");

    assert!(list.status.success());
    assert_eq!(String::from_utf8_lossy(&list.stdout), "apple 1\n");
    assert_eq!(String::from_utf8_lossy(&list.stderr), "");

    let invalid = Command::new(binary_path())
        .args(["hello", "world"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for invalid args");

    assert!(!invalid.status.success());
    assert_eq!(String::from_utf8_lossy(&invalid.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&invalid.stderr),
        "Usage: cargo run [NAME]\n"
    );
}
