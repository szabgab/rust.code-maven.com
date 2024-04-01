---
title: Pass a mutable vector to a function in Rust
timestamp: 2024-04-01T20:30:01
author: szabgab
published: true
description: We can pass a mutable vector lending it to a function as mutable to be filled there by values.
tags:
    - mut
    - Vec
    - fn
    - "&mut"
---

This is a simplified example showing how we can fill a vector inside a function.

Because we would like to change the vector we need to declare it **mutable** in the `main` function.

In the `add` function we are marking the expected parameter as `&mut` so it will only borrow the vector but in a mutable form.
Inside the function we used two `String`s to `push` onto the vector. In a real application we probably would not receive this string from the
caller, but we would maybe generate it or read it from some file.

Because of this we also need to pass it in using the `&mut` prefix.

{% include file="examples/fill-a-vector-in-a-function/src/main.rs" %}

The output will be

```
["camel", "camel:5", "snake", "snake:5", "crab", "crab:4"]
```

