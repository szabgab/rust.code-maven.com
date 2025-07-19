# SurrealDB - define field type - try to create entry with incorrect type (int, string)

* SCHEMAFULL
* TABLE
* FIELD
* DEFINE

* Using the `DEFINE` keyword we can define a table to have schema, then we can define the column and their type.
* Then if we try to insert (CREATE) an entry that does not match the type the query will fail. We need to use `check` to verify success.

{% embed include file="src/examples/surrealdb/define-field-type/src/main.rs" %}

{% embed include file="src/examples/surrealdb/define-field-type/out.out" %}


