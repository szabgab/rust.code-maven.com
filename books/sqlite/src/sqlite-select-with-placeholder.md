# SQLite SELECT with placeholder

* prepare
* bind_iter
* next

* We could also bind multiple variables in one statement. We can use either the index of the placeholder or its name. The latter makes more readable code.

{% embed include file="src/examples/sqlite/in-memory-select-placeholders/src/main.rs" %}

{% embed include file="src/examples/sqlite/in-memory-select-placeholders/out.out" %}

