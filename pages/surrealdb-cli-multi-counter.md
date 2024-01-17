---
title: Multi-counter with embedded SurrealDB database
timestamp: 2024-01-16T14:40:01
published: true
description: Simple demo of a command-line application using SurrealDB to store several counters.
tags:
    - SurrealDB
    - CLI
    - DEFINE
    - SELECT
    - INSERT
    - INDEX
    - DUPLICATE
---

Another one of my [counter examples](https://code-maven.com/counter) part of the [SurrealDB](/surrealdb) series. This one works on the command line and counts how many times we ran the program with different command line parameter:

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

The data will be stored in a [SurrealDB](/surrealdb) database. To make the setup easier we won't run a separate database, but we'll use the database embedded in our code.


## Dependencies in Cargo.toml

SurrealDB can use various storage backends, we are going to us [RocksDB](https://rocksdb.org/).

![](examples/surrealdb/cli-multi-counter/Cargo.toml)

## The code

### Getting the command line from args

This has nothing to do with SurrealDB yet. We only create a vector from the values on the command line:


```rust
let args = std::env::args().collect::<Vec<String>>();
```

### Getting the path to the database

We check the `DATABASE_PATH` environment variable and use that if it is available. This will be used by the tests to be able to use
a database in a temporary folder.

If that variable is not available then we are going to create the database in the `db` folder in the current directory.

I did not use the plain `db` because of a [bug](https://github.com/surrealdb/docs.surrealdb.com/issues/185) that was fixed recently.

I also used [unwrap](/unwrap) disregarding the possibility that we don't have a current working directory. For a more robust solution
you might want to deal with that as well.

```rust
let database_folder = match std::env::var("DATABASE_PATH") {
    Ok(val) => std::path::PathBuf::from(val),
    Err(_) => {
        let current_dir = std::env::current_dir().unwrap();
        current_dir.join("db")
    }
};
```

### Connect to the database on the filesystem

Then we connect to the database folder via the `RocksDb` driver.

```rust
let db = Surreal::new::<RocksDb>(database_folder).await?;
```

### Namespace and database

Each SurrealDB installation can handle multiple namespaces and multiple databases in each namespace. We just need to pick one for each:

```rust
db.use_ns("counter_ns").use_db("counter_db").await?;
```

### Make column unique

Let's make sure we don't add the same counter name twice.

```rust
let _response = db
    .query("DEFINE INDEX counter_name ON TABLE counter COLUMNS name UNIQUE")
    .await?;
```

### Too many arguments

Tell the user how to use the tool if there is more than one value on the command line. (args includes the name of the executable file so we need to compare with 2 here).

```rust
if 2 < args.len() {
    eprintln!("Usage: {} NAME     to count NAME", args[0]);
    eprintln!("       {}          to list all the counters", args[0]);
    std::process::exit(1);
}
```

### Increment the counter

I guess this is the heart of the program where we fetch the current value of the counter using an `INSERT` statement that
will either `CREATE` a new record or `UPDATE` and existing one. This will only work because we have defined the `INDEX` to be `UNIQUE`.

Thanks to Beryl on the SurrealDB Discord server for a much shorter and better solution than the one I wrote originally.

```rust
if 2 == args.len() {
    increment(&db, &args[1]).await?;
    return Ok(());
}
```

```rust
async fn increment(db: &Surreal<Db>, name: &str) -> surrealdb::Result<()> {
    let response = db
        .query("INSERT INTO counter (name, count) VALUES ($name, $count) ON DUPLICATE KEY UPDATE count += 1;")
        .bind(("name", name))
        .bind(("count", 1))
        .await?;

    match response.check() {
        Ok(mut entries) => {
            let entries: Vec<Entry> = entries.take(0)?;
            // fetching the first (and hopefully only) entry
            if let Some(entry) = entries.into_iter().next() {
                println!("{}", entry.count);
            }

            Ok(())
        }
        Err(err) => {
            eprintln!("Could not add entry {}", err);
            std::process::exit(2);
        }
    }
}
```

### Listing all the counters

The last part of the code deal with the case when we don't supply any parameter. It will fetch all the counters and print the name of the counter and the current count.

```rust
println!("Listing counters");
println!("----------------");
let mut entries = db
    .query("SELECT name, count FROM counter ORDER BY count DESC")
    .await?;
let entries: Vec<Entry> = entries.take(0)?;
for entry in entries {
    println!("{}: {}", entry.name, entry.count);
}
```


## The full code

![](examples/surrealdb/cli-multi-counter/src/main.rs)


## The tests

In the test we run the command line application as an external executable and then compare the result that was printed to the Standard Output with the expected values.

![](examples/surrealdb/cli-multi-counter/tests/tests.rs)

## Conclusion

This looks quite similar to plain SQL. We have at least one improvement to make in the code.

