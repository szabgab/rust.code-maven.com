# Testing with tempfiles and environment variables


How to pass the path of a temporary file to an application during testing?

There are many applications that need access to files on the filesystem or to a database. In order to have clear separation between the test runs I create a temporary folder (or a temporary database)
and then I need to pass this to the test somehow. While programming in Perl or Python I used to do this by setting an environment variable in the test and then reading it in the application itself.

Sometimes this environment variable would contain a path to a configuration file that is then used by the application.

This patters is somewhat broken when trying use it in Rust. The reason is that Rust by default runs the tests in parallel in threads and environment variables are per process.
Thus if I set two different temporary paths in two test-files one will override the other. Then depending on the timing one test might look at the temporary file of the other test.

Here is an example. I tried to simplify it, but it still feels a bit convoluted.

## The dependencies

{% embed include file="examples/test-tempfile/Cargo.toml" %}


## The code with the tests

{% embed include file="examples/test-tempfile/src/main.rs" %}

## Running the tests

Running the tests in the usual way will randomly break as the environment variable get mixed between the two test-cases.

```
cargo test
```

The solution, for now, is to run the tests in a single thread:

```
cargo test -- --test-threads=1
```

Unfortunately people new to the project will likely try to run the default command and the test will fail for them.

We could mention this in the README file, but can we really rely on people reading the documentation?

My [question on the users forum](https://users.rust-lang.org/t/how-to-configure-tests-in-cargo-toml-test-threads-1/105549) got a suggestion to
include the setting in the `cargo/config.toml` file:

{% embed include file="examples/test-tempfile/.cargo/config.toml" %}

Actually I include two suggestions.


## Alias

The first one was to create an alias for the command. This way I can run

```
cargo t
```

and it will run the tests with the `--test-threads=1`. This is convenient, but people will still run `cargo test` and complain about the failures.


## Set the `RUST_TEST_THREADS` env variable

The second advice was to set the `RUST_TEST_THREADS` environment variable in the `.cargo/config.toml` file.

That worked. Now if I run

```
cargo test
```

It will run in a single thread.


## Future

I am not entirely happy with this whole solution as it means I needed some special code in my application and I can't run the tests in a thread.
I [asked](https://users.rust-lang.org/t/how-to-pass-parameters-during-tests/105551) about this and I'll update this post when I have a better solution.

- tags:
    - testing
    - tempfile
    - threads
    - RUST_TEST_THREADS

