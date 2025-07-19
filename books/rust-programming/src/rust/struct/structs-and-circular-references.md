# Structs and circural references

* Rust will make sure we cannot create circular reference in this way.
* `#[allow(unused_mut)]` is needed to silence clippy, the linter

{% embed include file="src/examples/struct/circural-references/src/main.rs" %}
{% embed include file="src/examples/struct/circural-references/out.out" %}

* Try to enable the commented out code and see the error message.

{% embed include file="src/examples/struct/circural-references/err.out" %}


