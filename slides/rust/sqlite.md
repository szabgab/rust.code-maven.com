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

## SQLite in-memory COUNT
{id: sqlite-in-memory-count}

![](examples/sqlite/in-memory-count/src/main.rs)
![](examples/sqlite/in-memory-count/out.out)

## SQLite in-memory plain placeholder (?)
{id: sqlite-in-memory-plain-placeholder}

![](examples/sqlite/in-memory-placeholder/src/main.rs)
![](examples/sqlite/in-memory-placeholder/out.out)

## SQLite SELECT with named placeholder - bind variables
{id: sqlite-select-with-named-placeholder-bind-variables}
{i: bind}
{i: prepare}

* When the values are in variables - which is almost always - then we should use placeholders and bind the variables to them.
* We can use the `bind` method to bind them one-by-one.

![](examples/sqlite/in-memory-select-placeholders-bind/src/main.rs)
![](examples/sqlite/in-memory-select-placeholders-bind/out.out)

## SQLite SELECT with placeholder
{id: sqlite-select-with-placeholder}
{i: prepare}
{i: bind_iter}
{i: next}

* We could also bind multiple variables in one statement. We can use either the index of the placeholder or its name. The latter makes more readable code.

![](examples/sqlite/in-memory-select-placeholders/src/main.rs)
![](examples/sqlite/in-memory-select-placeholders/out.out)

## SQLite INSERT with placeholder
{id: sqlite-insert-with-placeholder}
{i: prepare}
{i: bind}
{i: next}

* We can (and should) use placeholders in INSERT statements as well

![](examples/sqlite/in-memory-insert-placeholders/src/main.rs)
![](examples/sqlite/in-memory-insert-placeholders/out.out)

## SQLite Multi-Counter
{id: sqlite-multi-counter}

* Command line counter using the `counter.db` file as the database to store the counters.

![](examples/sqlite/counter/src/main.rs)

## SQLite with AUTOINCREMENT
{id: sqlite-with-autoincrement}
{i: AUTOINCREMENT}
{i: TBD}

* Currently this example does not work.
* [issue](https://github.com/stainless-steel/sqlite/issues/99)

![](examples/sqlite/with-autoincrement/src/main.rs)

## SQLite - field with DEFAULT value
{id: sqlite-field-with-default-value}
{i: TBD}

* Currently this example does not work. See AUTOINCREMENT

![](examples/sqlite/field-with-default/src/main.rs)

TODO: User and Group (with )

