---
title: Create empty HashMap in Rust without type definition
timestamp: 2024-04-25T10:20:01
author: szabgab
published: true
description: We don't always have to define the type of the keys and the values of the HashMap, we can let Rust figure it out.
tags:
    - HashMap
    - new
    - insert
---

In the other example we [defined a HashMap and declared the type of the keys and the values](/create-empty-hashmap-and-insert-pairs) then we used the [HashMap::new](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.new) function to actually create and initialize the HashMap. While people coming from some languages might think this is normal, other might dislike the duplicate mention of HashMap. Indeed in many cases we don't need to declare the type of the keys and those of the values. Rust can figure out the type by the first `insert` to the HashMap.


In this example we initialized the HashMap using the `HashMap::new()` call, but did not tell it the types. Then when the the Rust compiler encountered the first `insert` statement it could deduct the type of the keys and the values.

If you use an editor that provides inline type-hints (e.g. VS Code with the Rust-analyzer plugin) then you'd also see that Rust deducted that the invisible type declaration is `HashMap<String, i32>`.

You might notice that it is not exactly the same as we had in the [other example](/create-empty-hashmap-and-insert-pairs) where we explicitly define it as `HashMap<String, u32>`.
That's because the default integer type of Rust is `u23`.

This, of course, might cause problems if additional data comes from some external source and the values don't fit in `i32` only in a `u32` and that's why sometimes
you need to manually declare the type.

## The code

{% include file="examples/hashmap/create-empty-hashmap-no-types/src/main.rs" %}

## The output

```
0
{}

2
{
    "snake": 320,
    "snail": 4,
}
```

