# Square root (sqrt)

* sqrt
* as
* f64

* Calling the `sqrt` method needs to know the underlying type.
* It suggests `i32`, but in fact integers don't have `sqrt` implemented, only floats.
* We can convert (cast) an integer to a floating point using the [as](https://doc.rust-lang.org/std/keyword.as.html) keyword.

{% embed include file="src/examples/numbers/sqrt/src/main.rs" %}


