# Pass and return ownership

* An alternate way to handle this situation is to return the vector.
* This way we pass the ownership back to the caller.

* This would only work properly if the threads do not need the same variable at the same time.
* So either they run sequentially in which case we don't gain CPU or each thread needs a different variable.

{% embed include file="src/examples/threads/pass-and-return-reference/src/main.rs" %}
{% embed include file="src/examples/threads/pass-and-return-reference/out.out" %}


