---
title: Pre-allocation of memory for a string
timestamp: 2024-06-28T14:30:01
author: szabgab
published: false
description:
tags:
    - Rust
todo:
    - TODO
---

In this example I am trying to show how pre-allocation of memory for a string can improve speed.
However in this example I can only see 3% time difference which I think is meaningless.

{% include file="examples/pre-allocation/src/main.rs" %}

{% include file="examples/pre-allocation/out.txt" %}



