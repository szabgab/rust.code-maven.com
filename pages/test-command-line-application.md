---
title: Test command line application written in Rust
timestamp: 2023-12-30T10:30:01
author: szabgab
published: true
description: Testing a CLI - Command Line Interface or command line application by running it as an external program.
tags:
    - CLI
    - testing
    - Command
    - ExitStatus
    - STDOUT
    - STDERR
    - panic!
---

For every application, including command line tools, the ideal way would be to move all the code into functions and even separate libraries,
but I rarely manage to reach that ideal. Especially when I get started I usually just want to get something done the simplest way I can.
On the other hand I like it when my code is tested. It makes me feel safe.

Later, as I fix bugs or as I want to add more features I might start to refactor the code to be structured better and thus also to make it easier
to write unit-tests for the individual functions, but even then I'd still want to have some tests that run the tool as a single unit.

That's why I need a way to test the command line tool as a stand-alone executable.

In this article we see a simple example.

## The Command Line tool

For this I created a crate called `test-cli` with the following code:

![](examples/test-cli/src/main.rs)

There is nothing fancy in this.

If the user supplies 2 parameters we try to convert them into `u32` integers using the `parse` method an the [Turbofish](/turbofish).
Then we multiply them. I did not even implement proper error handling, just called `unwrap` as this code is not the important part of the article.

The only precaution I did was checking if `args` has 3 values meaning the user supplied two values. If not, then an error message will be printed to
the Standard Error channel and th program will exit with exit code 2. This number is totally arbitrary. 0 means success, any other number means failure.

We can use this program by running

```
cargo run -q
```

to get the error message

or

```
cargo run -q 6 7
```

to get the result which is 42.


## Testing the Command Line tool

In order to test this command line application we created a folder called `tests` and in there a file called `tests.rs` with 3 test functions:

![](examples/test-cli/tests/tests.rs)

At first there is this rather complex `use`-statement. I think running `cargo fmt` mad it that way.

This would achieve the same:

```rust
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;
```

In the code we use [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) to run the external program.
Here we had a choice:

1. Run `cargo run -q` (where `-q` make it quiet)
2. Run `cargo build` once and then `./target/debug/test-cli` for every test.
3. Run `cargo build --release` once and then `./target/release/test-cli` for every test.

In most cases there should not be any difference in the results in the 3 cases. If this was an important real-world tool then I'd probably do the 3rd,
and for easier debugging I might also do either the 1st or the 2nd.

In this example I used the 1st option.

In every test function we run `cargo run -q` with some parameters. The result can be divided into 3 distinct parts.

* Whatever the program prints to STDOUT (Standard Output channel)
* Whatever the program prints to STDERR (Standard Error channel)
* The exit code  of the program. (On Linux macOS that would be the content of `$?`. On MS Windows that would be `ERROR_LEVEL`).

## Test empty call (missing parameters)

In the first test function called `test_empty_call` we run `cargo run -q`.

* We expect STDOUT to be empty.
* We expect STDERR to contain the usage-message we printed with `eprintln!`.
* The exit code is expected to be 2, but we actually get 256 times whatever the real exit code is. I wrote the multiplication explicitly as I think it makes the code more readable.

## Test with proper parameters

In the second test function called `test_multiply` we run `cargo run -q 6 7`.

* We expect STDOUT to contain the correct answer.
* We expect STDERR to be empty.
* The exit code to be 0 indicating success.

## Test with invalid parameters

In the third test function called `test_bad_input` we run `cargo run -q 3 4.2`.

This is incorrect as our implementation was only "designed" to handle integers. However we did not do any error checking so this is expected to `panic!`.

I think in a real-world application this case **should NOT** exist as no application should ever `panic!`. The code should be fixed to handle this properly
and then we should write a test to verify that the application provides the proper error message and exit code.

However as this is what we have here, let's see how do we test this?

* We expect STDOUT to be the empty string.
* The STDERR would be some long message. We could copy the error message we received the first time and set it as the exact expectation for further runs of the test, but it would be a bit fragile. Especially as it includes the row and column number of the location of the panic!. Moving the code around, even just adding an empty row at the top of the file would break the test. So instead of exact match we check if the string `InvalidDigit` appears in the error message.
* After running the code for the first time with this input I checked the exit code and that's how I set the expectation to be 25856.

## How do we run the tests?

```
$ cargo test -q
```

And the output is:

```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```


## Regression tests

Let me emphasize the interesting part of the 3rd test case as similar things happen often.

I did not have a clear idea of what exactly to expect. So I ran the code once and saved the result (or part of it) as the expectation for any further execution of the same test.

I don't know if this is the **correct** result, but I know that I'll notice when the result of this changes.

I use this technique quite often when for some reason I cannot verify the correctness of every result. At least I can verify that the results don't unexpectedly change as someone
changes the code or upgrades the compiler or one of the dependencies.

## Conclusion

This was a simple example. In more complex ones we the tool might need some other external input (e.g. a file, a folder, a database) and it might make changes to such external resources.
Preparing those and making sure they don't interfere with the real data might be a lot of extra work, but the core of the test-running and result verification can be seen in this example.


