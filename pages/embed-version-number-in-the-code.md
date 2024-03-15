---
title: Embed version number in the binary compiled by Rust
timestamp: 2024-03-15T08:50:01
author: szabgab
published: true
description: We can bake the version number of the application in the code taking it from Cargo.toml.
tags:
    - version
    - CARGO_PKG_VERSION
    - env!
    - static
    - const
todo:
    - constant functions https://doc.rust-lang.org/std/keyword.const.html
---

There are a number of [environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html) available during the **build** of a binary.
One of them is the `CARGO_PKG_VERSION` variable that contains the version number as written in the `Cargo.toml` file.

Using the [env!](https://doc.rust-lang.org/std/macro.env.html) macro we can extract the value during the **build** phase of the compilation and we can include it
on our code either as a constant using the [const](https://doc.rust-lang.org/std/keyword.const.html) keyword or we can make it a regular variable either explicitely making
it a [static](https://doc.rust-lang.org/std/keyword.static.html) variable or letting Rust figure out the type.

The difference will be the location where the value is stored.

## The code

{% include file="examples/embed-version-number/src/main.rs" %}


## Cargo.toml with the version number

{% include file="examples/embed-version-number/Cargo.toml" %}

## Note

I'd go with the `const`.

If you only need to have a `--version` flag you can [tell Clap to create a --version flag](/clap-show-version-number).
