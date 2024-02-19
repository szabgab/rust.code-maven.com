---
title: Use shadow-rs to embed sha1 and other build-time information in the code
timestamp: 2024-02-12T13:20:01
author: szabgab
published: true
description: When a user reports an error it is crucial to know which version of the application they use.
tags:
    - shadow-rs
    - version
    - sha1
    - build
todo:
    - TODO
---

When a user reports an error it is crucial to know which version of the application they use. The [shadow-rs](https://crates.io/crates/shadow-rs) crates makes it very easy to embed the version number found in `Cargo.toml`
along with a lot of additional build-time information. For example the exact sha1 from the most recent commit in git, and a plethora of other pieces of data.

For this example I basically followed the README of the crate

## Dependencies in Cargo.toml

{% include file="examples/use-shadow/Cargo.toml" %}

I've included

```
build = "build.rs"
```

in the Cargo.toml, despite the fact that it seems [it is not necessary](https://github.com/baoyachi/shadow-rs/issues/150).

## Build code

{% include file="examples/use-shadow/build.rs" %}

## Code

In the README of the Crate there are a lot more variables with a lot more details, but I was mainly interested in these:

{% include file="examples/use-shadow/src/main.rs" %}

## Build and run

This is how you build a release from your crate:

```
cargo buld --release
```

Then I moved the generated binary to some other location:

```
mv target/release/use-shadow ~/
```

Then I ran the program

```
cd ~
./use-shadow
```


```
$ ./use-shadow
branch:  main
version: 0.1.0
sha1:    9b773fe2
rust:    1.76.0
```

## Real world usage

I a real world application I'd probably include the version number in the output of the `--version` flag, maybe along with the sha1.

Then I'd add another flag, for example `-V` that would display a lot more details.


## Why do I need the sha1 ?

If I already have the version number, why do I need the sha1?

In at least one project every time I push to GitHub, it runs some tests and then generates a released binary. It won't make sense to change the version number in `Cargo.toml` for every commit
so we will have multiple releases with the same version number. In this case it is quite important to know which sha1 it was built from.



