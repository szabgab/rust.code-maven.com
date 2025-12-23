# Test coverage with Tarpaulin

[cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) is a code coverage reporting tool for the Cargo build system, named for a waterproof cloth used to cover cargo on a ship.


## Install Tarpaulin

```
cargo install cargo-tarpaulin
```


## Run Tarpaulin generate HTML report

```
cargo tarpaulin --out Html
```

## Run Tarpaulin - print minimalistic text report on the screen

```
cargo tarpaulin --out Stdout
```

## Run Tarpaulin - both output

```
cargo tarpaulin --out Html --out Stdout
```

## Only run s subset of the tests

For example, I have a project where I have a number of tests files:

```
src/
├── collect.rs
├── db.rs
├── language.rs
├── main.rs
├── test_collect.rs
├── test_db.rs
├── test_utils.rs
└── test_web.rs
```

I could get a coverage report of the database tests by running the following:

```
cargo tarpaulin --out Html  --out Stdout -- test_db
```

## ASLR disable failed: EPERM: Operation not permitted

When running inside a Docker container you might get the above error message.

Using the Llvm engine solved this problem for me.

```
--engine Llvm
```
