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
{i: last_insert_rowid}

* See also [last_insert_rowid](https://sqlite.org/c3ref/last_insert_rowid.html).

![](examples/sqlite/with-autoincrement/src/main.rs)
![](examples/sqlite/with-autoincrement/out.out)

## SQLite - field with DEFAULT value
{id: sqlite-field-with-default-value}


![](examples/sqlite/field-with-default/src/main.rs)
![](examples/sqlite/field-with-default/out.out)

## SQLite Transaction
{id: sqlite-transacions}

The default behavior of SQLite is that statement that change the database (e.g. INSERT, UPDATE, DELETE) only prepare the action,
but the user still needs to issue `COMMIT` command to have the statement take effect.

The default connection of the Rust SQLite client uses `autocommit`, meaning that using this crate we don't need issue the `COMMIT`commend. This is easier, however it raises another issue.

What if we would like to have 2 or more statements to be atomit. That is either all of them succeed to all of them fail.
For example if we implement a bank a money transfer from user Mary to use Jane would need 2 statement: deduct the money from
Mary and add the money to Jane. What if there is some error after the deduction (e.g. a panic, or someone reboots the computer, or there is a power failure?) Then the system lost the money that was deducted from Mary.

The solution is to use transactions. A transaction starts with the `BEGIN` statement and ends with a `COMMIT` statement. In between the two you can have any number of actions that would change the database. If the code does not reach the `COMMIT` statement (e.g. because of the said panic, then none of the actions will take place. The money will stay in its original location.

The `BEGIN` statement turns off the autocommit mode and the `COMMIT` turns it on again.

In this example we tried to imitate the issue.

![](examples/sqlite/transaction/Cargo.toml)

![](examples/sqlite/transaction/src/main.rs)


```
$ cargo run -q show
Jane :    0
Mary : 1000
Ann  : 1000
Total: 2000
-----

$ cargo run -q plain
Jane :  100
Mary :  900
Ann  : 1000
Total: 2000
-----

$ cargo run -q panic
thread 'main' panicked at src/main.rs:81:9:
Problem
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ cargo run -q show
Jane :    0
Mary :  900
Ann  : 1000
Total: 1900
-----

$ cargo run -q transaction
BEGIN
thread 'main' panicked at src/main.rs:81:9:
Problem
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ cargo run -q show
Jane :    0
Mary : 1000
Ann  : 1000
Total: 2000
-----
```

## SQLite transaction - bank
{id: sqlite-transaction-in-a-bank}

![](examples/sqlite/bank/Cargo.toml)
![](examples/sqlite/bank/src/main.rs)

## SQLite - Groups and Owners (with FOREIGN KEY)
{id: sqlite-groups-and-owners}
{i: FOREIGN KEY}

In order to make SQLite enforce FOREIGN KEY we need to turn on the [foreign_keys PRAGMA](https://sqlite.org/pragma.html#pragma_foreign_keys) on the connection.

* There is also a compile-time option called [default_foreign_keys](https://sqlite.org/compile.html#default_foreign_keys) but I think we don't want to compile our own sqlite.

![](examples/sqlite/groups-and-owner/src/main.rs)


## SQLite get_autocommit function
{id: sqlite-get-autocommit}
{i: get_autocommit}
{i: raw}

* See [get_autocommit](https://sqlite.org/c3ref/get_autocommit.html)
* [All the SQLite functions](https://sqlite.org/c3ref/funclist.html)

![](examples/sqlite/sqlite_get_autocommit/src/main.rs)

