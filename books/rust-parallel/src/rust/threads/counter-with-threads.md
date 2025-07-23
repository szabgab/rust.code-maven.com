# Counter with threads (shared variable) using Mutex

* Solution is using [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)

* This solution is actually slower than the single-threaded solution because the the threads are waiting for each other to free the guards.

{% embed include file="src/examples/threads/counter-with-mutex/src/main.rs" %}
{% embed include file="src/examples/threads/counter-with-mutex/out.out" %}

---

* Mutex
* lock


