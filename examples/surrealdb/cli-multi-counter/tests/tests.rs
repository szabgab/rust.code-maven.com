use std::{
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus},
};
use tempdir::TempDir;

#[test]
fn test_counter() {
    let tmp_dir = TempDir::new("counter").unwrap();
    println!("tmp_dir: {:?}", tmp_dir);

    std::env::set_var("DATABASE_PATH", tmp_dir.path());

    check("foo", "1\n");
    check("foo", "2\n");
    check("foo", "3\n");
    check("bar", "1\n");
    check("bar", "2\n");
    check("foo", "4\n");

    let result = Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("command failed to start");

    assert_eq!(
        std::str::from_utf8(&result.stdout).unwrap(),
        "Listing counters\n----------------\nfoo: 4\nbar: 2\n"
    );
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    drop(tmp_dir);
}

fn check(name: &str, expected_stdout: &str) {

    let result = Command::new("cargo")
        .args(["run", "-q", name])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), expected_stdout);
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));
}

