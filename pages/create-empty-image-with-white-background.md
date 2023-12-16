---
title: Create empty image with white background and black borders using Rust
timestamp: 2023-10-02T10:00:01
published: true
description: The default background is black. We can set the background to any color. This case we set it to white.
tags:
    - image
---

When we create an image the default color of each pixel is black as you can see in the [empty image example](/create-empty-image).
In this example we just set all the pixels to be white. We could of course set them to any color we would like expressed in RGB - red-green-blue triplets.

After setting all the pixel to white I realized that you won't be able to see the white image on the white background of this page, so I also added two
more loops to set the borders of the image to black.

## The source code

![](examples/create-white-image/src/main.rs)

## The result

![](examples/create-white-image/white.png)


## The setup

![](examples/create-white-image/Cargo.toml)

