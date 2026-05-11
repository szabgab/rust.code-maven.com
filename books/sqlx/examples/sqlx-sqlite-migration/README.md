# sqlx-sqlite-migration

## Overview

Rust CLI that imports student grades from `grades.csv` into SQLite and applies SQLx migrations.

## What This Example Demonstrates

- Reading and validating CSV-like input.
- Converting missing values (`-`) into nullable database fields.
- Running `sqlx::migrate!` from a local `migrations` directory.
- Upserting records with `ON CONFLICT(student) DO UPDATE`.

## Run

- `cargo run`
  - Reads `grades.csv`, runs migrations, and loads data into `grades.db`.

## Files

- `grades.csv`: input data source.
- `migrations/`: schema migration files.
- `grades.db`: generated SQLite database file.

## Copilot Usage Hint

If you add new columns to `grades.csv`, update all three places together: `GradeRecord`, CSV parsing logic, and the SQL `INSERT ... ON CONFLICT` statement.
