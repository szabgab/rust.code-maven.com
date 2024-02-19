---
title: SurrealDB in-memory with SQL demo in Rust
timestamp: 2024-01-10T18:30:01
author: szabgab
published: true
description: In-memory databases can be used for easier data analysis and also for showing example without the need to think about the disk.
tags:
    - SurrealDB
    - Mem
    - surrealdb::Result
    - CREATE
    - SELECT
    - UPDATE
    - DELETE
---

In this article we'll use [SurrealDB](/surrealdb) embedded in our Rust program using an in-memory database.
As an in-memory database does not have persistent storage it is not good to store data for a long period of time,
but sometimes it can be quite useful. For example one can load data into such database from various other formats
and then use the available query language to create reports from the data.


## Dependencies

These are the same as we saw in the example [setting up an in-memory database](/surrealdb-embedded-with-in-memory-database).

{% include file="examples/surrealdb/in-memory-demo/Cargo.toml" %}

## Setting up the in-memory Surreal database

```rust
let db = Surreal::new::<Mem>(()).await?;
```

## Defining the namespace and the name of the database

```rust
db.use_ns("test_ns").use_db("test_db").await?;
```

These can be arbitrary strings. They are probably a lot more interesting in on-disk databases where multiple applications
use the same server and we would like to have a clear distinction between them.


## Add an index to a column and make it UNIQUE

```rust
let _response = db
    .query("DEFINE INDEX entry_email ON TABLE entry COLUMNS name UNIQUE")
    .await?;
```

This will setup and index for faster data lookup and it will enforce the uniqueness of the "name" field (column in SQL) in the "entry" table.

As you see we did not have to defined the schema and we could already add rules to the database. Strange, but interesting.

This line is not really needed for most of the example.
The reason I added it is so I can demonstrate a failure when I try to add a new record to the database
with a "name" that is already in the database. Then the `UNIQUE` rule will reject it.

## CREATE record (INSERT data)

In order to add data to the database we need to execute the `CREATE` command of the **SurrealQL** which is similar to `INSERT` in SQL.

I put it into a separate function to make it reusable and to have a clear separation.

The function expects to received the object connecting us to the database and a vector of tuples we'll insert into the database.

We iterate over the tuples.

In the `query` we use named placeholders for the value of the two fields we are inserting.

So basically we will have a table with two column (fields) "name" and "phone".

Then we use the `bind` method to bind the value in the `name` variable to the placeholder called `name`. Same with the `phone` field.


```rust
async fn add_to(db: &Surreal<Db>, data: Vec<(&str, &str)>) -> surrealdb::Result<()> {
    for (name, phone) in data {
        let response = db
            .query("CREATE entry SET name=$name, phone=$phone")
            .bind(("name", name))
            .bind(("phone", phone))
            .await?;

        match response.check() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Could not add entry: '{}'", err);
                return Err(err);
            }
        };
    }
    Ok(())
}
```

## SELECT to fetch data

This query will fetch all the data, so it only needs access to the `db`.

In this example we used a placeholder for the name of the table.

I could say I was doing this because I found great education value in showing different ways providing the name of the table
but the reality is that this example grew out of an example on the [SurrealDB](https://docs.surrealdb.com/) web site and
that's how I know I can use a placeholder for the name of the table as well.

I guess this helps eliminating another set of [SQL injection attacks](https://bobby-tables.com/) when the table name is inferred from the
http request.


```rust
async fn list_all(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut entries = db
        .query("SELECT name, phone FROM type::table($table) ORDER BY name ASC")
        .bind(("table", "entry"))
        .await?;
    let entries: Vec<Entry> = entries.take(0)?;
    for entry in entries {
        println!("{}: {}", entry.name, entry.phone);
    }

    Ok(())
}
```

In this example we used a struct called `Entry` we defined at the beginning of the file to map the field of the table to attributes of a struct:

```rust
#[derive(Debug, Deserialize)]
struct Entry {
    name: String,
    phone: String,
}

```

## UPDATE field of a row


It looks very similar to an SQL statement.

``rust
async fn update(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let name = "Jane";
    let phone = "55555555";

    let response = db
        .query("UPDATE entry SET phone=$phone WHERE name=$name")
        .bind(("name", name))
        .bind(("phone", phone))
        .await?;

    match response.check() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Could not add entry {}", err);
            Err(err)
        }
    }
}
```


## DELETE a record (a row)

This too looks like regular SQL.

```rust
async fn delete(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let name = "Jane";

    let response = db
        .query("DELETE entry WHERE name=$name")
        .bind(("name", name))
        .await?;

    match response.check() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Could not delete entry {}", err);
            Err(err)
        }
    }
}
```


## The full source code

{% include file="examples/surrealdb/in-memory-demo/src/main.rs" %}


## The output or cargo run

Don't be surprised if the first time running this takes several minutes. Effectively Rust needs to compile the whole database code-base before our program can run.


```
$ cargo run
   Compiling in-memory-demo v0.1.0 (/home/gabor/work/rust.code-maven.com/examples/surrealdb/in-memory-demo)
    Finished dev [unoptimized + debuginfo] target(s) in 5.82s
     Running `target/debug/in-memory-demo`
Jane: 456
Jil: 789
Joe: 123
-------------
Jane: 55555555
Jil: 789
Joe: 123
-------------
Jil: 789
Joe: 123
-------------
Could not add entry: 'Database index `entry_email` already contains 'Joe', with record `entry:g0ebkz9yqzjnzyumv4r0`'
Error: Db(IndexExists { thing: Thing { tb: "entry", id: String("g0ebkz9yqzjnzyumv4r0") }, index: "entry_email", value: "'Joe'" })
```

## Conclusion

It was a nice experiment creating this example, but now we need something bigger, something filesystem-based. Check out the other [articles about SurrealDB with Rust](/surrealdb).

