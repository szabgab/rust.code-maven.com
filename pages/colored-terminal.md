---
title: Print colored text to the terminal
timestamp: 2024-06-28T09:30:01
author: szabgab
published: show
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
