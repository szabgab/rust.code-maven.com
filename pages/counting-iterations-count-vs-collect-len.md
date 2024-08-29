---
title: Counting iterations - count() vs collect().len()
timestamp: 2024-08-29T09:30:01
author: szabgab
published: false
description:
tags:
    - count
    - collect
    - len
    - next
    - drop
todo:
    - TODO
---

Given an iterator, how can we know how many elements it returns?

Unlike with vectors if we have an iterator we cannot just simpli fetch the number of iterations it has.
We actually need to iterate over all the iterations and count somehow.

Doing so will consume the iterator so if later on we do want to go over the values as well then we'll have to create a new iterator.

However there are cases when you don't care the actual content, you are only interesed in the number of iterations.
For example I was writing an [implementation of wc in Rust](/implementing-wc-in-rust) where I had to

There are several ways we can do that

In the following ex



{% include file="examples/count-vs-collect-len/src/main.rs" %}

Output

{% include file="examples/count-vs-collect-len/out.txt" %}

