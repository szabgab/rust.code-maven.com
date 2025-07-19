# Liquid Hello World

* parse
* build
* object!
* render

* Depened on the liquid crate

{% embed include file="src/examples/liquid/liquid-hello-world/Cargo.toml" %}

* Start with a template that is part of the Rust source code.
* We use the `parse` and `build` methods to create the template object.
* We use `unwrap` here which is probably not ideal, but it simlifies the examples.
* Using the `liquid::object!` macro we create an object from the data we would like to pass to the template.
* Using the `render` method we combine the data with the template and generate (render) the resuling text.


{% embed include file="src/examples/liquid/liquid-hello-world/src/main.rs" %}
{% embed include file="src/examples/liquid/liquid-hello-world/out.out" %}


