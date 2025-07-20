# Rust testing setup and teardown fixtures

We can create setup/teardown fixtures using a struct and the Drop trait that will be executed even if the test panics.

{% embed include file="src/examples/testing/fixtures/src/lib.rs" %}

{% embed include file="src/examples/testing/fixtures/out.out" %}

