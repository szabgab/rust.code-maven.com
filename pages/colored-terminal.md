---
title: Print colored text to the terminal
timestamp: 2024-06-28T09:30:01
author: szabgab
published: true
description: Using ANSI sequences we can print colored text to the terminal
tags:
    - colored
    - terminal
    - ANSI
---

Using the [colored](https://crates.io/crates/colored) crate it is super easy to print text on the terminal using colored text.

## Dependency

{% include file="examples/colored-terminal/Cargo.toml" %}

## Code

{% include file="examples/colored-terminal/src/main.rs" %}

## Output

![](images/colored-hello-world.png)


## Windows

In the source code of [ruff](https://docs.astral.sh/ruff/) I saw the following lines in the `main` function:

```rust
    // Enabled ANSI colors on Windows 10.
    #[cfg(windows)]
    assert!(colored::control::set_virtual_terminal(true).is_ok());
```

I only tried this crate on Ubuntu Linux.


