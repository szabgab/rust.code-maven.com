# SurrealDB with RocksDB backend in Rust embedded client with local database storage

* SurrealDB
* kv-rocksdb
* RocksDB

* We can also use local database files (just like in sqlite).

* This version does not need an external database server either.

* The compilation time is longer as we also compile the database backend, but this can be used as an embedded, but already persistan database.

* No need for authentication here either.

{% embed include file="src/examples/surrealdb/embedded-rocksdb/Cargo.toml" %}

* This will create a folder called `tempdb` in the root of the crate. You could also give a path there to some other folder.

{% embed include file="src/examples/surrealdb/embedded-rocksdb/src/main.rs" %}


