---
title: Clap - Showing the description in the help using the about command
timestamp: 2024-02-10T10:30:01
author: szabgab
published: true
description: Using the about command we can include arbitrary text or the content of the description field in the help page.
tags:
    - Clap
    - about
    - description
    - help
todo:
    - embed sha
---


[Clap](/clap) provides a command called `about`. Using it we can add some extra text to the command line tool to be shown using the `--help` flag.

The text can be either embedded in the code or it can be taken from the `description` field of `Cargo.toml`.


## Taking the text from the description field

In `Cargo.toml` we need to depend on `clap` and we need to include the `derive` feature.

We also included a field called `description` in the `Cargo.toml` file.

![](examples/clap/about/Cargo.toml)

We add the **about** `command` to the `struct`:

![](examples/clap/about/src/main.rs)

Getting help:

```
$ cargo run -q -- --help

Hello Command Line Attribute Parser!

Usage: about --host <HOST>

Options:
      --host <HOST>
  -h, --help         Print help
```

In release:
```
$ cargo build --release

$ ./target/release/about --help
```


## Embedding the text in the about code

Although we added the `description` field to `Cargo.toml` in this example we are not going to use it.

![](examples/clap/about-expression/Cargo.toml)

We add the **about** `command` to the `struct` and we added some text after it.

![](examples/clap/about-expression/src/main.rs)

Getting help:

```
$ cargo run -q -- --help

Hello from the code

Usage: about --host <HOST>

Options:
      --host <HOST>
  -h, --help         Print help
```


