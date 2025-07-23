# Threads with messages

* We can facilitate communication between the main thread and the spawned thread.
* In this example the spawned thread is sending a message to the main thread.
* The `move` keyword tells Rust that the variables declared before spawning that are also used in the spawned code need to be moved. (`tx` in this case)
* We can use `recv`, which is blocking the main thread, to wait for a message from the spawned thread.
* In that case we will have to know how many messages to expect and if we are still waiting for a message while the spawned thread exits then we either get stuck or get panic!.
* Using the second loop is a better solution.


{% embed include file="src/examples/threads/threads-messages/src/main.rs" %}
{% embed include file="src/examples/threads/threads-messages/out.out" %}

---

* channel
* send
* recv
* move


