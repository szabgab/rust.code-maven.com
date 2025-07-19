# Borrow String when passing to a function

* &}

* When passing the variable we need to prefix it with `&`.
* In the function definition we also include the `&` in-front of the type.
* Inside the function we can prefix it with `*` to dereference the variable but in general we don't need to as Rust figures that out.

{% embed include file="src/examples/ownership/string-function-borrow/src/main.rs" %}
{% embed include file="src/examples/ownership/string-function-borrow/out.out" %}


