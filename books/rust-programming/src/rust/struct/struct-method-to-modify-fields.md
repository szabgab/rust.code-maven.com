# Struct method to modify fields

* impl
* mut
* &mut

* We can add methods to a `struct` using the `impl` keyword (we implement the method) that will modify the struct.
* For this we need to write `&mut self` in the method and the struct instance must be also mutable.

{% embed include file="src/examples/struct/point-with-mutable-method/src/main.rs" %}
{% embed include file="src/examples/struct/point-with-mutable-method/out.out" %}


