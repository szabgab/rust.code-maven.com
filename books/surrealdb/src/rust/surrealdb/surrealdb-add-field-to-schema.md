# SurrealDB - add field to schema

* We have a schema and some entries in the databas.
* We change the schema adding another field, but that field does not exist in the data that is already in the database.
* For new data we have to supply the new field as well.
* For old data we don't have to make changes but when we SELECT the data we need to have a struct where this field is `Option`.

{% embed include file="src/examples/surrealdb/add-field-to-schema/src/main.rs" %}
{% embed include file="src/examples/surrealdb/add-field-to-schema/out.out" %}


