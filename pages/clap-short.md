---
title: Clap - short command line parameters
timestamp: 2023-12-27T10:15:01
author: szabgab
published: true
description: Short command line flags make it easier to type in commonly used parameters.
tags:
    - arg
    - short
---

This is part of the series about [Clap](/clap), the Command Line Argument Parser of Rust.

Having **long** command line parameter names is good, because they are usually easier to remember than the short ones, and they are clearer once you wrote them down,
but they also take a lot longer to type in. So often, besides having a flag called `--debug` we would also like to allow the user to just write `-d`.
In addition of `--help` Clap allows us to use the `-h` flag and get help. When we use the [version command](/clap-show-version-number) we automatically get two flags
the `--version` and the `-V` flag. Apparently for some reason the "short name for the lower-case version is the upper-case V.

In order to allow short parameter name(s) we only have to supply the `short` keyword in the `arg` definition of each parameter where we want to have such a flag:

## Short parameter names

```rust
#[arg(short)]
debug: bool,
```

This will allow the user to use `-d`.


```rust
#[arg(long, short)]
debug: bool,
```

This will allow the user to use either `-d` or `--debug`.


```
#[arg(long, short)]
host: String,
```

This will lead to a runtime `panic!`:

```
Command short: Short option names must be unique for each argument, but '-h' is in use by both 'host' and 'help' (call `cmd.disable_help_flag(true)` to remove the auto-generated `--help`)
```

In addition to the solution suggested in this message we can also set the short letter explicitly:

```
#[arg(long, short='H')]
host: String,
```

As explained in this [issue](https://github.com/clap-rs/clap/issues/5271) there is no plan to include this suggestion in the panic message.


## Dependencies

As in the other [articles about Clap](/clap) I include the content of `Cargo.toml` to make it easier to copy this example.

{% include file="examples/clap/short/Cargo.toml" %}

## The full example

{% include file="examples/clap/short/src/main.rs" %}

In this example I included the [version command](/clap-show-version-number), `long` parameters, and [default values](/clap-default-values) as well.


## Usage with defaults:

```
$ cargo run -q

debug:   false
host:    127.0.0.1
verbose: false
```

## Setting some parameters

```
$ cargo run -q -- -v -H localhost

debug:   false
host:    localhost
verbose: true
```

Here we can see that we can use the `-H` short name for `--host` and we can use `-v` for verbose. Maybe the reason the short of `--version` is the capital `-V` is to allow `-v` to be short of `verbose`.

## Combining short flags

```
$ cargo run -q -- -dvH localhost

debug:   true
host:    localhost
verbose: true
```

We can even combine the short `bool` flags and the last one of the combination can also be a flag the expects a value.



