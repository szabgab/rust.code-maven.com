---
title: Create empty image with the image crate of Rust
timestamp: 2023-10-01T17:30:01
description:
tags:
    - image
    - empty
---

In this example we create an empty image using the [image](https://crates.io/crates/image) crate of Rust.

This is the result:

![](examples/crate-empty-image/empty.png)


```
cargo new crate-empty-image
cd crate-empty-image
cargo add image
```

Results in the following file:

![](examples/crate-empty-image/Cargo.toml)

Then we add the code:

![](examples/crate-empty-image/src/main.rs)

and run

```
cargo run
```
