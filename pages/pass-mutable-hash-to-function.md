---
title: Pass a mutable hash to a function in Rust
timestamp: 2024-04-01T20:40:01
author: szabgab
published: true
description:
tags:
    - HashMap
    - to_owned
    - fn
    - "&mut"
    - or_insert
---

{% include file="examples/fill-a-hash-in-a-function/src/main.rs" %}



The output

```
{
    "crab": 4,
    "camel": 3,
    "snake": 1,
}
```


See also [Pass a mutable vector to a function in Rust](/pass-mutable-vector-to-function)
