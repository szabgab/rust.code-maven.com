

A command line application written in Rust that stores a family tree in an SQLite database using the `sqlx` crate.
Each person has a name, a father, and a moder. For some people we don't know the maother and/or the father.


- Run `cargo run list` (or just `cargo run`) to list all known people.
- Run `cargo run add NAME` to add a person.
- Run `cargo run father-of CHILD FATHER` to set the father of a child.
  - Both names must already exist in the database.
- Run `cargo run mother-of CHILD MOTHER` to set the mother of a child.
  - Both names must already exist in the database.
