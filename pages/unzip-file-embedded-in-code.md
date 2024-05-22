---
title: Unzip a file that was embedded in the Rust application
timestamp: 2024-05-22T12:00:01
author: szabgab
published: true
description: A tar.gz file can be embedded in the code during build and unzipped and untarred with a few lines of Rust code to the disk.
tags:
    - tar
    - zip
    - unzip
    - gzip
    - flate2
    - include_bytes
---

The [rocket-starter](https://crates.io/crates/rocket-starter) crate includes several sets of files, each one is a skeleton to start writing a [Rocket](/rocket)-based
web application in Rust.

In another article we saw how to [extract the content of a tar.gz file on the disk](/unzip-file) and it turns out the code is almost exactly the
same in this case as well.

The difference is that we use the [bufread::GzDecoder](https://docs.rs/flate2/latest/flate2/bufread/struct.GzDecoder.html) instead of the [read::GzDecoder](https://docs.rs/flate2/latest/flate2/read/struct.GzDecoder.html)


There is a file called `example.tar.gz` in the root of the crate.

In order to embed that tar.gz file in the executable binary we use the [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html) macro.

```rust
let zipped = include_bytes!("../example.tar.gz");
```


We use the [flate2](https://crates.io/crates/flate2) to uncompress the stored bytes and the the [tar](https://crates.io/crates/tar) crate to separate it into individual files.

## Dependencies

{% include file="examples/unzip-from-memory/Cargo.toml" %}

## Full example unzipping a file to the disk

{% include file="examples/unzip-from-memory/src/main.rs" %}




