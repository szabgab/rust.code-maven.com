# Accessing envrionment variables

* std::env
* std::env::var

We can access the environment variables from Rust using the [std::env::var](https://doc.rust-lang.org/std/env/fn.var.html) function.
It returns a `Result`.

{% embed include file="src/examples/other/get-environment-variable/src/main.rs" %}

Run as

```
cargo run
RUST=42 cargo run
```


