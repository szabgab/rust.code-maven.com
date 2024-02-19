---
title: "Slurp: read content of a text file into a string"
timestamp: 2023-11-22T10:30:01
author: szabgab
published: true
description: Read the content of a file into memory in one short function call.
tags:
    - slurp
    - read_to_string
---

People who programmed in Perl are probably familiar with what was called **slurp mode** that was later converted to a function called **slurp**.
The whole idea was to make it easy to read the content of a text file into memory, to slurp it in.

In Rust there is a function called [read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) that does this.

{% include file="examples/slurp/src/main.rs" %}


It is nice and easy to use, you just have to remember that this will read the whole file into memory which is not going to work well
for huge files.
