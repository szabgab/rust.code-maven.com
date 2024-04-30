---
title: Iterate over both index and value of a vector (enumerate)
timestamp: 2024-04-30T15:00:01
author: szabgab
published: true
description: Iterate over both the indexes and the values of a vector
tags:
    - vec!
    - iter
    - enumerate
    - needless_range_loop
todo:
    - iterate over HashMap
---

In most cases when when want to process each value in a vector we don't car about the actual location of the value.
We just iterate over the values and do something with them. E.g. print them.  This is what we see in the first example.


There are, however cases when we also need to know the location of each value. There are two approaches we can use.

We can iterate over the numbers from 0 till the number of elements in the vector. These will be the indices. Then using the current number we can access the respective value in the vector.
This is the second example.
This code triggers a [needless_range_loop](https://rust-lang.github.io/rust-clippy/master/index.html#/needless_range_loop) violation suggesting the third solution


We can create an iterator out of the vector using [iter](https://doc.rust-lang.org/std/iter/index.html) and calling [enumerate](https://doc.rust-lang.org/std/iter/struct.Enumerate.html) on it.
This will allow us to iterate over the index-value pairs.


## Code

{% include file="examples/enumerate/src/main.rs" %}


## Output

```
cat
snake
camel

0 cat
1 snake
2 camel

0 cat
1 snake
2 camel
```


