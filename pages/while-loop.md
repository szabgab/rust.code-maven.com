---
title: while-loop in Rust
timestamp: 2024-04-30T09:30:01
author: szabgab
published: true
description: A loop when we don't know up-front how many iterations we are going to have.
tags:
    - while
    - rand
    - random
---

We usually use a [while](https://doc.rust-lang.org/std/keyword.while.html) loop if have a simple condition to end the loop,
but we don't know up-front how many iterations we are going to need to reach that condition.

For example we are generating random numbers between 1-10 and we would like to stop when their sum passes 50.

{% include file="examples/while-loop/src/main.rs" %}


## Dependency

We use the [rand](https://crates.io/crates/rand) crate as we have also seen in the [generate random numbers](/generate-random-numbers) example.


{% include file="examples/while-loop/Cargo.toml" %}







