# SQLite in memory example with INSERT and SELECT

* memory
* open
* execute
* INSERT
* SELECT

* In this example we use an in-memory database.
* That's useful for the examples and it can be also useful if we have a lot of data on the disk that we would like analyze using SQL statements but we don't need to store the data in a database.

* In general you won't have values embedded in the SQL statements, but in this first example we use that for simplicity.

{% embed include file="src/examples/sqlite/in-memory-example/Cargo.toml" %}

{% embed include file="src/examples/sqlite/in-memory-example/src/main.rs" %}

{% embed include file="src/examples/sqlite/in-memory-example/out.out" %}


