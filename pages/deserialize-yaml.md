---
title: Deserialize complex YAML file in Rust
timestamp: 2023-12-18T14:30:01
author: szabgab
published: false
description:
tags:
    - Rust
---

We already saw how to [read and deserialize a simple YAML file in Rust](/read-simple-yaml) and we saw a number of other
aricles about [YAML in Rust](/yaml). This time we are going to see a rather complex YAML file.
It is copied from the [Kantoniko](https://kantoniko.com/) project which is a Ladino dictionary.

This file represents the word `kaza` with its tanslations to a number of languages.

{% include file="examples/deserialize-yaml/kaza.yaml" %}

## Dependencies in Cargo.toml

{% include file="examples/deserialize-yaml/Cargo.toml" %}



{% include file="examples/deserialize-yaml/src/main.rs" %}

