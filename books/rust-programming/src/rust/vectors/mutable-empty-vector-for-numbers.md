# Mutable empty vector for numbers (push)

* push
* mut
* vec!

* We can also create a mutable empty vector without even declaring the types of the values.
* When we `push` the first value that's how Rust will know what is the type of the elements of the vector.
* Trying to push values of other types would then generate a compilation error.

{% embed include file="src/examples/vectors/mutable-empty-vector-for-integers/src/main.rs" %}
{% embed include file="src/examples/vectors/mutable-empty-vector-for-integers/out.out" %}


