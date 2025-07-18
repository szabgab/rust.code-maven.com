# Stack and the capacity of a vector

* push
* pop
* len
* capacity

A little example showing how a vector growth its length (`len`) and `capacity` as we `push` more and more values on it.

Then how it reduces the length but keeps the allocated `capacity` as we `pop` out elements.

Finally how it can reduce the `capacity` when calling `shrink_to_fit`.

{% embed include file="src/examples/vectors/stack/src/main.rs" %}
{% embed include file="src/examples/vectors/stack/out.out" %}


