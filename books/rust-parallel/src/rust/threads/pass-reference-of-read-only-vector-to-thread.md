# Pass reference of read-only vector to thread

* [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) allows us to have reference counting.
* Here the `clone` only copies the reference and not the whole data structure.

{% embed include file="src/examples/threads/pass-reference-to-vector/src/main.rs" %}
{% embed include file="src/examples/threads/pass-reference-to-vector/out.out" %}

---

* Arc
* clone


