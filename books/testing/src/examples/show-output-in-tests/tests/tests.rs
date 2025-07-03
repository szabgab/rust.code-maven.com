use std::{
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus},
};

#[test]
fn test_cli() {
    println!("STDOUT In test_cli");
    eprintln!("STDERR In test_cli");

    let result = Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("command failed to start");

    assert_eq!(
        std::str::from_utf8(&result.stdout).unwrap(),
        "STDOUT in code\n"
    );
    assert_eq!(
        std::str::from_utf8(&result.stderr).unwrap(),
        "STDERR in code\n"
    );
    assert_eq!(result.status, ExitStatus::from_raw(0));
}
