# SQLx SQLite Counter

{% embed include file="examples/sqlx-sqlite-counter/README.md" %}

Source code:

{% embed include file="examples/sqlx-sqlite-counter/src/main.rs" %}

This example uses `sqlx` and SQLite to store named counters.

Run these commands from `examples/sqlx-sqlite-counter`:

```shell
cargo run
cargo run -- visits
cargo run -- visits
cargo run -- pageviews
cargo run -- pageviews
cargo run -- pageviews
cargo run
```

Expected output:

```text
pageviews 3
visits 2
pageviews 3
visits 2
pageviews 3
```
