# Redeclare immutable variable and change type - Shadowing

* When shadowing we can also change the type of the variable.
* We don't even need to make it mutable.
* e.g. we read from a file or from STDIN a string that we then convert to a number. We can use the same variable name.

{% embed include file="src/examples/variables/change-type/src/main.rs" %}
{% embed include file="src/examples/variables/change-type/out.out" %}


