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

{% include file="examples/clap/about/Cargo.toml" %}

We add the **about** `command` to the `struct`:

{% include file="examples/clap/about/src/main.rs" %}

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

{% include file="examples/clap/about-expression/Cargo.toml" %}

We add the **about** `command` to the `struct` and we added some text after it.

{% include file="examples/clap/about-expression/src/main.rs" %}

Getting help:

```
$ cargo run -q -- --help

Hello from the code

Usage: about --host <HOST>

Options:
      --host <HOST>
  -h, --help         Print help
```

## Generate about text at run-time

In this example we will see how to generate the **about** text displayed by the `--help` flag while the program is running.

The dependency is the same as before:

{% include file="examples/clap/about-generated-expression/Cargo.toml" %}

As the text is generated at run-time I wanted to include something that will change on every run, so the about text includes the
[time elapsed since the epcoh](/time-elapsed-since-epoch) using only the standard library.


In the **about** command we include a call to the function that will generate the text. The function itself can of course have any name,
it needs to return a `String`.

{% include file="examples/clap/about-generated-expression/src/main.rs" %}

If we build a release version and then we run the command with the `--help` flag multiple times, we'll see the time at the top of the response changes.

```
$ cargo build --release

$ ./target/release/about --help

1707555211863921199

Usage: about --host <HOST>

Options:
      --host <HOST>  
  -h, --help         Print help
```
