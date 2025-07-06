---
title: GitHub Actions for Rust projects
timestamp: 2025-07-06T12:30:01
author: szabgab
published: true
show_related: true
description:
---

Using `ubuntu-latests`, `macos-latest`, or `windows-latests` will automatically bring you the most recent stable version of the Rust compiler. [see](/default-github-workflow-for-rust-on-linux).

This is nice as you won't need to worry about updating it, but it has the obvious disadvantage that the rust compiler automatically changes without you making any change in the configuration.


Many projects use the [actions-rs/toolchain](https://github.com/actions-rs/toolchain), but it has been deprecated on October 13, 2023.

Many use [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) to set the toolchain(s) explicitely. You can either set a specific version number or you can use the words:
`stable` for the most recent stable version, `beta` for the next version, `nightly` for wilde development efforts.

{% include file="examples/github-workflows/matrix.yml" %}


## GitHub Actions

* [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) to install the right toolchain(s),  target(s), and component(s).
* [bnjbvr/cargo-machete](https://github.com/bnjbvr/cargo-machete) to remove unused Rust dependencies


## Cargo commands

Various cargo commands I saw in the GitHub Workflows

```
cargo build --verbose
```

```
cargo test --verbose
```


```
cargo fmt
cargo fmt -- --check
cargo fmt -- --all --check
```


```
cargo clippy
cargo clippy -- --deny clippy::pedantic
```


```
cargo doc --no-deps --all-features
```

