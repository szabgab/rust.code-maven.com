---
title: Setting up embedded SurrealDB with RocksDB backend in Rust
timestamp: 2024-01-10T10:30:01
author: szabgab
published: true
description: A simple example just to show the basics of setting up and embedded SurrealDB database in Rust
tags:
    - SurrealDB
    - RocksDB
todo:
    - example with in-memory database
---

Before we get into the use of [SurrealDB](/surrealdb) from Rust, let's see a very simple example of setting it up as an **embedded** database.

Depending on the use case you might want a stand-alone running SurrealDB to which you can connect from some external process or you might want a database
that does not need a separate process. In this case you might use an in-memory version of SurreealDB, but for my current goal I wanted to have a persistent database
where the data is stored on the disk. I just did not want to deal with a separate database server process.

For the disk storage I had to pick one of the database systems that provide the file stores. (As I understand there are several options. I went with **RocksDB**.


## Crate a crate

```
cargo new embedded-rocksdb
cd embedded-rocksdb
```

## Dependencies

These are the dependencies in the `Cargo.toml` file.

![](examples/surrealdb/embedded-rocksdb/Cargo.toml)

## The code

![](examples/surrealdb/embedded-rocksdb/src/main.rs)


## Running the code

```
cargo run
```

Running this code will first need to compile it that might take several minutes. Then it will create a **folder** called `tempdb` thats store the database.

This folder will be created in the root of the crate.

In some earlier versions of SurrealDB this feature was [broken](https://github.com/surrealdb/docs.surrealdb.com/issues/185) and I had to use a full-path to the folder.
I left that code in as comment to show you that you can also provide a full path to the database folder.

## Conclusion

I know we have not done much yet, but I like to celebrate even small steps.

If you need an in-memory database with slight modifications this can be converted to one.
See [Setting up an in-memory SurrealDB database in Rust](/surrealdb-embedded-with-in-memory-database).


