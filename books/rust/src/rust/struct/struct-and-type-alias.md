# Other: Struct and type alias - Polygon

* struct
* type

* The simplest way to represent a polygon (a series of points) is a vector of `Point` instances.
* We can even give it a name using the [type](https://doc.rust-lang.org/std/keyword.type.html) keyword.
* Despite its name it does **not** create a new type, just an alias.
* That's why we cannot use `impl` to add a method.

{% embed include file="src/examples/struct/polygon-type/src/main.rs" %}
{% embed include file="src/examples/struct/polygon-type/out.out" %}



