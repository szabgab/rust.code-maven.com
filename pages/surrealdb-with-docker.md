---
title: Getting started with SurrealDB using Docker and a Rust client
timestamp: 2024-04-17T16:00:01
author: szabgab
published: true
description: Getting started with SurrealDB using Docker
tags:
    - SurrealDB
    - Docker
---

This is a simple example on how to use [SurrealDB](/surrealdb) running in Docker and connect to it from your computer using
a Rust-based client.

It is a simplified version of the [example provided by SurrealDB](https://surrealdb.com/docs/surrealdb/integration/sdks/rust).

I assume you already have [Docker](https://docker.com/) and [Rust](https://www.rust-lang.org/) installed.

## Start the SurrealDB in Docker

### Create a volume for the database

First we create a volume in Docker to make it easy to have a persistent database without the need to think where to put it.

```
docker volume create my-surreal-db
```

### Start the database server in one terminal

Then we can start the SurrealDB server in a Docker container.

The `--name` flag  tells Docker to call the container `surrealdb`. This is an arbitrary name that can help us identify it if necessary.

The `--rm` flag tells Docker to remove the container once it stopped running so having an easily recognizable name might not be that important.

The `-p 8000:8000` flag tells Docker to share the internal port 8000 where SurrealDB listens on the 8000 port of the computer. By default this will only accept connections from your computer so this is fine.

The `--user root` tells the Docker to run as user root.

The `-v my-surreal-db:/database` maps the volume we created earlier to the `/database` folder internally. Apparently this is where SurrealDB stores its files.

The `surrealdb/surrealdb:latest` is the name of of the [image](https://hub.docker.com/r/surrealdb/surrealdb) we will use. It will be downloaded automatically if you don't have it on the disk yet.

The `start --log trace file://database` is passed to SurrealDB. This is here we tell what is the name of the user and its password we would accept. It is also here where we tell SurrealDB to store its files in the `/database` folder.

```
docker run --name surrealdb --rm -p 8000:8000 --user root \
   -v my-surreal-db:/database surrealdb/surrealdb:latest   \
   start --log trace file://database
```

## The Rust client

Once we have the database server running we need to create a crate for our code, add the dependencies and run.


### Create the crate

```
$ cargo new in-docker-demo
$ cd in-docker-demo/
```

### Add the dependencies to Cargo.toml

Obviously we need [SurrealDB](https://crates.io/crates/surrealdb), but we also need [serde](https://crates.io/crates/serde) to serialized and deserialize the structs and we need [tokio](https://crates.io/crates/tokio) for the async.

The specific versions probably do not matter much but if you want to check the code, you'll find the `Cargo.lock` file in our GitHub repository.

{% include file="examples/surrealdb/in-docker-demo/Cargo.toml" %}

### The code

Finally we have the code:

{% include file="examples/surrealdb/in-docker-demo/src/main.rs" %}

First we connect to the server that listens on port 8000 (as defined on the Docker command line) on 127.0.0.1 (aka. localhost) as that's the default for Docker.

Then we select the namespace and database we are going to use. You can use any name there. The idea behind them is to allow you to map departments and project
in a single organization to namespaces and databases.

Then we get a value from the command line. If the user supplied a value we use that value to create a new record in the database.

Otherwise we list the existing messages.

## Running the code

This will add the message "Hello World" to the database:

```
cargo run "Hello World"
```

This will list all the messages.

```
cargo run
```


## Stop and restart server

We can stop the database by pressing Ctrl-C.

We can then start it again with the same `docker` command we started it earlier.

If we list the messages now they are all still there as they were saved in the database on that volume somewhere on your hard-disk.


## Remove the volume

The volume with the database takes up some space on your disk. After you are done with all the experiments you can remove it using the following command:

```
docker volume remove my-surreal-db
```


