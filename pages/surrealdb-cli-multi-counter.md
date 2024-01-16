---
title: Multi counter with embedded SurrealDB database
timestamp: 2024-01-16T14:40:01
published: true
description: Simple demo of a command-line application using SurrealDB to store several counters.
tags:
    - SurrealDB
    - CLI
---

Another one of my [counter examples](https://code-maven.com/counter). This one works on the command line and counts how many times we ran the program with different command line parameter:

It works like this:

```
$ counter foo
1
$ counter foo
2
$ counter foo
3
$ counter bar
1
$ counter bar
2
$ counter foo
4
$ counter
foo 4
bar 2
````

The data will be stored in a [SurrealDB](/surrealdb) database. To make the setup easier we won't run a separate database, but we'll use the databas embedded in our code.


## Dependencies in Cargo.toml

SurrealDB can use various storage backends, we are going to us [RocksDB](https://rocksdb.org/).

![](examples/surrealdb/cli-multi-counter/Cargo.toml)


## The full code

![](examples/surrealdb/cli-multi-counter/src/main.rs)


## The tests

![](examples/surrealdb/cli-multi-counter/tests/tests.rs)

