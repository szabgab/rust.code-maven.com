---
title: Embed list of values in Rust application
timestamp: 2023-11-07T17:15:01
description: Make it easy for non-progammers to edit a list of values and then embed it in the program for easy distribution.
tags:
    - const
    - include!
    - concat!
    - env!
    - OUT_DIR
    - build
---

In the previous example we saw how to [embed a simple string](/embedding-text-file) in our code that is stored in an external file.

What if the data we would like to maintain is more complex? What if it is a list of strings? For example a list of words to be used in a Wordle game?

What if our data looks like this:

![](examples/crate-build/data/animals.txt)

We still would like to maintain this data in a separate, code-less file to make it easy for non-programmers to edit without any fear of breaking the code or even seeing code.

## Embedding

We could embed it as a string and then when the program starts we could split it up into a vector, but that would mean we store the data both in the
code and then in the heap. A waste of memory.

I found a much better solution in the [names](https://crates.io/crates/names) crated by [Fletcher Nichol](https://github.com/fnichol).

It uses a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) to convert the text file into a rust file that will be
converted to an `rs`-file during the build process. Then it is loaded in a const
It uses the `OUT_DIR` from the [environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html) available during build.

The way this work is that when `cargo` builds the executable of the crate, the first step is to compile and run the content of the `build.rs` file located
in the root of the project.

## The build script

![](examples/crate-build/build.rs)

This takes the `animals.txt` file from the `data` folder of the project and converts it into a file called `animals.rs` in the `OUT_DIR`.

When I executed `cargo run` it was `./target/debug/build/crate-build-29933f5e206e748f/out/animals.rs`.

When I executed `cargo build --releases` it was in `./target/release/build/crate-build-185456e02760ac01/out/animals.rs`.

(crate-build is the name of the crate I am using for this examples)

The `animals.rs` file looked like this:

```
[
"cow",
"pig",
"cat",
"dog",
]
```

## The embedding

![](examples/crate-build/src/main.rs)

We used the following macros:

* [include!](https://doc.rust-lang.org/std/macro.include.html)
* [concat!](https://doc.rust-lang.org/std/macro.concat.html)
* [env!](https://doc.rust-lang.org/std/macro.env.html)

Running this code as `cargo run` we get the following output:

```
["cow", "pig", "cat", "dog"]
cow
pig
cat
dog
```

We can also build the executable with `cargo build --release` then we can move the generated executable (`target/release/crate-build` in our case)
to any other place and run it. We'll get the same results.




