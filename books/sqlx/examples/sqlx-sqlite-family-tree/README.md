# sqlx-sqlite-family-tree

## Overview

Rust CLI that stores people and parent relationships in an SQLite database using `sqlx` with foreign keys and joins.

## What This Example Demonstrates

- Schema creation with foreign key references.
- Inserting people with uniqueness constraints.
- Updating parent relationships with validation.
- Listing relational data using SQL joins.

## Run

- `cargo run`
- `cargo run -- list`
  - Lists all known people and their parents.
- `cargo run -- add Alice`
  - Adds a person (or keeps existing person unchanged).
- `cargo run -- father-of Bob John`
- `cargo run -- mother-of Bob Mary`
  - For parent commands, both child and parent names must already exist.

## Configuration

- Default database URL: `sqlite://family-tree.db`
- Optional override: `DATABASE_URL=sqlite://my-family.db cargo run -- list`

## Copilot Usage Hint

When adding new commands, follow the current argument-dispatch style in `run(args, database_url)` and keep usage text synchronized with accepted command shapes.
