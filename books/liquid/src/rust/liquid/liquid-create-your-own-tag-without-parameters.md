# Liquid create your own tag without parameters

* expect_nothing

This is probably the simplest example of extending the Liquid syntax by new tags. I am not sure how usefule is this in the real world as I think the same could be done with the `include` tag, but this might help understanding how to create more complex tags.

* We need both [liquid](https://crates.io/crates/liquid) and [liquid-core](https://crates.io/crates/liquid-core)

{% embed include file="src/examples/liquid/single-tag/Cargo.toml" %}

* We need to add the struct implementing our tag (`single_tag::SingleTag`) to our parser using the `tag` method.
* Then we can use the `{% single %}` tag in our template.

{% embed include file="src/examples/liquid/single-tag/src/main.rs" %}


{% embed include file="src/examples/liquid/single-tag/src/single_tag.rs" %}



