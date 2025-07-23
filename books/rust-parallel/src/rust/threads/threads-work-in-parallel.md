# Show that threads work in parallel

* `spawn` will create a new thread. We can use `thread::current().id()` to get the id of the current thread.
* `join` in the main thread will block till the other thread stops.
* We can see "Loop in main thread ended" is already printed before the "Spawned thread ended", but then the main thread waits.

{% embed include file="src/examples/threads/try-threads/src/main.rs" %}
{% embed include file="src/examples/threads/try-threads/out.out" %}


---

* thread
* spawn
* sleep
* join
* current


