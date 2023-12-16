---
title: Crop image using Rust
timestamp: 2023-12-05T16:30:01
published: true
description: Cropping is the act of cutting out a part of an image while keeping the quality of the left-over image.
tags:
    - image
    - open
    - height
    - width
    - crop
    - crop_imm
    - save
---

Just as in the example where we were [resizing an image](/resize-image), here too we use the [image](https://crates.io/crates/image) crate.

## Create the crate

Let's create a new crate called `crop-image` and add the `image` crate as a dependency:


```
cargo new crop-image
cd crop-image
cargo add image
```

Or manually update `Cargo.toml`:

```
[dependencies]
image = "0.24"
```

## How to use this command?

```
cargo run some_image.jpg name_of_the_cropped_image.png 200 200 100 100
```

Running `cargo run` will show you the usage message that will include the expected parameters.


```
INFILE OUTFILE X Y WIDTH HEIGHT
```

From the original image we will retain the image that is located at the (x,y) of the original image, using the top left corner as (0, 0)
and has a width and hight as provided on the command line. So if we have an image that is 800 pixels wide and 600 high then

* `0 0 400 300` will create an image from the top left quarter of the original images.
* `400 300 400 300` will create an image from the bottom right quarter of the original image.
* `200 150 400 300` will create an image from the middle of the original image.
* `0 0 800 100` will create an image from the top 100 pixels of the original image.



## About the code

We have a function called `get_args` that will get the arguments from the command line and would print a usage-message if the user has not supplied
the correct number of arguments.


![](examples/crop-image/src/main.rs)

The [image::open](https://docs.rs/image/latest/image/fn.open.html) function will read the image into memory,
[crop_imm](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.crop_imm) will return a cropped version of the image.
That is "crop immutable". There is also a function called [crop](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.crop),
but that required the `img` to be **mutable**.

The [save](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.save) method will save the new image in format appropriate to the file extension we gave to it.



## Conclusion

It is quite easy to crop an image. Let's see what else can we do easily.


