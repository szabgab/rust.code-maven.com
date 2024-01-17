---
title: "Remove whitespace: trim, strip in Rust"
timestamp: 2024-01-03T20:20:01
author: szabgab
published: true
description: Rust has functions to remove spaces, newlines, tabs from the left and the right end of a string.
tags:
    - trim
    - trim_start
    - trim_end
    - strip
    - rstrip
    - lstrip
---

How to remove the spaces and/or newline from the end of a string.

How to remove spaces, tabs from either end of a string?

Rust has the [trim_end](https://doc.rust-lang.org/std/primitive.str.html#method.trim_end) method to remove any white-space from the end of a string. (right-hand side).
The [trim_start](https://doc.rust-lang.org/std/primitive.str.html#method.trim_start) method to remove the white-spaces from the beginning (left-hand side) of the string.

There used to be a `trim_left` and `trim_right` but they were replaced by the `trim_start` and `trim_end`.

[trim](https://doc.rust-lang.org/std/primitive.str.html#method.trim)  will remove the white-spaces from both ends of the string.


These will remove spaces, tabs, new-lines from the beginnin or the end of the string.


![](examples/trim/src/main.rs)


The result:

```
$ cargo run -q
original:   '   text with space inside and outside

'
trim:       'text with space inside and outside'
trim_end    '   text with space inside and outside'
trim_start: 'text with space inside and outside

'
```

