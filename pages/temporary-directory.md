---
title: Temporary directory in Rust
timestamp: 2023-12-30T19:20:01
published: true
description: Temporary directory
tags:
    - TempDir
todo:
    - when does the directory remain? Is it when it has content? Some other reason?
---

The [tempdir](https://crates.io/crates/tempdir) crate allows us to create a temporary directory.


## Dependencies

![](examples/temporary-folder/Cargo.toml)

## Code

![](examples/temporary-folder/src/main.rs)

In this example the directory is removed at when the variable goes out of scope.

