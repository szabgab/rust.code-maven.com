# SurrealDB - extra fields are ignored in SCHEMAFULL

* We DEFINE a SCHEMAFULL table with several field.
* If we try to create an entry using an extra field that field will be silently ignored.
* [see feature request](https://github.com/surrealdb/surrealdb/issues/4907)

{% embed include file="src/examples/surrealdb/schemafull-ignore-extra-fields/src/main.rs" %}

{% embed include file="src/examples/surrealdb/schemafull-ignore-extra-fields/out.out" %}


