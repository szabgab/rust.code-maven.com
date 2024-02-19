---
title: home_dir, the home directory of the current user in Rust
timestamp: 2023-12-28T18:35:01
author: szabgab
published: true
description: Accessing the home_dir of the current user from Rust is easy, we just need a crate.
tags:
    - directories
    - BaseDirs
    - home_dir
---

{% include file="examples/hello.rs" %}

## Create a crate

Cleverly (or confusingly ?) call the crate `home-dir`.

```
cargo new home-dir
cd home-dir
```

## Add directories crate as a dependency


The [directories](https://crates.io/crates/directories) crate seems to offer what we need here.


```
cargo add directories
```

This will update the `Cargo.toml` file:

{% include file="examples/home-dir/Cargo.toml" %}


## Source code

{% include file="examples/home-dir/src/main.rs" %}


## Running it:

```
cargo run


home_dir: "/home/gabor"
```

Indeed that's my home directory on this computer.

