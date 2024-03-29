---
title: Show used and free memory of the computer using Rust
timestamp: 2024-03-29T18:40:01
author: szabgab
published: true
description: Get the total available memory, the used and the free memory.
tags:
    - refresh_all
    - total_memory
    - used_memory
    - total_swap
    - used_swap
    - separate_with_commas
---


The [sysinfo](https://crates.io/crates/sysinfo/) crate can provide a lot of information about the operating system.
In this example we'll see how to fetch and display the memory of the device. We'll also use the [thousands](https://crates.io/crates/thousands)
crate to commafy the numbers to make them more readable.

## Dependencies in the Cargo.toml file

{% include file="examples/show-used-and-free-memory/Cargo.toml" %}

## The source code

{% include file="examples/show-used-and-free-memory/src/main.rs" %}


## The output on my computer:

```
total memory:  29,166,948,352 bytes
used memory :   8,064,937,984 bytes
total swap  :   8,589,930,496 bytes
used swap   :   3,432,517,632 bytes
```

The `{:>15}` formatting will pad the field up to 15 characters and right-align the content.

The functions `total_memory`, `used_memory`, `total_swap`, `used_swap` return `u64` 64 bit unsigned integers.

The `separate_with_commas` converts the numbers into more readable strings.


