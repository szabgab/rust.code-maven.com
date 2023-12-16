---
title: Create empty image with the image crate of Rust
timestamp: 2023-10-01T17:30:01
published: true
description:
tags:
    - image
    - empty
---

In this example we create an empty image using the [image](https://crates.io/crates/image) crate of Rust.

This is the result:

![](examples/create-empty-image/empty.png)

Yes, this is just a black square.

```
cargo new create-empty-image
cd create-empty-image
cargo add image
```

Results in the following file:

![](examples/create-empty-image/Cargo.toml)

Then we add the code:

![](examples/create-empty-image/src/main.rs)

and run

```
cargo run
```
