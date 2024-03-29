---
title: Press ENTER to continue
timestamp: 2024-03-29T18:27:01
author: szabgab
published: true
description: Function that will wait till the user presses ENTER.
tags:
    - ENTER
    - STDIN
---

A small function I wanted to use in one of my other examples so I can "hold" the process while switching to another terminal to look at the memory
usage via `htop`. I ended up using a crate to [show the used and free memory](/show-used-and-free-memory) from inside the example.

{% include file="examples/press-enter-to-continue/src/main.rs" %}


See also the [prompt](/prompt) as this is just a simple verion of it.
