---
title: Given a key, get value from a HashMap
timestamp: 2024-04-25T14:20:01
author: szabgab
published: true
description: There are two ways to get the value of a key in a HashMap. One requires us to deal with an Option, the other might panic if the key is missing.
tags:
    - HashMap
    - get
    - Option
    - match
---

There are two ways to get the value from a HashMap in Rust if we know the key. One of them is to access in with square brackets and the other one is to use the
[get](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.get) method.


## Access value using square brackets

This is how we access the value of the key "snake". This works.

```rust
let snake = &animal["snake"];
```

If we try to do the same with a key that does not exist we will get a run-time `panic!`: `no entry found for key`.

```rust
let turtle = &animal["turtle"];
```

So we commented out that part of the example.

We could protect our code from panic by first checking if [the HashMap contains the key](/check-of-key-exists-in-hashmap) and
only access it if the key exists as we did in that other example.


## Access value using the get method

```
let value = animal.get(name);
```

This will return an [Option](https://doc.rust-lang.org/std/option/enum.Option.html) that will either contain `Some(value)` or it will contain `None`.
There are plenty of ways to extract the value from such Option, in this example we use a [match](https://doc.rust-lang.org/std/keyword.match.html).

## What if the value itself can be None?

In other programming languages (or at least in Perl and Python) there are similar ways to get the value of a hash or a dictionary respectively. They will
return `undef` or `None` both when the key is missing and when the key is there but the value itself is `undef` or `None`. In Rust we don't have that issue.
If the values of a HashMap can contain `None` then all the other values must be in `Some()` so when call `get` we will get back an `Option<Option<value>>`
or an `Option<None>`. So we just need to peal of another layer of Option.

This is what I was trying to show in the `value_is_option` function at the end of the example.


## The full code example

{% include file="examples/hashmap/get-value-from-hashmap/src/main.rs" %}


