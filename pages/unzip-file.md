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


## Dependencies

{% include file="examples/unzip-file/Cargo.toml" %}

## Full example unzipping a file to the disk

{% include file="examples/unzip-file/src/main.rs" %}




