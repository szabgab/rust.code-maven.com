---
title: Rocket - Early return from routes
timestamp: 2024-02-01T15:00:01
author: szabgab
published: true
description:
tags:
    - Rocket
    - web
    - match
    - map_err
todo:
    - write text
---

Another [Rocket](/rocket) example.

3 ways to handle error handling. I am not sure any of these would be a recommended way, but they are good for experimentation.


{% include file="examples/rocket/early-return/Cargo.toml" %}

{% include file="examples/rocket/early-return/src/main.rs" %}

{% include file="examples/rocket/early-return/src/tests.rs" %}

{% include file="examples/rocket/early-return/templates/index.html.tera" %}

{% include file="examples/rocket/early-return/templates/message.html.tera" %}

{% include file="examples/rocket/early-return/templates/number.html.tera" %}

