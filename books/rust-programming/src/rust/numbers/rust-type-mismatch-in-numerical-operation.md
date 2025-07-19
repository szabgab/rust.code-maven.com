# Rust type mismatch in numerical operation

* i32
* i64

{% embed include file="src/examples/numbers/type-mismatch/src/main.rs" %}

* If we remove the `i32` then this works even though the default is `i32`.
* That's because Rust will infere the type of the first variable from the type of the second variable and the operation.


