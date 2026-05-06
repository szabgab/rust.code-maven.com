A command line application that handles multiple counters in an SQLite database (`counter.db`) using the `sqlx` crate.

- Run `cargo run` to list all counters as `name number`.
- Run `cargo run NAME` to increment the counter `NAME` by 1.
	- If `NAME` does not exist yet, it is created with value `1`.

