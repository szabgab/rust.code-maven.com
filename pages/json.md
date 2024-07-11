---
title: JSON and Rust
timestamp: 2023-12-28T08:30:01
author: szabgab
published: true
description: JSON is used everywhere so being able to serialized and deserialize JSON is very important in Rustlang as well.
tags:
    - JSON
---

[JSON](https://www.json.org/) is used everywhere so being able to serialized and deserialize JSON is very important in Rust as well.
The two tools used for this are [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json). In this
series of articles we'll see how to use them to work with JSON in Rust.

* [Read arbitrary JSON without much preparation](/read-arbitrary-json) - `serde_json::Value`, `serde_json::from_reader`.
* [Read simple JSON and deserialize into a struct](/read-simple-json) - `serde::Deserialize`, `serde_json::from_reader`.
* [Serialize and deserialize HashMap to JSON in Rust](/serialize-hash-to-json) - `to_string`, `from_str`, `serde_json`.


* [Reading JSON files in Rust](/reading-json-files-in-rust) - video
