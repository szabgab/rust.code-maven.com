---
title: Create image with text written on it using Rust
timestamp: 2023-10-02T11:30:01
author: szabgab
published: true
description: In many cases it is not enought to be able to draw nice images. We would also want to write some text on the image.
tags:
    - image
    - draw_text_mut
    - text_size
    - Font
    - Scale
    - Rgb
    - RgbImage
    - get_pixel_mut
todo:
    - include the font at run-time?
---

In this example we create a new image with white background and the we write some text on it centered both horizontally and vertically.

This is the result:

![](examples/create-image-with-text/image.png)

## Find the fonts on your computer

First we need to find and select a font that we have on our system:


For example on Linux or macOS you can use either the `locate` command:

```
locate "*.ttf"
```

or the `find` command

```
find / -name "*.ttf" 2> /dev/null
```

to find True Type fonts.

On Windows I guess you'd use the File-explorer to find font files.

We could include the font where it is on our system, but it is probably a good idea to copy the file containing the selected font to the folder of the crate so they will be available on any system.


## Crates

We are using the following crates:

* [image](https://crates.io/crates/image)
* [imageproc](https://crates.io/crates/imageproc)
* [rusttype](https://crates.io/crates/rusttype)

![](examples/create-image-with-text/Cargo.toml)

## Source code

![](examples/create-image-with-text/src/main.rs)

* First we expect and accept two parameters on the command line. The name of the image file and the text we would like to write on the image.

* Then we create the image and set all the pixels to be white.

* Then we create the font using the [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html) macro that embeds the content of the font file in the binary `rustc` compiles.

* Then we calculate the scale of the text and using the `text_size` function we compute the real size of the text taking into account the font that we'll use.

* Based on these and the dimension of the image we calculate the x,y point where the text should start in order to be in the center of the image.

* We then draw the text using the `draw_text_mut` function.

* Finally we save the image to the file.

## Running on the command line during development

We provide the name of the file we would like to create and the text we would like to write on the image.

```
cargo run -- image.png "Welcome to Rust"
```

## Running on the command line

We can now create a stand-alone binary from our code:

```
cargo build --relase
```

Then we can distribute the file from `target/release/create-image-with-text`.

The users will be able to run it as

```
create-image-with-text image.png "Hello world!"
```

The font is already embedded in the binary so we don't need to worry about that.



