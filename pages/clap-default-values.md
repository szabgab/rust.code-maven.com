---
title: Clap - default values for CLI command line parameters in Rust
timestamp: 2023-12-27T08:35:01
author: szabgab
published: true
description: Command line parameters are usually optional and have default values. This is how it is done in Rust using Clap.
tags:
    - Rust
    - Clap
    - default_value
    - default_value_t
---

Part of the series about [clap](/clap).

For most of the command line applications, most of the flags are optional and have default values.
In this example we'll see how to define the default values for string, integer, floating point number, boolean an even PathBuf command line parameters.

## The dependencies in Cargo.toml

![](examples/clap/default-values/Cargo.toml)


## The full code

![](examples/clap/default-values/src/main.rs)

Each parameter can have a default value using an attribute of the arguments.

`default_value` is used for strings and PathBuf

`default_value_t` is used for numbers and boolean values.

There are several other default types and there are more arg attributes.

See the full list of [arg attributes](https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes)

## Usage

As we set a default value to each one of the arguments we can run the program without providing any information on the command line. Each argument will have its default value.

```
$ cargo run
host:  127.0.0.1
port   5000
small: 0
float: 0
debug: false
path:  .
```

We can also pass some or all of the arguments on the command line:

```
$ cargo run -- --host localhost --port 3000

host:  localhost
port   3000
small: 0
float: 0
debug: false
path:  .
```


## Help showing the default values

When using the `--help` flag that was added by Clap automatically we'll see all the accepted parameter names along with the default values. Very neat.

```
$ cargo run -- --help

Usage: default-values [OPTIONS]

Options:
      --host <HOST>    [default: 127.0.0.1]
      --port <PORT>    [default: 5000]
      --small <SMALL>  [default: 0]
      --float <FLOAT>  [default: 0]
      --debug
      --path <PATH>    [default: .]
  -h, --help           Print help
  -V, --version        Print version
```

It is unclear to me why do we not see `[default: false]` for the `--debug` flag. As explained in this [issue](https://github.com/clap-rs/clap/issues/5270), flags
don't have a user-facing meaning (false/true) so showing the default would be meaningless.


