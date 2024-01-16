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

    let result = Command::new("cargo")
        .args(["run", "-q", "foo"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "1\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));

    let result = Command::new("cargo")
        .args(["run", "-q", "foo"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "2\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));


    // let result = Command::new("cargo")
    //     .args(["run", "-q", "add", "foo", "789"])
    //     .output()
    //     .expect("command failed to start");

    // assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    // assert!(std::str::from_utf8(&result.stderr).unwrap().contains(
    //     "Could not add entry Database index `entry_email` already contains 'foo', with record"
    // ));
    // assert_eq!(result.status, ExitStatus::from_raw(2 * 256));

    // let result = Command::new("cargo")
    //     .args(["run", "-q", "show", "foo"])
    //     .output()
    //     .expect("command failed to start");

    // assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "foo: 123\n");
    // assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    // assert_eq!(result.status, ExitStatus::from_raw(0));

    // let result = Command::new("cargo")
    //     .args(["run", "-q", "show", "bar"])
    //     .output()
    //     .expect("command failed to start");

    // assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "bar: 456\n");
    // assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    // assert_eq!(result.status, ExitStatus::from_raw(0));

    // let result = Command::new("cargo")
    //     .args(["run", "-q", "list"])
    //     .output()
    //     .expect("command failed to start");

    // //println!("{}", std::str::from_utf8(&result.stdout).unwrap());
    // assert_eq!(
    //     std::str::from_utf8(&result.stdout).unwrap(),
    //     "bar: 456\nfoo: 123\n"
    // );
    // assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    // assert_eq!(result.status, ExitStatus::from_raw(0));

    // let result = Command::new("cargo")
    //     .args(["run", "-q", "delete", "foo"])
    //     .output()
    //     .expect("command failed to start");

    // assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    // assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    // assert_eq!(result.status, ExitStatus::from_raw(0));

    // let result = Command::new("cargo")
    //     .args(["run", "-q", "list"])
    //     .output()
    //     .expect("command failed to start");

    // //println!("{}", std::str::from_utf8(&result.stdout).unwrap());
    // assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "bar: 456\n");
    // assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    // assert_eq!(result.status, ExitStatus::from_raw(0));

    drop(tmp_dir);
}
