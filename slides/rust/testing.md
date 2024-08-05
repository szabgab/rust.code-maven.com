# Testing
{id: testing}

## Testing a library crate
{id: testing-a-library-crate}
{i: lib}
{i: test}

```
$ cargo new --lib test-lib
cd test-lib
cargo test
```

![](examples/testing/test-lib/src/lib.rs)

## Test coverage report with tarpaulin
{id: test-coverage-with-tarpaulin}
{i: coverage}
{i: tarpaulin}

* [Tarpaulin](https://github.com/xd009642/tarpaulin)

![](examples/testing/test-coverage/src/lib.rs)

```
cargo install cargo-tarpaulin
```

```
cargo tarpaulin
```

Exclude the test functions from the report:

```
cargo tarpaulin --ignore-tests
```

* Generate HTML report

```
cargo tarpaulin --ignore-tests -o Html
```


## Test a function in a crate
{id: test-a-function}
{i: test}
{i: assert_eq}

![](examples/testing/test-function/Cargo.toml)
![](examples/testing/test-function/src/main.rs)

## Show STDOUT and STDERR during testing
{id: show-stdout-and-stderr-during-testing}
{i: nocapture}

In this example there are print-statements both in the code and in the test function.

![](examples/testing/nocapture/src/lib.rs)

If we run `cargo test` we don't see any of this as the tester captures them.

If we run `cargo test -- --nocapture` then we'll see the output of all the 4 print-statements.


## Testing crates
{id: testing-crates}

TODO

![](examples/testing/test-crate/Cargo.toml)
![](examples/testing/test-crate/src/main.rs)

