# Macro with many parameters to say hello

* macro_rules!
* expr
* "*"
* "+"

* This macro can accept 0 or more parameters and then it will repeate the code as many times as parameters we got.
* Instead of `*` we could use `+` in the declaration and that would mean the macro accepts 1 or more parameters.

{% embed include file="src/examples/macros/say-hello-many-times/src/main.rs" %}
{% embed include file="src/examples/macros/say-hello-many-times/out.out" %}



