# Rocket - Single hit-counter using a text file

{% embed include file="src/examples/rocket/single-counter-in-text-file/Cargo.toml" %}
{% embed include file="src/examples/rocket/single-counter-in-text-file/src/main.rs" %}
{% embed include file="src/examples/rocket/single-counter-in-text-file/src/tests.rs" %}

* Error handling - `unwrap`.
* File operations are not atomic.
* We don't handle variable overflow properly.


