# SurrealDB - missing field


* We have a SCHEMAFULL table but not all the fields are supplied.
* The CREATE will fail on missing fields

{% embed include file="src/examples/surrealdb/missing-field/src/main.rs" %}
{% embed include file="src/examples/surrealdb/missing-field/out.out" %}


