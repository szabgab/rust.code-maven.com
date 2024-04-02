---
title: Rust HashMap where tuples are the keys
timestamp: 2024-04-02T11:10:01
author: szabgab
published: true
description: A HashMap can have various things as key, for example tuples.
tags:
    - HashMap
    - tuple
    - insert
---

{% include file="examples/hash-of-tuples-as-keys/src/main.rs" %}

The third enty has the sam key as the first entry and thus the value replaces the old value.

```
{
    (
        "apple",
        "red",
    ): "23",
}

{
    (
        "apple",
        "red",
    ): "23",
    (
        "banana",
        "green",
    ): "3",
}

{
    (
        "apple",
        "red",
    ): "42",
    (
        "banana",
        "green",
    ): "3",
}
```
