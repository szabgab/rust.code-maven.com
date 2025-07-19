# Enumeration with non-exhaustive patterns

* enum
* dead_code

* In this example in the `match` we don't hanle every variant of the enum and thus we have to handle the "deafult" case using and `_` underscore.
* Try running this code after commenting out the row handline `_`.

{% embed include file="src/examples/enums/weekdays/src/main.rs" %}
{% embed include file="src/examples/enums/weekdays/out.out" %}


