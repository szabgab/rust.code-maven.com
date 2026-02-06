---
title: Embed time of compilation and other build-time information in a rust binary
timestamp: 2025-02-06T09:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - build
---

Sometimes it is useful to be able to show information about the compilation of a rust crate.

For example the date and time when it was compiled.

## Cargo.toml

We need to add the [built](https://crates.io/crates/built) crate as a build-time dependency.

{% include file="examples/embed-build-time-data/Cargo.toml" %}

## build.rs

We need to create `build.rs` in the root of the crate.

{% include file="examples/embed-build-time-data/build.rs" %}

## main.rs

We can then access the data in our code during run-time.

{% include file="examples/embed-build-time-data/src/main.rs" %}


## Output

```
$cargo run

TARGET                "x86_64-unknown-linux-gnu"
PKG_VERSION           "0.1.0"
RISTC_VERSION         "rustc 1.93.0 (254b59607 2026-01-19)"
GIT_VERSION           Some("9c6447a")
GIT_DIRTY             Some(true)
GIT_COMMIT_HASH       Some("9c6447a01046142ccf471a597f21d2c08c67c2cd")
GIT_COMMIT_HASH_SHORT Some("9c6447a")
BUILT_TIME_UTC        "Fri, 6 Feb 2026 10:42:50 +0000"
CI_PLATFORM           None
```
