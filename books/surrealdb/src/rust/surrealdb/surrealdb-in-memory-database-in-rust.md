# SurrealDB in-memory database in Rust

* kv-mem
* Mem

Using the in-memory database can be very useful, especially in short-lived examples ike the ones we have here,
but in other cases as well. It does not need any additional server component.

In this example we only setup the database connection, the namespace and the database without doing anything.

For the in-memory database we don't need authentication as only our process can access it.

{% embed include file="src/examples/surrealdb/in-memory-setup/Cargo.toml" %}

{% embed include file="src/examples/surrealdb/in-memory-setup/src/main.rs" %}


