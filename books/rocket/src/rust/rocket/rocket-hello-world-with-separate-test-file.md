# Rocket - Hello World with separate test file

* Create a new crate, add rocket as a dependency `cargo add rocket`

{% embed include file="src/examples/rocket/hello-world-external-test-file/Cargo.toml" %}


{% embed include file="src/examples/rocket/hello-world-external-test-file/src/main.rs" %}

```
cargo run
```

The tests

{% embed include file="src/examples/rocket/hello-world-external-test-file/src/tests.rs" %}

```
cargo test
```

```
curl -i http://localhost:8000/
```
