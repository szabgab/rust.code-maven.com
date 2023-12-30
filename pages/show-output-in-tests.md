---
title: Show standard output and standard error in tests in Rust
timestamp: 2023-12-30T18:30:01
published: true
description: Sometimes we would like to include print statements in the tests in Rust. How can we see them?
tags:
    - tests
    - STDOUT
    - STDERR
    - --show-output
todo:
    - test capturing and validating the output in a function in an inline test
---

Sometimes our code prints to the screen, either to the standard output channel or the standard error channel.
There are cases when we might want to verify the content of what was printed in other case we just would like to see.

By default when we run `cargo test`, it will capture and hide both channels. Using the `--show-output` we can convince
`cargo` to print the captured output.

In this example we'll see this working both inline tests that are usually used for unit-testing and external tests that, in this case,
run our code as an external program as we can also see in the article on [how to test command line application](/test-command-line-application).

## Code with inline tests

![](examples/show-output-in-tests/src/main.rs)

## External tests

Running the program as a command line application we would probably check that the text printed to the STDOUT and STDERR **by the program**
is what we expect. Still we have some text printed **by the test** that we would like to see.

![](examples/show-output-in-tests/tests/tests.rs)

## Regular test output

```
$ cargo test -q

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

The upper part shows the inline test


## Running the tests and showing the output

```
$ cargo test -q -- --show-output

running 1 test
.
successes:

---- test_function stdout ----
STDOUT In test_function
STDERR In test_function
STDOUT in code
STDERR in code


successes:
    test_function

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
successes:

---- test_cli stdout ----
STDOUT In test_cli
STDERR In test_cli


successes:
    test_cli

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Note

The `-q` is an option of `cargo`, but --show-output` is a parameter of the test runner. So we need the extra `--` in order to pass this argument to the actual test runner.




