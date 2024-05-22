---
title: Embed image in newly created image using Rust
timestamp: 2024-05-12T09:50:01
author: szabgab
published: true
description: Create a new image, set its background, embed an image.
tags:
    - image
    - Rgba
    - RgbaImage
    - ImageBuffer
todo:
    - TODO
---

In this example we are going to create one image and embed another image into it.


## Dependencies

{% include file="examples/image/embed-image/Cargo.toml" %}

## Code

{% include file="examples/image/embed-image/src/main.rs" %}

## Imgae to be embedded

![](examples/image/embed-image/rust.png)


## Generated image

![](examples/image/embed-image/created.png)


