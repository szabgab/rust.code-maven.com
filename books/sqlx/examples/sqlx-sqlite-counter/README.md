# sqlx-sqlite-counter

## Overview

Small Rust CLI that stores named counters in SQLite using `sqlx`.

## What This Example Demonstrates

- Creating and connecting to an SQLite database with `sqlx`.
- Creating a table if it does not exist.
- Upsert-style updates (`INSERT ... ON CONFLICT ... DO UPDATE`).
- Basic CLI behavior from positional arguments.

## Run

- `cargo run`
	- Lists all counters as `name number`.
- `cargo run -- visits`
	- Increments `visits` by 1 (creates it with value `1` if missing).

## Configuration

- Default database URL: `sqlite://counter.db`
- Optional override: `DATABASE_URL=sqlite://my-counter.db cargo run -- visits`

## Copilot Usage Hint

When extending this example, keep the single-argument CLI shape: no argument lists all counters, one argument increments that counter.

