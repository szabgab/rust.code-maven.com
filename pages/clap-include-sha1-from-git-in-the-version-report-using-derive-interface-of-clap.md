---
title: How to include sha1 from git in the version report using derive interface of Clap?
timestamp: 2024-02-12T15:30:01
author: szabgab
published:  true
description: Detailes of the compiled binary can help a lot mapping bugs to versions.
tags:
    - Clap
    - shadow-rs
    - sha1
    - git
    - version
    - help
---

If you distribute some binary code and get an error report, it is quite useful to get as much details about the exact version of the binary as you can.
The [shadow-rs](https://crates.io/crates/shadow-rs) crate can help you collect this information.

In this example we'll see how to integrate it with the derive interface of [Clap](/clap).

## Dependencies

![](examples/clap/show-build-details/Cargo.toml)


## The build program

This will run when we build the binary.

![](examples/clap/show-build-details/build.rs)


## The full code

![](examples/clap/show-build-details/src/main.rs)


## Usage and explanation


We build the binary like this:

```
cargo build --release
```

There are two wolutions here.

The first, including the **version command** in the definition, we already saw a simplified version of this when
we wanted to [show version number of the command line application in Rust](/clap-show-version-number).

```rust
version = build::CLAP_LONG_VERSION
```

It will change the output generated when the user supplies the `--version` flag.


```
./target/release/show-build-details --version

show-build-details 0.1.0
branch:main
commit_hash:dc662fea
build_time:2024-02-12 13:51:46 +02:00
build_env:rustc 1.76.0 (07dca489a 2024-02-04),stable-x86_64-unknown-linux-gnu
```

For the second we set the **about command** where we can provide a string returned by a function call:

```rust
about = get_detailed_version()
```

The string returned by the function can include anything and that will be displayed at the top of the output shown
when the user supplies the `--help` flag.

We already saw a version of this when we were [showing the description in the help using the about command](/clap-about-the-application).

The output in our case looks like this:

```
$ ./target/release/show-build-details --help

    version: 0.1.0
    branch:  main
    sha1:    dc662fea
    rust:    rustc 1.76.0 (07dca489a 2024-02-04)

Usage: show-build-details --host <HOST>

Options:
      --host <HOST>
  -h, --help         Print help
  -V, --version      Print version
```



