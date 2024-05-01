---
title: Create empty HashSet, insert elements
timestamp: 2024-05-01T10:35:01
author: szabgab
published: true
description: We can use sets in Rust.
tags:
    - HashSet
    - new
    - insert
---


Rust provides the [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html) struct to hold a set of values.
Behind the scenes it is implemented as a [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

We can create a new HashSet using the `new` function. We can declare the type of the values, but in most cases Rust will
be able to reason about the value type based on what we inster into it.

If we would like to change the set (e.g. insert elements, remove elements) then we have to defined the variable with the
`mut` keyword to be mutable.

In this example we start with an empty set and then isert values.

As you can observe, inserting the same value more than once does not make a difference on a set. That's because sets only care
about values being in or not being in the set.


{% include file="examples/hashset/create-empty-insert/src/main.rs" %}


```
{}
{"chair"}
{"chair", "table"}
{"chair", "table"}
```
