---
title: Access the process starting time everywhere
timestamp: 2026-06-12T08:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - LazyLock
    - SystemTime
---

If you need to access the process starting time from several functions, then instead of passing it around or instead of using the [caching](/cache-the-result-of-a-function)
we could use the [std::sync::LazyLock](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).

{% include file="examples/fixed-start-time/src/main.rs" %}

## Output

{% include file="examples/fixed-start-time/out.txt" %}

