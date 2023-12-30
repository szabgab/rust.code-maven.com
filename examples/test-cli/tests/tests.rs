use std::{
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus},
};

#[test]
fn test_empty_call() {
    let result = Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    assert_eq!(
        std::str::from_utf8(&result.stderr).unwrap(),
        "Usage: target/debug/test-cli WIDTH HEIGHT\n"
    );
    assert_eq!(result.status, ExitStatus::from_raw(256 * 2));
}

#[test]
fn test_multiply() {
    let result = Command::new("cargo")
        .args(["run", "-q", "6", "7"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "42\n");
    assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert_eq!(result.status, ExitStatus::from_raw(0));
}

#[test]
fn test_bad_input() {
    let result = Command::new("cargo")
        .args(["run", "-q", "3", "4.2"])
        .output()
        .expect("command failed to start");

    assert_eq!(std::str::from_utf8(&result.stdout).unwrap(), "");
    // "thread 'main' panicked at src/main.rs:8:36:\ncalled `Result::unwrap()` on an `Err` value:
    // ParseIntError { kind: InvalidDigit }\n
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"
    //assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), "");
    assert!(std::str::from_utf8(&result.stderr)
        .unwrap()
        .contains("InvalidDigit"));
    assert_eq!(result.status, ExitStatus::from_raw(256 * 101));
}
