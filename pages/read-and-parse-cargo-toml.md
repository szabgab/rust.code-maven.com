---
title: Read and parse Cargo.toml using the toml crate defining our own structs
timestamp: 2024-04-11T21:30:01
author: szabgab
published: true
description: The toml crate is a generic toml parser that can return structs
tags:
    - toml
    - Cargo.toml
    - serde
todo:
    - add explanation
    - full parser
---


## Cargo.toml

This is the real `Cargo.toml` of this project that we are going to parse

{% include file="examples/read-and-parse-cargo-toml/Cargo.toml" %}

## Code

This is the code including the definition of the structs.

{% include file="examples/read-and-parse-cargo-toml/src/main.rs" %}


