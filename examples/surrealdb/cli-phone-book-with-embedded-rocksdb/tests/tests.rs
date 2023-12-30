use std::{
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus},
};
use tempdir::TempDir;

#[test]
fn test_phonebook() {
    let tmp_dir = TempDir::new("phonebook").unwrap();
    println!("tmp_dir: {:?}", tmp_dir);

    std::env::set_var("DATABASE_PATH", tmp_dir.path());

    let result = Command::new("cargo")
        .args(["run", "-q", "add", "foo", "123"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    let result = Command::new("cargo")
        .args(["run", "-q", "add", "bar", "456"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    let result = Command::new("cargo")
        .args(["run", "-q", "show", "foo"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "foo: 123\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    let result = Command::new("cargo")
        .args(["run", "-q", "show", "bar"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "bar: 456\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    let result = Command::new("cargo")
        .args(["run", "-q", "list"])
        .output()
        .expect("command failed to start");

    //println!("{}", std::str::from_utf8(&result.stdout).unwrap());
    assert_eq!(
        std::str::from_utf8(&result.stdout).unwrap(),
        "bar: 456\nfoo: 123\n"
    );
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    let result = Command::new("cargo")
        .args(["run", "-q", "delete", "foo"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    let result = Command::new("cargo")
        .args(["run", "-q", "list"])
        .output()
        .expect("command failed to start");

    //println!("{}", std::str::from_utf8(&result.stdout).unwrap());
    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "bar: 456\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    drop(tmp_dir);
}
