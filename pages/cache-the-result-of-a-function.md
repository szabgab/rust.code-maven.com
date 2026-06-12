---
title: Cache the result of a function
timestamp: 2026-06-11T11:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - cache
    - time
---


I have some code where I wanted to cache the result of a function. I found the [cached crate](https://crates.io/crates/cached) to be useful.

Here is the first experiment with it showing how it caches the value in one function, but not in the other.
The first 2 tv_nsec values differ, as time have elapsed between the calls.
The 3rd and 4th tv_nsec values are the same as the value got cached after the first call.

{% include file="examples/cache-result-of-function/src/main.rs" %}

{% include file="examples/cache-result-of-function/out.txt" %}

{% include file="examples/cache-result-of-function/Cargo.toml" %}


If you only want to store a timestamp then there are better solutions as well. For example check out [the starting time of the process](/fixed-start-time).

