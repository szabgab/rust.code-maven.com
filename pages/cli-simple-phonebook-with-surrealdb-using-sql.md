---
title: Simple command line phonebook with SurrealDB using SQL
timestamp: 2023-12-30T21:30:01
author: szabgab
published: false
description:
tags:
    - SurrealDB
    - SQL
todo:
    - TODO
---

This is an implementation of a simple phonebook for the command line that uses an embedded SurrealDB databse with a RocksDb storage using SQL.

It seems that there are so many ways to use SurrealDB that we need that many words to describe the particular implementation we have.

SurrealDB can be used either as a stand-along database server

![](examples/surrealdb/cli-phone-book-with-embedded-rocksdb/Cargo.toml)
![](examples/surrealdb/cli-phone-book-with-embedded-rocksdb/src/main.rs)

![](examples/surrealdb/cli-phone-book-with-embedded-rocksdb/tests/test_empty.rs)
![](examples/surrealdb/cli-phone-book-with-embedded-rocksdb/tests/tests.rs)


