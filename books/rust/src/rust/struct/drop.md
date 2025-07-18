# Drop - destructor

* Drop
* drop

* Implement the [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait
* By default when there is a panic! Rust will unwind the allocated memory and it will call the `drop` method on each object it encounters. We can set the `panic` compiler option to 'abort' in the Cargo.toml file to make Rust exit without unwinding. This will make shutting down the program faster, but in this case the `drop` methods will not be called.

{% embed include file="src/examples/struct/drop-demo/src/main.rs" %}
{% embed include file="src/examples/struct/drop-demo/Cargo.toml" %}


