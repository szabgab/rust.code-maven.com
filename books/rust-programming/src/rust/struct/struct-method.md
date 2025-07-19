# Implement a method for a struct

* impl
* self
* &self

* We can add methods to a `struct` using the `impl` keyword (we implement the method).
* The first parameter of every method must be the variable name `self` that will represent the current instance.
* We can have either `self` or `&self`. The latter will borrow the instance and it will allow us to call two methods (or the same method twice as in our example).
* If we used only `self` the instance would be moved on the first method call. Rarely used.

{% embed include file="src/examples/struct/point-with-method/src/main.rs" %}
{% embed include file="src/examples/struct/point-with-method/out.out" %}



