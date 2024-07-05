---
title: String slice in Rust
timestamp: 2024-02-20T11:20:01
author: szabgab
published: true
description: How to get part of a string
tags:
    - str
    - String
    - to_owned
todo:
    - TODO
---

When we create a variable like `let text = "Some text";` this strings is included in the binary compiled by `rustc`. That means in the first example the variable `text`
is itself a slice in the whole binary. We can still refer to parts of it (slices of it). We had to make the variable mutable so we can change it.

In the second example we use the `String::from` function to create a string and allocate memory for it on the heap. That means we can actully change the string
(by pushing some data on it) but what we see here is that we can access parts of it by referring to thos parts as references.

{% include file="examples/string-slice/src/main.rs" %}

{% include file="examples/string-slice/out.txt" %}

