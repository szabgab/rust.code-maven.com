# SQLx SQLite Todo

{% embed include file="examples/sqlx-sqlite-todo/README.md" %}

Source code:

{% embed include file="examples/sqlx-sqlite-todo/src/main.rs" %}

This example uses `sqlx` and SQLite to store a simple todo list.

Run these commands from `examples/sqlx-sqlite-todo`:

```shell
DATABASE_FILE=todos.db cargo run
DATABASE_FILE=todos.db cargo run -- add "buy milk"
DATABASE_FILE=todos.db cargo run -- add "walk the dog"
DATABASE_FILE=todos.db cargo run -- done 1
DATABASE_FILE=todos.db cargo run
```

Expected output:

```text
Printing list of all todos
- [ ] 1: buy milk
- [x] 2: walk the dog
```

The `DATABASE_FILE` environment variable is required and specifies where the SQLite database file is stored. The application automatically creates the database and tables on first run.
