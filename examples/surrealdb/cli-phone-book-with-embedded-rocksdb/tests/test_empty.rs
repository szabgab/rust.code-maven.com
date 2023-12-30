use std::{
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus},
};
use tempdir::TempDir;

#[test]
fn test_phonebook_no_params() {
    let tmp_dir = TempDir::new("phonebook").unwrap();
    println!("tmp_dir: {:?}", tmp_dir);

    std::env::set_var("DATABASE_PATH", tmp_dir.path());
    let result = Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    assert_eq!(
        std::str::from_utf8(&result.stderr).unwrap(),
        "Usage: target/debug/phonebook add name value\n"
    );
    assert_eq!(result.status, ExitStatus::from_raw(1 * 256));
}
