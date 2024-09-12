# SurrealDB
{id: surrealdb}

## What is SurrealDB
{id: what-is-surrealdb}

* [SurrealDB](https://surrealdb.com/)

* We can use it embedded or as a separate server.
* Embedded can work with in-memory databse (which is not persistent) or with on-disk backend.
* When using as a separate server we can have one node or multiple nodes.

* SurrealDB itself is written in Rust and can be used with multiple programming languages.

* [SurrealDB for SQL developers](https://surrealdb.com/docs/surrealdb/introduction/sql)
* [SurrealDB for MongoDB developers](https://surrealdb.com/docs/surrealdb/introduction/mongo)

## SurrealDB in-memory database in Rust
{id: surrealdb-in-memory-database-in-rust}
{i: kv-mem}
{i: Mem}

![](examples/surrealdb/in-memory-setup/Cargo.toml)

![](examples/surrealdb/in-memory-setup/src/main.rs)

## SurrealDB with RocksDB backend in Rust
{id: surrealdb-with-rocksdb-backend-in-rust}
{i: SurrealDB}
{i: RocksDB}

![](examples/surrealdb/embedded-rocksdb/Cargo.toml)

* This will create a folder called `tempdb` in the root of the crate. You could also give a path there to some other folder.

![](examples/surrealdb/embedded-rocksdb/src/main.rs)

## Start SurrealDB in Docker
{id: start-surrealdb-in-docker}

* Install Docker
* Create a volume to store the data

```
docker volume create my-surreal-db
```

* Start the Docker container using the latest SurrealDB image:

```
docker run --name surrealdb --rm -p 8000:8000 --user root -v my-surreal-db:/database surrealdb/surrealdb:latest start --log trace file://database
```

* Or a specific version:

```
docker run --name surrealdb --rm -p 8000:8000 --user root -v my-surreal-db:/database surrealdb/surrealdb:v1.5.5 start --log trace file://database
```

* This one will listen on port 8000. You could tell it to listen on some other port. e.g. port 8001:  `-p 8001:8000`.

* Stop the container:

```
Ctrl-c
```

* Remove the volume

```
docker volume remove my-surreal-db
```

* See the [tags](https://hub.docker.com/r/surrealdb/surrealdb/tags) for available versions.


## SurrealDB in-memory with SQL demo in Rust - CREATE (INSERT), SELECT, UPDATE, DELETE
{id: surrealdb-in-memory-with-sql-demo-in-rust}
{i: Mem}
{i: DEFINE}
{i: CREATE}
{i: SELECT}
{i: UPDATE}
{i: DELETE}
{i: surrealdb::Result}

* First think would be to authenticate, but we are skipping that in these examples
* then select the namespace and the database.
* There can be as many namespaces as you want and each namespace can have multipled databases.
* A namespace migh correspond to a department in a company and each database is for a product or (micro)service.

![](examples/surrealdb/in-memory-demo/src/main.rs)



## SurrealDB create INSERT SELECT
{id: surrealdb-insert-select}
{i: create}
{i: use_db}
{i: use_ns}

![](examples/surrealdb/insert-select/Cargo.toml)
![](examples/surrealdb/insert-select/src/main.rs)

```
$ cargo run -q "Hello World"
[src/main.rs:34:9] created = [
    Record {
        id: Thing {
            tb: "messages",
            id: String(
                "85d6tb5du6fsxp31wtnj",
            ),
        },
    },
]

$ cargo run -q
Hello World

$ cargo run -q "Other message"
[src/main.rs:34:9] created = [
    Record {
        id: Thing {
            tb: "messages",
            id: String(
                "z17l4ylmkbyfj05iozqs",
            ),
        },
    },
]

$ cargo run -q
Hello World
Other message
```
## SurrealDB experiments
{id: surrealdb-experiments}
{i: TBD}

![](examples/surrealdb/create-select/Cargo.toml)
![](examples/surrealdb/create-select/src/main.rs)

## SurrealDB - REMOVE NAMESPACE
{id: surrealdb-remove-namespace}

* [Bug: Server panic when trying to remove namespace](https://github.com/surrealdb/surrealdb/issues/3903)
* [Bug: InvalidQuery RenderedError for "REMOVE NAMESPACE IF EXISTS surrealdb"](https://github.com/surrealdb/surrealdb/issues/3904)

![](examples/surrealdb/remove-namespace/Cargo.toml)
![](examples/surrealdb/remove-namespace/src/main.rs)

## INSERT and SELECT in Memory
{id: insert-and-select}


![](examples/surrealdb/insert-select-in-memory/src/main.rs)

![](examples/surrealdb/insert-select-in-memory/out.out)


## SurrealDB - CREATE, SELECT, DELETE
{id: surreldb-create-select-delete}
{i: CREATE}
{i: SELECT}
{i: DELETE}

![](examples/surrealdb/create-select-delete/src/main.rs)
![](examples/surrealdb/create-select-delete/out.out)

## SurrealDB - Datetime with Chrono
{id: surrealdb-datetime-with-chrono}
{i: DATETIME}

* [Datetimes in SurrealDB](https://surrealdb.com/docs/surrealdb/surrealql/datamodel/datetimes)

![](examples/surrealdb/date-and-time/src/main.rs)
![](examples/surrealdb/date-and-time/out.out)

## SurrealDB - CREATE, SELECT, UPDATE, DELETE
{id: suurealdb-create-select-update-delete}
{i: update}

![](examples/surrealdb/update/src/main.rs)
![](examples/surrealdb/update/out.out)

## SurrealDB - RSVP
{id: surrealdb-rsvp}

* Set a value to true or false and if it does not exist, create it first with the appropriate starting value

![](examples/surrealdb/rsvp/src/main.rs)
![](examples/surrealdb/rsvp/out.out)


## SurrealDB - toggle
{id: surrealdb-toggle}
{i: TBD}

* TODO: The error handling should be improved

![](examples/surrealdb/toggle/src/main.rs)
![](examples/surrealdb/toggle/out.out)

## Multi-counter with embedded SurrealDB database
{id: multi-counter-with-embedded-surrealdb-database}

![](examples/surrealdb/cli-multi-counter/Cargo.toml)

![](examples/surrealdb/cli-multi-counter/src/main.rs)

![](examples/surrealdb/cli-multi-counter/tests/tests.rs)

## Generate ID
{id: generate-id}


![](examples/surrealdb/generate-id/src/main.rs)
![](examples/surrealdb/generate-id/out.out)

