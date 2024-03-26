---
title: Extreme Clippy (for a new crate)
timestamp: 2024-03-26T10:30:01
author: szabgab
published: true
description: Turn on every lint of Clippy and then turn off the ones you don't like.
tags:
    - Rust
todo:
    - extreme clippy for existing crates
    - CI
    - pre-commit hook
---

[Rust Clippy](https://github.com/rust-lang/rust-clippy) is the standard linter for Rust. It is a very good tool to make your code better, but it is also a very good educational tool.
In this video you'll see how to take it to the extreme. Turn on every lint and then disable the ones you don't like while learning a lot about Rust.


{% youtube id="dEkr5c5Kul8" %}

{% include file="examples/extreme-clippy/Cargo.toml" %}

{% include file="examples/extreme-clippy/src/main.rs" %}
