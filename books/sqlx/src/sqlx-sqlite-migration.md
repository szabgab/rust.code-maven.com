# SQLx SQLite Migration

{% embed include file="examples/sqlx-sqlite-migration/README.md" %}

Source code:

{% embed include file="examples/sqlx-sqlite-migration/src/main.rs" %}

This example uses `sqlx` to apply migrations from the `migrations/` directory and imports student grades from a CSV file into SQLite.

Run this command from `examples/sqlx-sqlite-migration`:

```shell
cargo run
```

The program will:
1. Read `grades.csv`
2. Apply any pending migrations from `migrations/`
3. Insert or update all grade records into `grades.db`

Expected output:

```text
Loaded 5 grade rows into grades.db
```

The resulting database can be inspected with SQLite tools, and the data is upserted so running the command again is safe.
