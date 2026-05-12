
A sample command-line application to demonstrate transactions using Rust, sqlx, and sqlite.

The user needs to define the name of the database file using the following command.

```
$ export DATABASE_URL=sqlite://bank.db
```

The database is initialized by a file in the `migrations` folder using the following commands:

```
$ sqlx db create
$ sqlx migrate run
```

## Commands

Create an account in the "bank" with the given amount of money.

```
$ cargo run -- add NAME AMOUNT
```

List all the users and the amount of money they have.

```
$ cargo run -- list
```

Transfer the given amount of money between two people.

```
$ cargo run -- transfer AMOUNT FROM_NAME TO_NAME
```

Start transferring the given amount of money but call `panic!` in the middle of the transaction to simulate a crash in the middle of a transaction.

```
export PANIC=true
$ cargo run -- transfer AMOUNT FROM_NAME TO_NAME
```

## Testing

A separate file that will use a temporary database to test the application code.

