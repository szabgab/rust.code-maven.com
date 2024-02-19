---
title: Get random free port in Rust
timestamp: 2024-02-12T11:10:01
author: szabgab
published: true
description: When wanting to run a server for testing we will want to find a port that is not in use.
tags:
    - TcpListener
    - bind
    - local_addr
    - port
todo:
    - show example using it
---

{% include file="examples/get-random-free-port/src/main.rs" %}

This is being used in the tests of the [meet-os](https://github.com/szabgab/meetings.rs) project.
