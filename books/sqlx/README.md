# SQLx Book

This repository contains a practical book and runnable examples for learning how to use SQLx with SQLite in Rust.

The focus is on small command-line programs that show database connection setup, schema creation and migration, inserts and updates, query patterns, and simple CLI design.

@src/who-is-this-for.md

See the @src/SUMMARY.md for a list of pages that include examples.


## How Copilot Can Use This Repository

Copilot can use these examples as reference implementations for:

- SQLite connection options and database file handling.
- Table initialization and migrations.
- Typical CRUD-style query workflows.
- Argument parsing and command dispatch in Rust CLIs.
- Testable program structure for small SQLx apps.

## Suggested Copilot Prompts

- "Add a delete command to the todo example and include tests."
- "Extend the family tree example with a siblings command."
- "Add input validation and better error messages to the counter app."
- "Add a new column to grades and update migration plus import logic."

