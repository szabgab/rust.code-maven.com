# SurrealDB in-memory with SQL demo in Rust - CREATE (INSERT), SELECT, UPDATE, DELETE

* Mem
* DEFINE
* CREATE
* SELECT
* UPDATE
* DELETE
* surrealdb::Result

* First think would be to authenticate, but we are skipping that in these examples
* then select the namespace and the database.
* There can be as many namespaces as you want and each namespace can have multipled databases.
* A namespace migh correspond to a department in a company and each database is for a product or (micro)service.

{% embed include file="src/examples/surrealdb/in-memory-demo/src/main.rs" %}

```
cargo run -q > out.out 2>&1
```

{% embed include file="src/examples/surrealdb/in-memory-demo/out.out" %}



