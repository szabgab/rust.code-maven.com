# Simple thread with join

* thread
* spawn
* join

* The solution is to save the `handle` of the thread and the use `join` to wait for its termination.

{% embed include file="src/examples/threads/simple-with-join/src/main.rs" %}
{% embed include file="src/examples/threads/simple-with-join/out.out" %}


