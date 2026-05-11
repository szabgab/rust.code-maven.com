use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_sqlx-sqlite-family-tree"))
}

fn unique_database_url() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    let db_path = std::env::temp_dir().join(format!(
        "sqlx-sqlite-family-tree-cli-test-{}-{}.db",
        std::process::id(),
        nanos
    ));

    format!("sqlite://{}", db_path.display())
}

#[test]
fn add_and_list_people() {
    let database_url = unique_database_url();

    let add_child = Command::new(binary_path())
        .args(["add", "Alice"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for add child");

    assert!(add_child.status.success());
    assert_eq!(String::from_utf8_lossy(&add_child.stdout), "");
    assert_eq!(String::from_utf8_lossy(&add_child.stderr), "");

    let add_father = Command::new(binary_path())
        .args(["add", "Bob"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for add father");

    assert!(add_father.status.success());
    assert_eq!(String::from_utf8_lossy(&add_father.stdout), "");
    assert_eq!(String::from_utf8_lossy(&add_father.stderr), "");

    let add_mother = Command::new(binary_path())
        .args(["add", "Carol"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for add mother");

    assert!(add_mother.status.success());
    assert_eq!(String::from_utf8_lossy(&add_mother.stdout), "");
    assert_eq!(String::from_utf8_lossy(&add_mother.stderr), "");

    let father_of = Command::new(binary_path())
        .args(["father-of", "Alice", "Bob"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for father-of");

    assert!(father_of.status.success());
    assert_eq!(String::from_utf8_lossy(&father_of.stdout), "");
    assert_eq!(String::from_utf8_lossy(&father_of.stderr), "");

    let mother_of = Command::new(binary_path())
        .args(["mother-of", "Alice", "Carol"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for mother-of");

    assert!(mother_of.status.success());
    assert_eq!(String::from_utf8_lossy(&mother_of.stdout), "");
    assert_eq!(String::from_utf8_lossy(&mother_of.stderr), "");

    let list = Command::new(binary_path())
        .arg("list")
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for list");

    assert!(list.status.success());
    assert_eq!(
        String::from_utf8_lossy(&list.stdout),
        "name | father | mother\n---- | ------ | ------\nAlice | Bob | Carol\nBob | - | -\nCarol | - | -\n"
    );
    assert_eq!(String::from_utf8_lossy(&list.stderr), "");
}

#[test]
fn father_of_requires_existing_people() {
    let database_url = unique_database_url();

    let add = Command::new(binary_path())
        .args(["add", "Alice"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to add Alice");
    assert!(add.status.success());

    let set_father = Command::new(binary_path())
        .args(["father-of", "Alice", "Bob"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run father-of");

    assert!(!set_father.status.success());
    assert_eq!(String::from_utf8_lossy(&set_father.stdout), "");
    assert_eq!(String::from_utf8_lossy(&set_father.stderr), "Unknown father: Bob\n");
}

#[test]
fn invalid_arguments_print_usage_on_stderr() {
    let database_url = unique_database_url();

    let invalid = Command::new(binary_path())
        .args(["hello", "world"])
        .env("DATABASE_URL", &database_url)
        .output()
        .expect("failed to run binary for invalid arguments");

    assert!(!invalid.status.success());
    assert_eq!(String::from_utf8_lossy(&invalid.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&invalid.stderr),
        "Usage: cargo run [list] | cargo run add NAME | cargo run father-of CHILD FATHER | cargo run mother-of CHILD MOTHER\n"
    );
}
