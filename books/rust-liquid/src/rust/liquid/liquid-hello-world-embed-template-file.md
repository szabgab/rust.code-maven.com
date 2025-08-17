# Liquid Hello World embed template file

* parse
* include_str!

* If you would like to supply the temlates, probably the easiest is to embed them in the binary.
* Using `include_str!` we can embed a text-file in the compiled binary of our Rust code.
* In the source repository we have the templates as external files, but during `build` they are embedded in the code.

{% embed include file="src/examples/liquid/liquid-hello-world-embedded-file/src/main.rs" %}

{% embed include file="src/examples/liquid/liquid-hello-world-embedded-file/template.txt" %}


