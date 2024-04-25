---
title: Create empty HashMap and insert key-value pairs
timestamp: 2024-04-25T09:30:01
author: szabgab
published: true
description: Create a HashMap and make it mutable so we can insert data.
tags:
    - HashMap
    - mut
    - new
    - insert
    - ":#?"
---

[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) in Rust is a data-structure holding key-value pairs.

## Declare and initialize empty HashMap

We can declare a variable, define the type of it and initialize it using the following command:

```rust
let length_of: HashMap<String, u32> = HashMap::new();
```

This means that the keys of this HashMap will be of type `String` and the values will be of type `u32`.

The keys can be other things as well, for example we can have a [HashMap with tuples as keys](/hash-where-tuples-are-the-keys).

You might be also wondering why do you need to mention `HashMap` twice in that line. You don't. You can also rely on [rust figuring out the type of the HashMap](/create-empty-hashmap-without-type-definition).


## Declare and initialize an empty but mutable HashMap

However the above declaration is rather useless as we cannot change the content of that variable.

We need to declare it to be **mutable** using the `mut` keyword:

```rust
let mut length_of: HashMap<String, u32> = HashMap::new();
```

## Insert key-value pairs in the HashMap

We can use the [insert](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert) method to, well, insert
key-value pairs:

```rust
length_of.insert(String::from("snake"), 320);
```

## The size of the HashMap - the number of key-value pairs

The [len](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.len) method returns the **length** of the HashMap
which is the number of pairs in the HashMap.

The `:#?` formatting option helps print the HashMap in a more human-readable format.

## The full code

{% include file="examples/hashmap/create-empty-hashmap/src/main.rs" %}

```
0
{}

1
{
    "snake": 320,
}

2
{
    "snail": 4,
    "snake": 320,
}
```

