# Add column to table without a schema

* In this example we don't have a schema
* First we have a table with a single column. We add data. List the data.
* Then we would like to add a second column. We have two options. In the `Second` example we marked the new field to be `Option`. That way we can use the data with and without a value in the new column.
* In the `Third` example we require the new column. This means we cannot use the `SELECT` until we add values for the new column in every row in the table.

{% embed include file="src/examples/surrealdb/add-columns-without-schema/Cargo.toml" %}

{% embed include file="src/examples/surrealdb/add-columns-without-schema/src/main.rs" %}

{% embed include file="src/examples/surrealdb/add-columns-without-schema/out.out" %}


