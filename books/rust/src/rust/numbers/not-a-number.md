# NaN - Not a Number

* NaN
* sqrt

* Floating point numbers, [f32](https://doc.rust-lang.org/std/primitive.f32.html) or f64, can also represent a value called NaN or Not a Number.
* One way to get the number is to take the square root of -1.0.
* Most operations with a NaN result in NaN.
* Two NaN values are not equal.

* The `sqrt` (square root) method is not implemented for integers.

{% embed include file="src/examples/numbers/not-a-number/src/main.rs" %}
{% embed include file="src/examples/numbers/not-a-number/out.out" %}


