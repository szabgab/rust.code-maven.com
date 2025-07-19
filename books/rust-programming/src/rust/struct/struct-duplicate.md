# Other: Struct duplicate

* This seems to be an old example showing that if we don't compose one struct from another then we have to implement everything in both cases.
* In this case the `Circle` struct has its own x and y attribute and its own `mv` method.

{% embed include file="src/examples/struct/circle-duplicate/src/main.rs" %}
{% embed include file="src/examples/struct/circle-duplicate/out.out" %}


