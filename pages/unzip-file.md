---
title: Unzip a file to the disk using Rust
timestamp: 2024-05-22T11:10:01
author: szabgab
published: true
description: A tar.gz file can be unzipped and untarred with a few lines of Rust code.
tags:
    - tar
    - zip
    - unzip
    - gzip
    - flate2
    - compress
---


[flate2](https://crates.io/crates/flate2) is a crate to compress and uncompress files using various compression algorithms.


[tar](https://crates.io/crates/tar) is a crate to create tar files (aka. tarballs).

Combining the two could unzip and untar `tar.gz` file.

Note, we use the [read::GzDecoder](https://docs.rs/flate2/latest/flate2/read/struct.GzDecoder.html) of flate2.

In another example we [unzip a file that was embedded in the binary](/unzip-file-embedded-in-code) where we use another
version of this struct.

## Dependencies

{% include file="examples/unzip-file/Cargo.toml" %}

## Full example unzipping a file to the disk

{% include file="examples/unzip-file/src/main.rs" %}




