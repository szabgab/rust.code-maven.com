---
title: Check if key exists in HashMap - if the HashMap contains the key
timestamp: 2024-04-25T12:40:01
author: szabgab
published: true
description: The contains_key method will tell you if HashMap has a given key.
tags:
    - HashMap
    - contains_key
    - exists
---


If you'd like to know if a given key exists in a given HashMap you can use the [contains_key](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.contains_key)
method that will return a [bool](https://doc.rust-lang.org/std/primitive.bool.html), that's either `True` of `False`.

{% include file="examples/hashmap/check-if-key-exists/src/main.rs" %}


