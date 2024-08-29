---
title: "Command line tools: Implementing wc in Rust"
timestamp: 2024-08-28T12:00:01
author: szabgab
published: true
description:
tags:
    - wc
---

{% youtube id="eQuSA9kSLAY" %}

* [source code](https://github.com/szabgab/wc.rs)
* [pre-commit](https://pre-commit.com/)
* [man wc](https://duckduckgo.com/?q=man+wc)


## Plan for improvements in the next session

* Instead of `text.split('\n').collect::<Vec<_>>().len();` write `text.split('\n').count();`
* Instead of `for filename in args[1..].iter() {` write `for filename in std::env::args().skip(1)`
* Setup CI
* Test running the command line
* return struct instead tuple of 3 usize values
* Command line flags

