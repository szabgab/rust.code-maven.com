# Capture and verify STDOUT and STDERR in tests

In this example the function that we are testing also prints to the STDOUT and STDERR.
We would like to extend our tests to verify that the output is as expected on both channels.

{% embed include file="src/examples/test-output/src/lib.rs" %}

We'll use the [gag](https://crates.io/crates/gag) crate for this:

{% embed include file="src/examples/test-output/Cargo.toml" %}

Unfortunatelly for that crate to be able to the magic while in the tests we also need to pass the `--nocapture`
flag to the tests. We can do it manually `cargo run test --nocapture` or we can setup an alias in the `.cargo/config.toml` file:

{% embed include file="src/examples/test-output/.cargo/config.toml" %}


Then we can run `cargo t`.

The actual test looks like this:

{% embed include file="src/examples/test-output/src/tests.rs" %}

It has a lot of code, but eventually it collects the STDOUT and STDERR and we can compare
them to some expected value.



