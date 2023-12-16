---
title: Rust Compilation size and embedded string
timestamp: 2023-11-10T04:30:01
published: true
description: The embedded string and the size of the Rust are not linear, but we can reduce the size by more than 90%.
tags:
    - build
    - size
---

When we [embedded a CSV file in our code](/embedding-simple-csv-file) I noticed that the size of compiled executable jumped by a lot more than
the size of the embedded string.

I could not sleep well tonight, so when I woke up at 4 am I had to do a little experiment. I wanted to see how the compiled size changes as the size of the embedded string
changes.

So I created a new crate:

```
cargo new compile-size
```

The content of the crate was:

```rust
fn main() {
    println!("Hello World!");
}
```

Compiled it in release mode:

```
cargo build --release
```

and checked the size:

```
ls -l target/release/compile-size
```

I changed the embedded string, first I removed the whole "Hello World!" and left it with an empty string, then I started to add characters to it.


## 0

```rust
fn main() {
    println!("");
}
```

## 10

```rust
fn main() {
    println!("0123456789");
}
```

## Embedded string lengths vs Compile size in release mode

| String length | Compile size | Difference |
| ------------- | ------------ | ---------- |
| 0             | 4,654,808    |            |
| 1,343         | 4,654,808    |            |
| 1,344         | 4,658,904    | 4,096      |


So when Rust compiled the code with the embedded string it creates space for up to 1343 characters and it had to create a larger binary only
when the embedded string was 1,344 character long.

String length was measured by `.len()`.


## Minimizing compiled size.

There are several ways to make the compiled binary smaller. The [min-sized-rust](https://github.com/johnthagen/min-sized-rust) repository
contains explanations on the various methods. From [this post](https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge) I copied
the suggested instructions by adding the following to `Cargo.toml`.

```
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Enable link-time optimization
codegen-units = 1   # Reduce number of codegen units to increase optimizations
panic = 'abort'     # Abort on panic
strip = true        # Strip symbols from binary*
```

Then I ran the experiment again. This time I got the following numbers:

| String length | Compile size | Difference |
| ------------- | ------------ | ---------- |
| 0             | 326,072      |            |
| 3,224         | 326,072      |            |
| 3,225         | 330,168      | 4,096      |

First of all, the total size was reduced by more than 90%.

The embedded string could now grow to 3,224 characters without a change in the compiled size.

Something that I have not measured, but was obvious as I ran the experiment, the compilation took longer.

## Environment

I ran this experiment on this environment:

```
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/gabor/.rustup

stable-x86_64-unknown-linux-gnu (default)
rustc 1.73.0 (cc66ad468 2023-10-03)
```



## Conclusion

I still don't know why isn't the relationship between the embedded string and the compiled code is linear, but "wasting" up to 4,096 bytes is not a big issue.

I should probably add the optimizations to the releases of by command-line tools.

I found a page [monitoring the performance of Rust](https://perf.rust-lang.org/) versions, but as far as I can see it only (or primarily) monitors the speed performance and not the compiled size.

There is a lot more for me to [learn about build optimizations](https://github.com/johnthagen/min-sized-rust).

