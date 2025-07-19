# Literal string in mutable variable

* mut

* The variable can be made mutable using `mut`. Then it can be replaced, but the literal (hard-coded) string is baked into the code of the program and thus it cannot be changed runtime.
* This is an `str` type.

{% embed include file="src/examples/ownership/literal-string-in-mutable-variable/src/main.rs" %}
{% embed include file="src/examples/ownership/literal-string-in-mutable-variable/out.out" %}
{% embed include file="src/examples/ownership/literal-string-in-mutable-variable/err.out" %}


