---
title: Clap - show version number of the command line application in Rust
timestamp: 2023-12-25T12:30:01
author: szabgab
published: true
description: It is quite easy with clap to include the version number of the command-line tool.
tags:
    - clap
    - command
    - version
    - "--version"
    - "-V"
todo:
    - include additional information in the output of --version, just as we have in rustup
---

It is usually quite useful for command line applications to be able to tell which version they are.

The common way is that they have a command line flag `--version` and sometimes `-V` does the same.

Just try

```
rustup --version

rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.74.1 (a28077b28 2023-12-04)`
```

or

```
rustup -V
```


It is very easy to do this using [Clap](/clap), we only need to add the following to the `struct`
that defines the parameters:


```rust
#[command(version)]
```

Clap will automatically add a `--version` and the `-V` flags to the application and when the user runs the cli tool providing
either of those flags it will print the name of the cli tool and the version number taken from `Cargo.toml`.

Both values coming from the `Cargo.toml` file.

## Cargo.toml

To demonstrate I created a Crate called `show-version` and added the [Clap](https://crates.io/crates/clap) crate as a dependency
including the `derive` feature:

```
cargo add clap --features derive
```


The `Cargo.toml` looks like this:

{% include file="examples/clap/show-version/Cargo.toml" %}


## Full Code

{% include file="examples/clap/show-version/src/main.rs" %}

```
cargo run -- --version
```

```
show-version 1.2.3
```

As we saw in the article about [getting started with command line parsing in Rust](/clap-simple) we can also run the compiled code directly and then we don't need the `--` separator:

```
$ ./target/debug/show-version -V
show-version 1.2.3
```

If we move and rename that executable:

```
mv ./target/debug/show-version qqrq
./qqrq -V
show-version 1.2.3
```

So this **always** uses the name that was defined in `Cargo.toml` unlike the way we printed the **usage message** in
[ARGV - command line parameters for simple CLI program](/argv-simple-command-line-parameters) where we always print the name of the current executable.

## Notes

Actually you can [embed the version in the binary](/embed-version-number-in-the-code)  without Clap as well.


