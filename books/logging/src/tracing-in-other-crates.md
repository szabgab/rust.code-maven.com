# Tracing in other crates

* First enable tracing and observe the crate names it uses for tracing.
* Then enable the configuration in which we can set the level of tracing for each crate.
* Instead of using the name of our crate hard-coded we can get it from Cargo.toml using `env!("CARGO_CRATE_NAME")`.

{% embed include file="src/examples/tracing/trace-in-others/Cargo.toml" %}

{% embed include file="src/examples/tracing/trace-in-others/src/main.rs" %}


