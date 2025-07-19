# Exponent - power

* pow

* We can use the [pow](https://doc.rust-lang.org/std/primitive.i32.html#method.pow) method to get the exponent of a number, but Rust needs to know the exact type of that number.
* It can be set explicitly or implicitly as in the case of the function returning an `i16` number.

* We have to be careful as `pow` can overflow.

{% embed include file="src/examples/numbers/exponent/src/main.rs" %}


