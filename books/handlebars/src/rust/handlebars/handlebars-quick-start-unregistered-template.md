# Quick start - unregistered template

* Handlebars
* render_template
* json

* [Handlebars](https://crates.io/crates/handlebars) is templating language implemented in Rust. It is based on the ideas of the original [HandlebarsJS](https://handlebarsjs.com/) but it is not 100% compatible.

* We have a very simple template embedded in our Rust code. It has one placeholder `{{name}}`.
* We pass a JSON string to the `render_template` function.

{% embed include file="src/examples/handlebars/handlebars-quick/src/main.rs" %}

* The generated output looks like this:

{% embed include file="src/examples/handlebars/handlebars-quick/out.out" %}

{% embed include file="src/examples/handlebars/handlebars-quick/Cargo.toml" %}


