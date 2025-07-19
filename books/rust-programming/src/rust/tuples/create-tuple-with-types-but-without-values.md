# Create tuple with types, but without values

* mut

* We can create a tuple without initializing. In this case it seems even more useful to declare the types. (Though not required.)
* We must have the `mut` keyword to make it mutable.
* Then later we can assign all the values at once.
* Before we initialize all the values we cannot assign them one-by-one.

{% embed include file="src/examples/tuples/create-tuple-without-values/src/main.rs" %}
{% embed include file="src/examples/tuples/create-tuple-without-values/out.out" %}


