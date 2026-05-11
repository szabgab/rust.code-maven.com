# SQLx SQLite Family Tree

{% embed include file="examples/sqlx-sqlite-family-tree/README.md" %}


Source code:

{% embed include file="examples/sqlx-sqlite-family-tree/src/main.rs" %}

This example uses `sqlx` and SQLite to store people and their parents.

Run these commands from `examples/sqlx-sqlite-family-tree`:

```shell
cargo run add Alice
cargo run add Bob
cargo run add Carol
cargo run father-of Alice Bob
cargo run mother-of Alice Carol
cargo run
```

Expected output:

```text
name | father | mother
---- | ------ | ------
Alice | Bob | Carol
Bob | - | -
Carol | - | -
```