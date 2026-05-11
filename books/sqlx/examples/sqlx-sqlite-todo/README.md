# sqlx-sqlite-todo

## Overview

Rust CLI todo manager backed by SQLite using `sqlx` and `clap`.
Using subcommands and environment-based database configuration.

## What This Example Demonstrates

- Subcommand-based CLI (`add`, `done`, and default list).
- Automatic table initialization at startup.
- Insert, update, and query patterns with SQLx.
- Reading required configuration from environment variables.

## Run

- `DATABASE_FILE=todos.db cargo run`
  - Lists all todos.
- `DATABASE_FILE=todos.db cargo run -- add "buy milk"`
  - Adds a new todo item.
- `DATABASE_FILE=todos.db cargo run -- done 1`
  - Marks todo `1` as done.

## Configuration

- Required env var: `DATABASE_FILE`
- Example value: `todos.db` or `/tmp/my-todos.db`

## Copilot Usage Hint

Keep the command behavior centered in the `Command` enum and `match args.cmd` so new subcommands remain discoverable and testable.
