# Fixed vector of numbers


* vec!
* len

* A [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) vector is a series of values of the same type.
* We can initialize a vector using the [vec!](https://doc.rust-lang.org/std/macro.vec.html) macro.
* We can get the length of the vector using the [len](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len) method.
* We cannot print a vector with the simle `{}` placeholder because [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) is not implemented for it.
* However we can use the `{:?}` or the `{:#?}` placeholders.
* By default vectors are immutable.

{% embed include file="src/examples/vectors/numbers/src/main.rs" %}
{% embed include file="src/examples/vectors/numbers/out.out" %}


