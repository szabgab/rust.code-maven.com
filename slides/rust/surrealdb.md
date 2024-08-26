# SurrealDB
{id: surrealdb}

## Start SurrealDB in Docker
{id: start-surrealdb-in-docker}

* Install Docker
* Create a volume to store the data

```
docker volume create my-surreal-db
```

* Start the Docker container using the latest SurrealDB image:

```
docker run --name surrealdb --rm -p 8001:8000 --user root -v my-surreal-db:/database surrealdb/surrealdb:latest start --log trace file://database
```

* Stop the container:

```
Ctrl-c
```

* Remove the volume

```
docker volume remove my-surreal-db
```


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

