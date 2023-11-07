---
title: Embedding string, embedding text file
timestamp: 2023-11-07T12:30:01
description:
tags:
    - include_str!
    - &str
---

One of the first things we see in Rust that we can create a variable with some fixed, hard-coded string.
This string will be part of the compiled code of our application and thus we cannot change it at run-time.

![](examples/embedded-string/src/main.rs)

Sometimes it would be much better if we could store this string in an external file, and then baked into the executable.
For example this could be good to allow non-programmers to edit it without any fear of breaking the code.
Especially if it is a longer string.

It can be easily solved using he [include_str!](https://doc.rust-lang.org/std/macro.include_str.html) macro.

It will include the content of a text file relative to the `.rs` file. In our case we put `text.txt` in the root
of the project, next to the Cargo.toml file and the included it:

![](examples/embed-text-file/text.txt)

![](examples/embed-text-file/src/main.rs)


## How to verify


Compile the executable by:

```
cargo build --release
```

Because in the example the name of the crate was `embed-text-file`, this command created a file called `target/release/embed-text-file`.

I moved it to another folder and ran it from there.

It printed "Hello World!" as expected.



