# Mutable string in immutable variable

* push_str

* If we initialize the variable using `String::from` then the literal value is copied to the heap and it can be changed.
* But if the variable is not mutable, then what's the point?

{% embed include file="src/examples/ownership/mutable-string-in-immutable-variable/src/main.rs" %}
{% embed include file="src/examples/ownership/mutable-string-in-immutable-variable/out.out" %}
{% embed include file="src/examples/ownership/mutable-string-in-immutable-variable/err.out" %}


