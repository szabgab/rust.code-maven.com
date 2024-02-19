---
title: Setting up an in-memory SurrealDB database in Rust
timestamp: 2024-01-10T11:30:01
author: szabgab
published: true
description: Before getting started on any bigger demo, this example shows how to set up and in-memory Surreal Database in Rust.
tags:
    - SurrealDB
    - Mem
---

Before getting started on any bigger demo, this example shows how to set up and in-memory [Surreal Database](/surrealdb) in Rust.


## Dependencies

{% include file="examples/surrealdb/in-memory-setup/Cargo.toml" %}


## The code

{% include file="examples/surrealdb/in-memory-setup/src/main.rs" %}


## Conclusion

There is not much in this example.

This will create an in-memory database so anything we add to the database will be gone when the application stops running.

If you prefer a persistent database where the data is stored on the disk,
but you would still like to use the simple, embedded version, take a look at
[Setting up embedded SurrealDB with RocksDB backend](/surrealdb-embedded-with-rocksdb).


