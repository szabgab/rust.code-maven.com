---
title: "Rust Rocket: In memory hit counter using state"
timestamp: 2024-04-25T14:30:01
author: szabgab
published: true
description:
tags:
    - Rocket
    - AtomicUsize
    - Ordering
    - State
    - manage
todo:
    - sessions based?
---



Based on the [managed state](https://rocket.rs/guide/v0.5/state/#managed-state) example.


## Dependencies

{% include file="examples/rocket/in-memory-hit-counter-state/Cargo.toml" %}

## Code

{% include file="examples/rocket/in-memory-hit-counter-state/src/main.rs" %}

## Test

{% include file="examples/rocket/in-memory-hit-counter-state/src/tests.rs" %}





