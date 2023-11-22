---
title: How to get the smallest and biggest values from a vector in Rust?
timestamp: 2023-11-22T14:10:01
description: Every iterator has a min and a max method that return None if the vector was empty.
tags:
    - min
    - max
    - len
    - None
    - Option
---


Iterators have [min](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min) and [max](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max)
methods that allow us to fetch the minimum and maximum values from any data that can be converted into an iterator. For example a vector.

At compile time we usually cannot know if the vector, in which we are looking for the minimum (or maximum) value has any elements.
If it is empty then calling `min` or `max` will return `None` wrapped in an `Option`. We can either check the return value using
`match` or we can prevent the call by checking the `len` of the vector.

![](examples/min-max-vector/src/main.rs)


```
numbers: [23, 10, 78, 30]
min: 10
max: 78

min: 10
empty
min: 23
```
