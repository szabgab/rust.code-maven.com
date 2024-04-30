---
title: for in loop in Rust
timestamp: 2024-04-30T1:05:01
author: szabgab
published: true
description: Iterating over elements of an iterator can be done using the for-in loop, aka. iterator loop.
tags:
    - for
    - in
---

The [for](https://doc.rust-lang.org/std/keyword.for.html) loop, or for-in loop, or iterator loop is used when we want to
go over the elements of an iterator. This can be a fixed set of elements, but there can be also iterators where we don't know
the number of elements up-front, and even iteartors that, at least theoretically, could go on indefinitely.

In this example we see two simple cases.

In the first case we iterate over the elements of a range.

In the second example we iterate over the elements of a vector.


## The code

{% include file="examples/for-loop/src/main.rs" %}

## The output

```
$ cargo -q run
1
2
3
cat
dog
mouse
```


