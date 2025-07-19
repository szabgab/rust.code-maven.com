# map is lazy

In this example we can see that during the assignment to `double` nothing really happened. The iterator did not iterate.
Only after the first `println` when we iterated over the elements of `double`, only then we saw the output from the `print`-statements
inside the `map`.

{% embed include file="src/examples/vectors/map-is-lazy/src/main.rs" %}
{% embed include file="src/examples/vectors/map-is-lazy/out.out" %}


