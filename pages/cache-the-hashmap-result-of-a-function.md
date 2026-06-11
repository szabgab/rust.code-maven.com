---
title: Cache the result of a function HashMap
timestamp: 2026-06-11T12:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - cached
    - HashMap
---

Caching the result of a function using the [cached crate](https://crates.io/crates/cached)


{% include file="examples/cache-hashmap-result-of-function/src/main.rs" %}

The function runs only once.

{% include file="examples/cache-hashmap-result-of-function/out.txt" %}



{% include file="examples/cache-hashmap-result-of-function/Cargo.toml" %}

