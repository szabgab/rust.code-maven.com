---
title: Resize image using Rust
timestamp: 2023-12-05T10:55:01
description: It is very easy to resize an image in Rust, we just need to decide on the sampling filter and the new size.
tags:
    - image
    - FilterType
    - open
    - height
    - width
    - resize
    - save
    - match
todo:
    - exclamation mark
---


## Create the crate

For this example we create a crate called `resize-image` and then add [image](https://crates.io/crates/image) as a dependency.

```
cargo new resize-image
cd resize-image
cargo add image
```

Or manually update `Cargo.toml`:

```
[dependencies]
image = "0.24"
```

## How to use this crate?

We run the command like this:

```
cargo run ~/Pictures/locust_5.jpg resized.png 400 nearest
```

* The 1st parameter is the path to the original file.
* The 2nd parameter is the path to the new file. As you can see, the file extension does not have to be the same. The code will automatically recognize the file format based on the file extension.
* The 3rd parameter is the new `width` of the image. The new `hight` will be calculated based on this and the aspect ratio of the original file.
* The 4th and last parameter is the [FilterType](https://docs.rs/image/latest/image/imageops/enum.FilterType.html) In the code I've created a mapping from a string to the actual type in that `enum`.

We need to provide this last parameter in order to tell the `resize` method how to do its magic. From the [sample image](https://docs.rs/image/latest/image/imageops/enum.FilterType.html)
I can see that the "nearest" method is 10-30 times faster than the others, but it also creates the worse image.

## The code

The `src/main.rs` file then looks like this:

![](examples/resize-image/src/main.rs)

We have two functions, the `get_args` function will look at the command line and return the 4 parameters. The first two will be simple strings.
The 3rd one is converted to a `u32` (unsigned integer with 32 bits). The 4th parameter is expected to be a string that is then converted
to a [FilterType](https://docs.rs/image/latest/image/imageops/enum.FilterType.html) the pattern matching implemented in the `get_filter_type` function.

The [image::open](https://docs.rs/image/latest/image/fn.open.html) function reads the content of the image file in the memory. It return
[ImageResult](https://docs.rs/image/latest/image/error/type.ImageResult.html) structure with a [DynamicImage](https://docs.rs/image/latest/image/enum.DynamicImage.html)
in it.

This has a [width](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.width) and a [height](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.height)
method, both returning `u32` values.


The aspect ratio of the original images is calculated by `img.height() / img.width()`.

Then we use the [resize](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.resize) method to do the actual work.

Finally [save](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.save) will save the new image in format appropriate to the file extension we gave to it.

## Resize image with error checking

In the previous solution I did not have any error checking. In several places I used `unwrap`. One of the improvements suggested was to add some input validation and error checking.
I did not want to make the original solution even more complex, so I created a separate one that contains error handling.
This code also allows the user to provide only 3 parameters and use `FilterType::Nearest` as the default.

![](examples/resize-image-with-error-checking/src/main.rs)

In most cases the change was simple replacing the `unwrap` call by the use of the `match`. One case, however was particularly interesting.

In the `get_args` function we have this code:

```rust
    let width: u32 = match args[3].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!(
                "Invalid parameter for width: '{}'. It must be an integer",
                err
            );
            usage(&args[0])
        }
    };
```

Here we expect both arms of the `match` to return `u32`, but the `Err` arm calls a function that always exits and thus it should not return anything.
At first I declared it this way, but Rust complained that the `Err` arm returns `()` and not the expected `u32`.

```rust
fn usage(name: &str) {
    eprintln!("Usage: {} INFILE OUTFILE WIDTH FILTER", name);
    std::process::exit(1);
}
```

The solution I found was adding an exclamation mark where we would have declared the return values:

```rust
fn usage(name: &str) -> ! {
    eprintln!("Usage: {} INFILE OUTFILE WIDTH FILTER", name);
    std::process::exit(1);
}
```

Apparently functions that never return are called [Diverging functions](/diverging-functions).

## Conclusion

It is quite easy to resize an image. Let's see what else can we do easily.

For example we can [crop and image](/crop-image).

