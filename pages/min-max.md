---
title: Which value is smaller? Which is bigger? minimum and maximum in Rust
timestamp: 2023-11-22T10:30:01
description: Compare two numbers or two strings and tell us which is bigger and which is smaller.
tags:
    - min
    - max
    - cmp
---


As I was trying to figure out how to get the minimum and maximum value from a vector I bumped into
the [min](https://doc.rust-lang.org/std/cmp/fn.min.html) and [max](https://doc.rust-lang.org/std/cmp/fn.max.html)
functions that work on two values and can tell use which is the smaller and which is the bigger value.

In this example you can see them working on integers and on strings.

![](examples/min-max/src/main.rs)


```
min: 3
max: 5
min: hello
max: world
```

