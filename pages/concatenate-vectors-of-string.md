---
title: Concatenate vectors of strings (merge vectors, join vectors)
timestamp: 2024-03-24T22:30:01
author: szabgab
published: true
description: I searched for joining vectors togehter, but apparently the function I needed is called concat.
tags:
    - concat
    - vec!
    - clone
todo:
    - Unclear why the string seem to move on merge
---

{% include file="examples/concatenate-vectors-of-string/src/main.rs" %}

In the first example (the concat function) we simply join the vectors together using the [concat](https://doc.rust-lang.org/std/primitive.slice.html#method.concat) method.
In this example we don't try to use the original vectors after the concatenation.


In the second example (the concat_and_use function) we want to use one of the original vectors after we concatenated them. This creates a compilation errro:

```
borrow of moved value: `birds`
```

and the compler suggests the use of the `clone` to copy the content of the vector. This solves the problem.




