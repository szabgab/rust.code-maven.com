---
title: SurrealDB - relation between tables - foreign key
timestamp: 2024-02-03T14:30:01
author: szabgab
published: false
description: Tables can have relationships one-to-many, many-to-many using foreign keys.
tags:
    - SurrealDB
todo:
    - TODO
---


## Launching the database in Docker

In this example we are going to use SurrealDB running in a Docker container. All you need to do to launch is is to install [Docer](https://www.docker.com/)
and then to launch the database with the following command:

```
docker run --name surrealdb --rm -p 8000:8000 --user root \
   -v my-surreal-db:/database surrealdb/surrealdb:latest   \
   start --log trace file://database
```

For detailed explanation check out the article [Getting started with SurrealDB using Docker and a Rust client](/surrealdb-with-docker).


{% include file="examples/surrealdb/foreign-key/Cargo.toml" %}

{% include file="examples/surrealdb/foreign-key/src/main.rs" %}

{% include file="examples/surrealdb/foreign-key/src/example.rs" %}

