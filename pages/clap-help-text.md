---
title: Clap - Add help text for each command line parameter in Rust
timestamp: 2023-12-27T10:32:01
published: true
description: Extend the help message with detailed explanation for each parameter.
tags:
    - help
    - arg
---

Part of the series about [Clap, the Command Line Argument Parser of Rust](/clap)

As we already saw in the [first article in the series](/clap-simple) Clap automatically provides a default help that can be shown using either the `--help` or the `-h`
command line parameter. We can easily add extra explanation to each one of the flags by including the `help` attribute in the `arg`.

```rust
#[arg(long, default_value="127.0.0.1", help="The name of the host")]
host: String,
```

## Dependencies in the Cargo.toml file

![](examples/clap/help-text/Cargo.toml)


## The full example

![](examples/clap/help-text/src/main.rs)

## Showing the help


```
$ cargo run -q -- -h
Usage: help-text [OPTIONS]

Options:
      --host <HOST>  The name of the host [default: 127.0.0.1]
  -h, --help         Print help
```
