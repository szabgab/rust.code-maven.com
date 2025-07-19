# Procedural macros

* proc-macro
* TokenStream

* [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)
* Apparently the Macro needs to be in its own Crate separate from where it is being used.

* Add `proc-macro` to the `Cargo.toml`

* Here we show that the macro is executed during the compilation.
* It needs to return a TokenStream.

{% embed include file="src/examples/macros/hello-world-macro/Cargo.toml" %}
{% embed include file="src/examples/macros/hello-world-macro/src/lib.rs" %}

{% embed include file="src/examples/macros/hello-world-use/Cargo.toml" %}
{% embed include file="src/examples/macros/hello-world-use/src/main.rs" %}


