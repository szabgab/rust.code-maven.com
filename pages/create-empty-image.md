---
title: Create empty image with the image crate of Rust
timestamp: 2023-10-01T17:30:01
author: szabgab
published: true
description: Using the image crate of Rustlang it is very simple to create an image and save it in any format you like.
tags:
    - image
    - empty
    - RgbImage
    - ImageBuffer
    - save
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

{% include file="examples/create-empty-image/Cargo.toml" %}

Then we add the code:

{% include file="examples/create-empty-image/src/main.rs" %}

and run

```
cargo run
```
