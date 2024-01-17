---
title: Command line counter with plain text file storage in Rust
timestamp: 2023-12-28T18:30:01
author: szabgab
published: true
description: Every time we run this program on the command line it gives a number one higher than previously.
tags:
    - read_to_string
    - trim_end
    - parse
    - create
    - writeln!
    - directories
    - BaseDirs
    - home_dir
---

As part of the [Counter example](https://code-maven.com/counter) this program will show a series of ever increasing numbers every time  we run it.

```
$ cargo run
0
$ cargo run
1
$ cargo run
2
```

We use a file called "counter.txt"

We use the fact that an `if`-statement will return the last expression of the `if` or the `else` part.
If the file exists, we read the content using `std::fs::read_to_string`, remove the trailing newline with `trim_end`
and then convert it to an `i32` number using the `parse` method.

In-between we had to call `unwrap` twice to pick the result and `panic!` if there was an error.
As this is not some important application I thought I'll leave error handling out.

If the "counter.txt" file does not exist, the `else`-part returns 0.

Then we increment the counter.

Save it to the disk and print it.

![](examples/counter-text-file/src/main.rs)

One of the big issues of this program, besides not handling errors, is that the "counter.txt" file is relative to the current working directory of when the code is executed.
This is fine as long as we run it with `cargo run`, but if we create a binary and distribute it, it would be probably better if the "counter.txt" file was in a fixed place
on the computer, or at least fixed per user.

## Home directory

In order to solve this we need a way to find the home directory of the current user. The `home_dir` in the standard library is deprecated,
but I found the [directories crate](https://crates.io/crates/directories) to be useful and quite popular.

In order to use it we need to to add it as a dependency.

First we create a new crate:

```
cargo new counter-text-file-in-homedir
cd counter-text-file-in-homedir
```

Then we add the crate:

```
cargo add directories
```

This will add a line to the `Cargo.toml` file listing this dependency.

![](examples/counter-text-file-in-homedir/Cargo.toml)

Then we replace the code declaring the path with these two lines:

```rust
let bd = directories::BaseDirs::new().unwrap();
let path = bd.home_dir().join("counter.txt");
```

In this way "counter.txt" will be in the home directory of the current user.

We could use `cargo run`, but we would like to make sure that this code works regardless where the executable is located and what is our current working directory.
So we compile the code in release mode:

```
cargo build --release
```

This will create the file `target/release/counter-text-file-in-homedir`.
We can move it to any folder. (e.g. I have a folder called `~/work` where I have all of the projects I work on. So:

```
mv target/release/counter-text-file-in-homedir ~/work/
```

Then I can run `~/work/counter-text-file-in-homedir` and the counter will work. The `counter.txt` file will be located in the home directory: `~/counter.txt`.

## Full example

![](examples/counter-text-file-in-homedir/src/main.rs)

