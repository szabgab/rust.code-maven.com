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

