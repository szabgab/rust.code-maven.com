# http-client async with reqwest

In order to send asynchronous requests we need to depend on both the [reqwest](https://crates.io/crates/reqwest) crate and on [tokio](https://crates.io/crates/tokio) that provides the asynchronous runtime.
For the latter we add the `full` feature so we won't need to worry about anything missing.

```
cargo add reqwest
cargo add tokio -F full
```

## Cargo.toml

{% embed include file="src/examples/reqwest/http-client/Cargo.toml" %}

## The Code

In this example we are not particularily interested in any specific errors, so we handle them using the `?` operator that, in this case, will make our program exit with a non-zero exit code.

The `text` method will return the content of the web page. As we are accessing the main page of [httpbin.org](https://httpbin.org/) that should be some HTML document.

{% embed include file="src/examples/reqwest/http-client/src/main.rs" %}


---
* reqwest
* async
* tokio


