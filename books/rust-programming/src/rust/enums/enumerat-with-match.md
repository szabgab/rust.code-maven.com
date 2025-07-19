# Enumerate without PartialEq using match

* enum
* match
* dead_code

* If we don't have PartialEq on an enum and we don't want to add it or cannot add it (e.g. because it was supplied by an external crate) we can use `match`:

{% embed include file="src/examples/enums/colors-match/src/main.rs" %}
{% embed include file="src/examples/enums/colors-match/out.out" %}


