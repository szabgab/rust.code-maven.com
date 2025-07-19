# Command line arguments - print all args


* std::env
* args
* Vec

* We load the [std::env](https://doc.rust-lang.org/std/env/) module
* `#[allow(clippy::needless_range_loop)]` is needed to silence clippy, the linter

{% embed include file="src/examples/args/args/src/main.rs" %}

```
cargo run

My path is target/debug/args
Number of arguments is 1
```

```
cargo run apple banana

My path is target/debug/args
Number of arguments is 3
Parameter 1 is 'apple'
Parameter 2 is 'banana'
```


