# Mutable empty vector with type definition

* vec!
* push

* We can also declare the type of the values in the vector. It might make the code more readable.
* And it might eliminate the need to explicitely tell a `parse` method the target value type.
* In this case Rust will see the `parse` method and because the result is pushed onto a vector of `i32` numbers it will know that `i32` is the type of the **number** variable.

{% embed include file="src/examples/vectors/vector-with-type/src/main.rs" %}
{% embed include file="src/examples/vectors/vector-with-type/out.out" %}


