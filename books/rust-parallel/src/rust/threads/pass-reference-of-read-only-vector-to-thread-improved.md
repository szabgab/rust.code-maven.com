# Pass reference of read-only vector to thread improved

* Arc
* clone

* In this solution the call to `clone` the Arc was moved inside the spawn.

{% embed include file="src/examples/threads/pass-vector/src/main.rs" %}
{% embed include file="src/examples/threads/pass-vector/out.out" %}


