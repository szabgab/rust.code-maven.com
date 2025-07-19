# Vector with optional values - None or out of range?

* get
* is_none
* is_some
* TODO

* If we have a vector that some of the elements can be `None` then the other elements must be `Some`-values and the whole thing must be defined using `Option`.
* If we try to access an element in a vector that is out of range we get a run-time panic.
* In order to avoid such panic we either need to check if our index is in range or we can use the `get` method.
* We can use the `get` method to access the element. It will return `None` if the index was out of range.
* Then the question arise, how do we know if the value was out of range or if it was in the range but the value was `None`?

{% embed include file="src/examples/vectors/none-or-out-of-range/src/main.rs" %}
{% embed include file="src/examples/vectors/none-or-out-of-range/out.out" %}


