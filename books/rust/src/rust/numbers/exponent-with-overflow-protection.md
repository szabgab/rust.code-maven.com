# Exponent protecting  against overflow - checked_pow, saturating_pow

* checked_pow
* saturating_pow

* As many other mathematical operations, calling `pow` can also create a number that does not fit in the expected type and then the code would `panic!`.
* We can use the [checked_pow](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_pow) that returns an [Option](https://doc.rust-lang.org/std/option/enum.Option.html)
* It contains the computed value, if successful or `None` if there was an overflow.
* An alternative way is to use [saturating_pow](https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_pow).

{% embed include file="src/examples/numbers/checked-pow/src/main.rs" %}



