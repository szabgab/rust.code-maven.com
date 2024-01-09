---
title: Avoid unwrap
timestamp: 2024-01-09T08:40:01
published: true
description: unwrap can be useful, but if you want to make sure your code does not have it, then  you can ask Clippy to help.
tags:
    - unwrap
    - Clippy
todo:
    - create a page /clippy
---

You might have seen a lot of code using [unwrap](/unwrap) and even more people telling you not to use `unwrap` in production code.

[unwrap](/unwrap) is one of the easy ways to disregard errors in Rust. It might be nice in examples so the reader can focus on the rest of the
code, it might be good in **tests** where we want to have a panic if something goes wrong.

However, in general it is not that good to have in the code.

Luckily [Clippy](https://github.com/rust-lang/rust-clippy) can help us.


## Some simple code that uses unwrap

![](examples/avoid-unwrap/src/main.rs)

This code works.


## Configure clippy to avoid unwrap

![](examples/avoid-unwrap/Cargo.toml)

By adding 2 lines to the `Cargo.toml` file we can tell Clippy to report an error when `unwrap` is encountered in the code.

```
[lints.clippy]
unwrap_used = "deny"
```

When I ran `cargo clippy` this is the error message I received:

```
$ cargo clippy
    Checking avoid-unwrap v0.1.0 (/home/gabor/work/rust.code-maven.com/examples/avoid-unwrap)
error: used `unwrap()` on a `Result` value
 --> src/main.rs:3:18
  |
3 |     let answer = input.parse::<u32>().unwrap();
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: if this value is an `Err`, it will panic
  = help: consider using `expect()` to provide a better panic message
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unwrap_used
  = note: requested on the command line with `-D clippy::unwrap-used`

error: could not compile `avoid-unwrap` (bin "avoid-unwrap") due to previous error
```

Referring to [unwrap used](https://rust-lang.github.io/rust-clippy/master/index.html#unwrap_used).


