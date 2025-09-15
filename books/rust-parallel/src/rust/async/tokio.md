# tokio

* TODO

* [tokio](https://crates.io/crates/tokio)
* [tokio web site](https://tokio.rs/)

* See the http section for examples.

* TODO: show an example of a computationally heavy task that I run in async (severeal in parallel) and how they hand back execution to the eventloop.

```
cargo add tokio -F full
```

```rust
#[tokio::main]
async fn main() {
    ...
}
```

