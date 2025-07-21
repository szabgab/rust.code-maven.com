# Shared read-only variable with string value with Arc

* We can use [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) to create Reference Counting around the data.
* The `clone` on the Arc will only increment the reference counter, but does not copy the data.

{% embed include file="src/examples/threads/shared-read-only-variable-string-with-arc/src/main.rs" %}
{% embed include file="src/examples/threads/shared-read-only-variable-string-with-arc/out.out" %}


