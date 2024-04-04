---
title: Count distribution of values - how many times each word appears in a list of words
timestamp: 2024-04-04T10:10:01
author: szabgab
published: true
description: Counting the number of appearances while iterating over the values.
tags:
    - HashMap
    - entry
    - or_insert
    - test
todo:
    - is there a nicer way with partitioning?
---

Given a list of values we would like to know how many times each value appears. The source could be any, but we had them in a vector here.
The valus could be also anything, but in this case we have small strings.

{% include file="examples/count-distribution-of-values/src/main.rs" %}


