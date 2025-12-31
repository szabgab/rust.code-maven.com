use super::*;
use gag::BufferRedirect;
use std::io::Read;

#[test]
fn it_works() {
    let mut stdout_buf = BufferRedirect::stdout().unwrap();
    let mut stderr_buf = BufferRedirect::stderr().unwrap();

    let result = add(2, 3);
    assert_eq!(result, 5);

    let mut stdout_output = String::new();
    stdout_buf.read_to_string(&mut stdout_output).unwrap();
    drop(stdout_buf);

    let mut stderr_output = String::new();
    stderr_buf.read_to_string(&mut stderr_output).unwrap();
    drop(stderr_buf);

    assert_eq!(stdout_output, "Left: 2\n");
    assert_eq!(stderr_output, "Right: 3\n");
}
