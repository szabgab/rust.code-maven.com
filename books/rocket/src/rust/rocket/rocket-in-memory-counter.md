# Rocket - In-memory counter using State

* A single counter in-memory counter (multiple browsers share the counter)

* [rocket](https://crates.io/crates/rocket)
* [rocket_dyn_templates](https://crates.io/crates/rocket_dyn_templates)
* [tera](https://crates.io/crates/tera)
* [serde](https://crates.io/crates/serde)

{% embed include file="src/examples/rocket/in-memory-counter/Cargo.toml" %}

{% embed include file="src/examples/rocket/in-memory-counter/src/main.rs" %}
{% embed include file="src/examples/rocket/in-memory-counter/src/tests.rs" %}

{% embed include file="src/examples/rocket/in-memory-counter/Rocket.toml" %}
{% embed include file="src/examples/rocket/in-memory-counter/templates/404.html.tera" %}
{% embed include file="src/examples/rocket/in-memory-counter/templates/incl/footer.html.tera" %}
{% embed include file="src/examples/rocket/in-memory-counter/templates/incl/header.html.tera" %}
{% embed include file="src/examples/rocket/in-memory-counter/templates/index.html.tera" %}


