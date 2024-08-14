# SQLite
{id: sqlite}

## SQLite crate
{id: sqlite-crate}

* [SQLite](https://sqlite.org/) - the most populare embedded relational database.
* [sqlite crate](https://crates.io/crates/sqlite)

## SQLite in memory example with INSERT and SELECT
{id: sqlite-in-memory-example}
{i: memory}
{i: open}
{i: execute}
{i: INSERT}
{i: SELECT}

* In this example we use an in-memory database.
* That's useful for the examples and it can be also useful if we have a lot of data on the disk that we would like analyze using SQL statements but we don't need to store the data in a database.

* In general you won't have values embedded in the SQL statements, but in this first example we use that for simplicity.

![](examples/sqlite/in-memory-example/Cargo.toml)
![](examples/sqlite/in-memory-example/src/main.rs)
![](examples/sqlite/in-memory-example/out.out)



## SQLite SELECT with named placeholder - bind variables
{id: sqlite-select-with-named-placeholder-bind-variables}
{i: bind}
{i: prepare}

![](examples/sqlite/in-memory-select-placeholders-bind/src/main.rs)
![](examples/sqlite/in-memory-select-placeholders-bind/out.out)

## SQLite SELECT with placeholder
{id: sqlite-select-with-placeholder}
{i: prepare}
{i: bind_iter}
{i: next}

![](examples/sqlite/in-memory-select-placeholders/src/main.rs)
![](examples/sqlite/in-memory-select-placeholders/out.out)

## SQLite INSERT with placeholder
{id: sqlite-insert-with-placeholder}
{i: prepare}
{i: bind}
{i: next}

![](examples/sqlite/in-memory-insert-placeholders/src/main.rs)

## SQLite Counter
{id: sqlite-counter}

![](examples/sqlite/counter/src/main.rs)

