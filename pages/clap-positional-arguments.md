---
title: Clap - positional command line arguments in Rust
timestamp: 2023-12-27T12:46:01
published: true
description: Positional command line arguments do not have a named prefix starting with a dash.
tags:
    - arg
    - positional
    - required
    - default_value
---

Part of the [Clap](/clap) series.

In most of the examples we used named parameters where the name has one or two dashes. Those are the most common cases for command line parameters, but
sometimes, usually in simpler case we can use positional arguments as well.

If we only have one or one type of positional arguments, then we might be better off using [std::env::args](/argv-simple-command-line-parameters) directly,
but as we might want to combine positional arguments with named arguments, let's see how to handle these in Clap.

In order to have a positional parameter we don't need to provide the `arg`, we only need to define a struct and `derive` it from the `clap::Parser`:

```rust
#[derive(Parser)]
struct Cli {
    host: String,
}
```

In most cases positional arguments are required.

Nevertheless we can also defined a `default_value` in the same manner as we had [default values for named command line parameters](/clap-default-values), but I think
this would be rather rare.

```rust
#[derive(Parser)]
struct Cli {
    #[arg(default_value = "127.0.0.1")]
    host: String,
}
```


## Accept multiple values

We can **accept multiple values** for the last positional argument, by defining it as a **vector**:

```rust
#[derive(Parser)]
struct Cli {
    host: String,

    ports: Vec<u16>,
}
```

In this case we can run the program like this:

```
$ cargo run -q 127.0.0.1 3000 5000

host:  127.0.0.1
ports: [3000, 5000]
```

but also without providing any value for the argument that will be stored in a vector:

```
$ cargo run -q 127.0.0.1
host:  127.0.0.1
ports: []
```

## Required

If we would like to make sure that the user supplies at least one value for the `ports` argument in our example,
we can mark it as `required`.

```
#[derive(Parser)]
struct Cli {
    host: String,

    #[arg(required = true)]
    ports: Vec<u16>,
}

## Positional and named arguments

Finally, we can also mix the positional and the named arguments. Here `test` is a named argument.

```rust
#[derive(Parser)]
struct Cli {
    host: String,

    #[arg(required = true)]
    ports: Vec<u16>,

    #[arg(long)]
    test: String,
}
```

This is how we can use them:

```
$ cargo run -q 127.0.0.1 1000 --test acceptance

test:  acceptance
host:  127.0.0.1
ports: [1000]
```
## Dependencies in Cargo.toml

![](examples/clap/positional/Cargo.toml)


## The full example

![](examples/clap/positional/src/main.rs)


