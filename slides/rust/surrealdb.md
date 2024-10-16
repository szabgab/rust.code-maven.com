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

```
cargo run -q > out.out 2>&1
```

![](examples/surrealdb/in-memory-demo/out.out)

## SurrealDB connect to server
{id: surrealdb-connect-to-server}

![](examples/surrealdb/connect-to-server/Cargo.toml)

![](examples/surrealdb/connect-to-server/src/main.rs)



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
{i: TODO}

![](examples/surrealdb/create-select/Cargo.toml)
![](examples/surrealdb/create-select/src/main.rs)

## SurrealDB - REMOVE NAMESPACE
{id: surrealdb-remove-namespace}

* [Bug: Server panic when trying to remove namespace](https://github.com/surrealdb/surrealdb/issues/3903)
* [Bug: InvalidQuery RenderedError for "REMOVE NAMESPACE IF EXISTS surrealdb"](https://github.com/surrealdb/surrealdb/issues/3904)

![](examples/surrealdb/remove-namespace/Cargo.toml)
![](examples/surrealdb/remove-namespace/src/main.rs)
![](examples/surrealdb/remove-namespace/out.out)

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
{i: TODO}

* TODO: The error handling should be improved

![](examples/surrealdb/toggle/src/main.rs)
![](examples/surrealdb/toggle/out.out)

## Multi-counter with embedded SurrealDB database
{id: multi-counter-with-embedded-surrealdb-database}

![](examples/surrealdb/cli-multi-counter/Cargo.toml)

![](examples/surrealdb/cli-multi-counter/src/main.rs)

![](examples/surrealdb/cli-multi-counter/tests/tests.rs)

## Get version of SurrealDB
{id: version-of-surrealdb}

![](examples/surrealdb/version/src/main.rs)
![](examples/surrealdb/version/out.out)



## Generate ID in SurrealDB
{id: generate-id}


![](examples/surrealdb/generate-id/src/main.rs)
![](examples/surrealdb/generate-id/out.out)

## Generate Thing in SurrealDB
{id: generate-thing}

![](examples/surrealdb/generate-thing/src/main.rs)
![](examples/surrealdb/generate-thing/out.out)


## Map field to id of other table (FOREIGN KEY)
{id: map-field-to-id-of-other-table}

![](examples/surrealdb/map-field-to-id-of-other-table/src/main.rs)
![](examples/surrealdb/map-field-to-id-of-other-table/out.out)
![](examples/surrealdb/map-field-to-id-of-other-table/out.sql)

## SurrealDB Datetime
{id: surrealdb-datetime}
{i: Datetime}

![](examples/surrealdb/surrealdb-sql-datetime/src/main.rs)
![](examples/surrealdb/surrealdb-sql-datetime/out.out)

## Add column to table without a schema
{id: add-column-to-a-table-without-a-schema}

* In this example we don't have a schema
* First we have a table with a single column. We add data. List the data.
* Then we would like to add a second column. We have two options. In the `Second` example we marked the new field to be `Option`. That way we can use the data with and without a value in the new column.
* In the `Third` example we require the new column. This means we cannot use the `SELECT` until we add values for the new column in every row in the table.

![](examples/surrealdb/add-columns-without-schema/Cargo.toml)

![](examples/surrealdb/add-columns-without-schema/src/main.rs)

![](examples/surrealdb/add-columns-without-schema/out.out)


## SurrealDB - Schema
{id: surrealdb-schema}

## SurrealDB - define field type - try to create entry with incorrect type (int, string)
{id: surrealdb-define-field0type-int}
{i: SCHEMAFULL}
{i: TABLE}
{i: FIELD}
{i: DEFINE}

* Using the `DEFINE` keyword we can define a table to have schema, then we can define the column and their type.
* Then if we try to insert (CREATE) an entry that does not match the type the query will fail. We need to use `check` to verify success.

![](examples/surrealdb/define-field-type/src/main.rs)

![](examples/surrealdb/define-field-type/out.out)

## SurrealDB - extra fields are ignored in SCHEMAFULL
{id: surrealdb-extra-fields-are-ignored}

* We DEFINE a SCHEMAFULL table with several field.
* If we try to create an entry using an extra field that field will be silently ignored.
* [see feature request](https://github.com/surrealdb/surrealdb/issues/4907)

![](examples/surrealdb/schemafull-ignore-extra-fields/src/main.rs)

![](examples/surrealdb/schemafull-ignore-extra-fields/out.out)

## SurrealDB - missing field
{id: surrealdb-missing-field}

* We have a SCHEMAFULL table but not all the fields are supplied.
* The CREATE will fail on missing fields

![](examples/surrealdb/missing-field/src/main.rs)
![](examples/surrealdb/missing-field/out.out)

## SurrealDB - add field to schema
{id: surrealdb-add-field-to-schema}

* We have a schema and some entries in the databas.
* We change the schema adding another field, but that field does not exist in the data that is already in the database.
* For new data we have to supply the new field as well.
* For old data we don't have to make changes but when we SELECT the data we need to have a struct where this field is `Option`.

![](examples/surrealdb/add-field-to-schema/src/main.rs)
![](examples/surrealdb/add-field-to-schema/out.out)

## SurrealDB in Docker using the CLI
{id: surrealdb-in-docker-using-cli}
{i: TODO}

* TODO: types of data?
* TODO: schema?
* TODO: index


* We would like to try SurrealDB, but we rather use Docker instead of installing SurrealDB on our computer.
* At first we'll use the SurrealDB command line with an in-memory (not persistent) database.

We could use the `latest` tag, but to make sure total reproducability we prefer to pick the [latest tag](https://hub.docker.com/r/surrealdb/surrealdb/tags) in the  2.* series.

```
$ docker run -it --rm  --user root surrealdb/surrealdb:v2.0.2 sql --endpoint memory --ns ns --db db --pretty
```

* Using `ns` as our namespace and `db` as our database so they won't take much real estate in the prompt. Feel free to use anything else.


* Create the first entry in the new `planet` table with an automatically generated ID.

```
ns/db> CREATE planet SET name = 'Earth';
-- Query 1 (execution time: 906.201µs)
[
	{
		id: planet:l0tpjh7uykux7sr5johy,
		name: 'Earth'
	}
]
```

* Create another entry and set the ID manually.

```
ns/db> CREATE planet:4 SET name = 'Mars';
-- Query 1 (execution time: 315.322µs)
[
	{
		id: planet:4,
		name: 'Mars'
	}
]

```


```
ns/db> CREATE planet SET name = 'Mercury', distance = 0.7;
-- Query 1 (execution time: 544.853µs)
[
	{
		distance: 0.7f,
		id: planet:hxkrucoezhwmtayok509,
		name: 'Mercury'
	}
]

ns/db> select * from planet;
-- Query 1 (execution time: 176.03µs)
[
	{
		id: planet:4,
		name: 'Mars'
	},
	{
		id: planet:6xg3xbsnzbkyolssmtb6,
		name: 'Earth'
	},
	{
		distance: 0.7f,
		id: planet:hxkrucoezhwmtayok509,
		name: 'Mercury'
	}
]
```

```
ns/db> INFO for db;
-- Query 1 (execution time: 183.976µs)
{
	accesses: {},
	analyzers: {},
	configs: {},
	functions: {},
	models: {},
	params: {},
	tables: {
		person: 'DEFINE TABLE person TYPE ANY SCHEMALESS PERMISSIONS NONE',
		planet: 'DEFINE TABLE planet TYPE ANY SCHEMALESS PERMISSIONS NONE'
	},
	users: {}
}
```

* Select all the items from more than one table

```
ns/db> SELECT * from person, planet
-- Query 1 (execution time: 254.849µs)
[
	{
		home: planet:4,
		id: person:ea6qfpbi8mu9prnn8nfx,
		name: 'Elon Musk'
	},
	{
		home: planet:6xg3xbsnzbkyolssmtb6,
		id: person:f2c3w5699hx64q1guta6,
		name: 'Gabor'
	},
	{
		id: planet:4,
		name: 'Mars'
	},
	{
		id: planet:6xg3xbsnzbkyolssmtb6,
		name: 'Earth'
	},
	{
		distance: 0.7f,
		id: planet:hxkrucoezhwmtayok509,
		name: 'Mercury'
	}
]

```

## SurrealDB - references and SELECT
{id: surrealdb-references-and-select}
{i: TODO}

![](examples/surrealdb/references-and-selects/Cargo.toml)
![](examples/surrealdb/references-and-selects/src/main.rs)
![](examples/surrealdb/references-and-selects/out.out)
![](examples/surrealdb/references-and-selects/out.sql)


## SurrealDB Demo
{id: surreldb-demo}
{i: TODO}

![](examples/surrealdb/demo/Cargo.toml)
![](examples/surrealdb/demo/src/main.rs)

## SurrealDB columns with schema
{id: surrealdb-columns-with-schema}
{i: TODO}

![](examples/surrealdb/columns-with-schema/Cargo.toml)
![](examples/surrealdb/columns-with-schema/src/main.rs)

