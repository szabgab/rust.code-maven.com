# Rust functions return the unit by default


* By default function return nothing, more precisely they return `()`  the empty tuple which is called the `unit`.
* We need the `#[allow(clippy::let_unit_value, clippy::unit_cmp)]` in the example to make Clippy, the Rust linter accept this code.

{% embed include file="src/examples/functions/return-nothing/src/main.rs" %}
{% embed include file="src/examples/functions/return-nothing/out.out" %}




