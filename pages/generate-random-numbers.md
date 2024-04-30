---
title: Generate random numbers in Rust using the rand crate
timestamp: 2024-04-30T08:10:01
author: szabgab
published: true
description: Generate random numbers in Rust using the rand crate
tags:
    - rand
    - random
    - thread_rng
    - get_range
todo:
    - is this pseudorandom or real random?
    - seed
---

<!--
When facing a bug that does not always occure or a flaky we might feel that computer have their own will, but ususally we are pretty sure that they are deterministric.
So generating a true random number is not an easy task. That's why a long time ago the ancients invented the [Pseudorandom number generator](https://en.wikipedia.org/wiki/Pseudorandom_number_generator)
a mathematical function providing a series of numbers that have uniform distribution, or at least something that cannot be told apart from that.

While this is not really random, in many use-cases it is good enough.

-->

The [rand](https://crates.io/crates/rand) crate provides us plenty of ways to generate random numbers. Here are a few examples:


# Generate a random integer that fits in an i8

```rust
let random_i8: i8 = rand::random();
```

## Generate random floating point that fits in an f32


```rust
let random_f32: f32 = rand::random();
```

## Generate a random boolean

We can even ask it to generate a `true` or `false` randomly.

```rust
let random_bool: bool = rand::random();
```

## Generate a random whole number in a range

For this we need to load the [rand::Rng](https://docs.rs/rand/0.8.5/rand/trait.Rng.html)

```rust
use rand::Rng;

let random_number = rand::thread_rng().gen_range(1..=100);
```

## Dependency

{% include file="examples/generate-random-numbers/Cargo.toml" %}


## Code


{% include file="examples/generate-random-numbers/src/main.rs" %}


