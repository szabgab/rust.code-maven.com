# Simple async example with Tokio

* We have function called `do_something` that fakes real work by sleeping a bit. It is marked as an [async](https://doc.rust-lang.org/std/keyword.async.html) function.
* In the `main` function we setup the [Runtime](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html).
* Calling `do_something()` does not actually execute it. It returns a [Future](https://doc.rust-lang.org/std/future/trait.Future.html).

{% embed include file="src/examples/tokio/simple/src/main.rs" %}


{% embed include file="src/examples/tokio/simple/Cargo.toml" %}

{% embed include file="src/examples/tokio/simple/out.out" %}


