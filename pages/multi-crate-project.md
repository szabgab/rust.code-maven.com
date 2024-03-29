---
title: Rust Multi-crate project in a monorepo
timestamp: 2024-03-07T10:30:01
author: szabgab
published: true
description: the basics of creating a project that contains multiple crates in a single monorepo.
tags:
    - cargo
    - monorepo
---

As your project growth at one point your might feel that splitting the code into multiple crates could be a good idea. It might make the
code cleaner and more reusable across projects. You might even decide to publish some of these crates separately.

There are at least two ways to manage this. One is to put each crate in its own repository. It has the advantage of making them totally
independent, but it will create some extra overhead.

The alternative is to create [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html). Though the Rust documentation does not
mention this, this is similar to the idea of using **monorepo**.

Let's see how we go about doing this.

## Crate main crate

```
cargo new multi-crate-project
cd multi-crate-project
```

The `Cargo.toml` file looks like this:

```toml
[package]
name = "multi-crate-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

We edit the `src/main.rs` file to contain:

```rust
fn main() {
    println!("Hello, main");
}

#[test]
fn check_main() {
    assert_eq!(1, 1);
}
```

At this point we can `cargo run` or cargo test`. There is nothing special about it.

This is how the directory tree looks like:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```


## First sub-crate (a library)

Crate the folder `crates` in the root of the project.

```
mkdir crates
cd crates
cargo new first --lib
cd ..
```

This command created the `crates/first` folder and also changed the `Cargo.toml` in the root of the project adding the `workspace` entry:


```toml
workspace = { members = ["crates/first"] }
[package]
name = "multi-crate-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

In the `crates/first` folder there is a new `Cargo.toml` file with the following default content:


```toml
[package]
name = "first"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```


There is also the `crates/first/src/lib.rs` file that I changed to the following:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_works() {
        assert_eq!(1, 1);
    }
}
```

This was generated by the `cargo new` command, but I changed the test a little.

The directory structure looks like this:

```
.
├── Cargo.lock
├── Cargo.toml
├── crates
│   └── first
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
└── src
    └── main.rs
```

## Running tests

While being in the root of the project:

`cargo test` will run the tests of the main crate only.

`cargo test --workspace first` will run the tests of the `first` crate.

`cargo test --all` will run the tests of all the crates.

However, if one of the tests fails then this command will not run any more tests. It stops after the first failure.
we can change that by running

`cargo test --all --no-fail-fast`.

You can experiment with this by changing the `assert_eq!(1, 1)` to `assert_eq!(1, 2)` to make the test fail.


## alias

Typing that command every time is a bit boring. Luckily `cargo` can have a configuration file to set aliases
that come with the repository:
Create the `.cargo` folder and in there create the file `.cargo/config.toml` with the following content:


```toml
[alias]
t = "test --all --no-fail-fast"
```

This will let you type in `cargo t`.


## Create an executable sub-crate (bin)

While probably most of the internal crates will be libraries occasionally there might be an executable so I wanted to see how
that would work:

```
cd crates
cargo new second
cd ..
```

The directory tree now looks like this:

```
.
├── Cargo.lock
├── Cargo.toml
├── crates
│   ├── first
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── second
│       ├── Cargo.toml
│       └── src
│           └── main.rs
└── src
    └── main.rs
```


This command updated the main `Cargo.toml` file again, adding another entry to the list of workspaces:

```toml
workspace = { members = ["crates/first", "crates/second"] }
[package]
name = "multi-crate-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

It created a standard `crates/second/Crate.toml` file and a standard `crates/second/src/main.rs` file
that I changed a bit:

```rust
fn main() {
    println!("Hello, second");
}

#[test]
fn check_second() {
    assert_eq!(1, 1);
}
```

## Running the tests

The alias we created earlier works, so we can run all the tests of all the crates using:

```
cargo t
```

## Running the binary

`cargo run` will run the executable of the main crate.

`cargo run --package second` will run the executable of the crate called `second`.
You can create an alias for that if that makes life easier.

## Target folder

All the creates use the same `target` folder in the root of the project.

We can change the main `Cargo.toml` file to include all the crates in the `crates` folder.
We can also exclude some.

## Make Cargo.toml more generic

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[package]
name = "multi-crate-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

## Global configuration vs. local configuration

According to the [documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html) some things
can be configured globally in the main `Cargo.toml` file, but for example I had to add the Clippy lint
configuration to each one of the `Cargo.toml` files separately.

```toml
[lints.clippy]
unwrap_used = "deny"
```

## Source code

[source](https://github.com/szabgab/rust.code-maven.com/tree/main/examples/multi-crate-project)




