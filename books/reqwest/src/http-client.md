# http-client async with reqwest

In order to send asynchronous request we need to depend on both the [reqwest](https://crates.io/crates/reqwest) crate and on [tokio](https://crates.io/crates/tokio) that provides the asynchronous runtime.
For the latter we add the `full` feature so we won't need to worry about anything missing.

```
cargo add reqwest
cargo add tokio -F full
```

## Cargo.toml

{% embed include file="src/examples/reqwest/http-client/Cargo.toml" %}


{% embed include file="src/examples/reqwest/http-client/src/main.rs" %}


---
* reqwest
* async
* tokio


